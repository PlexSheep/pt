//! # Dump data
//!
//! This crate is part of [`pt`](../libpt/index.html), but can also be used as a standalone
//! module.
//!
//! Hedu is made for hexdumping data. `libpt` offers a cli application using this module.

use crate::display::humanbytes;
use anyhow::{bail, Result};
use libpt_log::{debug, error, trace, warn};
use std::io::{prelude::*, Read, SeekFrom};

const BYTES_PER_LINE: usize = 16;
const LINE_SEP_HORIZ: char = '─';
const LINE_SEP_VERT: char = '│';
const CHAR_BORDER: &'static str = "|";

#[derive(Debug)]
pub struct HeduConfig {
    pub chars: bool,
    pub skip: usize,
    pub show_identical: bool,
    pub limit: usize,
    stop: bool,
    len: usize,
    data_idx: usize,
    rd_counter: usize,
    buf: [[u8; BYTES_PER_LINE]; 2],
    alt_buf: usize,
    display_buf: String,
}

impl HeduConfig {
    pub fn new(chars: bool, skip: usize, show_identical: bool, limit: usize) -> Self {
        HeduConfig {
            chars,
            skip,
            show_identical,
            limit,
            stop: false,
            len: usize::MIN,
            data_idx: usize::MIN,
            rd_counter: usize::MIN,
            buf: [[0; BYTES_PER_LINE]; 2],
            alt_buf: 0,
            display_buf: String::new(),
        }
    }
    #[inline]
    fn display(&mut self) {
        println!("{}", self.display_buf);
        self.display_buf = String::new();
    }
}

pub trait DataSource: Read {
    fn skip(&mut self, length: usize) -> std::io::Result<()>;
}
impl DataSource for std::io::Stdin {
    fn skip(&mut self, _length: usize) -> std::io::Result<()> {
        warn!("can't skip bytes for the stdin!");
        Ok(())
    }
}
impl DataSource for std::fs::File {
    fn skip(&mut self, length: usize) -> std::io::Result<()> {
        self.seek(SeekFrom::Current(length as i64))?;
        // returns the new position from the start, we don't need it here.
        Ok(())
    }
}

pub fn dump(data: &mut dyn DataSource, mut config: HeduConfig) -> Result<()> {
    // prepare some variables

    // skip a given number of bytes
    if config.skip > 0 {
        data.skip(config.skip)?;
        config.data_idx += config.skip;
        adjust_data_idx(&mut config);
        debug!("Skipped {}", humanbytes(config.skip));
    }

    // print the head
    config.display_buf += &format!("DATA IDX {LINE_SEP_VERT} DATA AS HEX");
    if config.chars {
        config.display_buf += &format!("{:width$} {LINE_SEP_VERT}", "", width = 37);
    }
    config.display();
    if config.chars {
        config.display_buf += &format!("{LINE_SEP_HORIZ}").repeat(80);
    } else {
        config.display_buf += &format!("{LINE_SEP_HORIZ}").repeat(59);
    }
    config.display();

    // data dump loop
    rd_data(data, &mut config)?;
    while config.len > 0 {
        config.display_buf += &format!("{:08X} {LINE_SEP_VERT} ", config.data_idx);
        for i in 0..config.len {
            if i as usize % BYTES_PER_LINE == BYTES_PER_LINE / 2 {
                config.display_buf += " ";
            }
            config.display_buf += &format!("{:02X} ", config.buf[config.alt_buf][i]);
        }
        if config.len == BYTES_PER_LINE / 2 {
            config.display_buf += " "
        }
        for i in 0..(BYTES_PER_LINE - config.len) {
            if i as usize % BYTES_PER_LINE == BYTES_PER_LINE / 2 {
                config.display_buf += " ";
            }
            config.display_buf += "   ";
        }
        if config.chars {
            config.display_buf += &format!("{LINE_SEP_VERT} |");
            for i in 0..config.len {
                config.display_buf +=
                    &format!("{}", mask_chars(config.buf[config.alt_buf][i] as char));
            }
            config.display_buf += CHAR_BORDER;
        }
        config.display();

        // loop breaker logic
        if config.stop {
            break;
        }

        // after line logic
        rd_data(data, &mut config)?;
        config.alt_buf ^= 1; // toggle the alt buf
        if config.buf[0] == config.buf[1] && config.len == BYTES_PER_LINE && !config.show_identical
        {
            trace!(
                buf = format!("{:?}", config.buf),
                "found a duplicating line"
            );
            let start_line = config.data_idx;
            while config.buf[0] == config.buf[1] && config.len == BYTES_PER_LINE {
                rd_data(data, &mut config)?;
                config.data_idx += BYTES_PER_LINE; // FIXME: incorrect logic #59
            }
            config.display_buf += &format!(
                "^^^^^^^^ {LINE_SEP_VERT} (repeats {} lines)",
                (config.data_idx - start_line) / (BYTES_PER_LINE * 2)
            );
            config.display();
        }
        // switch to the second half of the buf, the original half is stored the old buffer
        // We detect duplicate lines with this
        config.alt_buf ^= 1; // toggle the alt buf
    }
    Ok(())
}

fn mask_chars(c: char) -> char {
    if c.is_ascii_graphic() {
        return c;
    } else if c == '\n' {
        return '↩';
    } else if c == ' ' {
        return '␣';
    } else if c == '\t' {
        return '⭾';
    } else {
        return '�';
    }
}

#[inline]
fn adjust_data_idx(config: &mut HeduConfig) {
    config.data_idx += BYTES_PER_LINE - config.data_idx % BYTES_PER_LINE;
}

fn rd_data(data: &mut dyn DataSource, config: &mut HeduConfig) -> Result<()> {
    match data.read(&mut config.buf[config.alt_buf]) {
        Ok(mut len) => {
            trace!(
                conf = format!("{:?}", config),
                eval = config.limit != 0 && config.rd_counter >= config.limit,
                "reached limit?"
            );
            if config.limit != 0 && config.rd_counter + (BYTES_PER_LINE - 1) >= config.limit {
                trace!(
                    conf = format!("{:?}", config),
                    nlen = (config.limit % BYTES_PER_LINE),
                    "byte counter is farther than limit"
                );
                len = config.limit % BYTES_PER_LINE;
                config.stop = true;
            }
            config.len = len;
            config.rd_counter += config.len;
            config.data_idx += config.len;
            adjust_data_idx(config);
            return Ok(());
        }
        Err(err) => {
            error!("error while reading data: {err}");
            bail!(err)
        }
    }
}
