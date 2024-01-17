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

pub const BYTES_PER_LINE: usize = 16;
pub const LINE_SEP_HORIZ: char = '─';
pub const LINE_SEP_VERT: char = '│';
pub const CHAR_BORDER: &'static str = "|";

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
    pub display_buf: String,
    first_iter: bool,
}

impl HeduConfig {
    pub fn new(chars: bool, skip: usize, show_identical: bool, limit: usize) -> Self {
        HeduConfig {
            chars,
            skip,
            show_identical,
            limit,
            stop: false,
            len: 0,
            data_idx: 0,
            rd_counter: 0,
            buf: [[0; BYTES_PER_LINE]; 2],
            alt_buf: 0,
            display_buf: String::new(),
            first_iter: true,
        }
    }
    #[inline]
    pub fn display(&mut self) {
        println!("{}", self.display_buf);
        self.display_buf = String::new();
    }
    #[inline]
    pub fn sep(&mut self) {
        if self.chars {
            self.display_buf += &format!("{LINE_SEP_HORIZ}").repeat(80);
        } else {
            self.display_buf += &format!("{LINE_SEP_HORIZ}").repeat(59);
        }
        self.display();
    }
    #[inline]
    pub fn newline(&mut self) {
        self.display_buf += "\n";
        self.display();
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

pub fn dump(data: &mut dyn DataSource, config: &mut HeduConfig) -> Result<()> {
    // skip a given number of bytes
    if config.skip > 0 {
        data.skip(config.skip)?;
        config.data_idx += config.skip;
        adjust_data_idx(config);
        debug!("Skipped {}", humanbytes(config.skip));
    }

    // print the head
    config.display_buf += &format!("DATA IDX {LINE_SEP_VERT} DATA AS HEX");
    if config.chars {
        config.display_buf += &format!("{:width$} {LINE_SEP_VERT} DATA AS CHAR", "", width = 37);
    }
    config.display();
    config.sep();

    // data dump loop
    while config.len > 0 || config.first_iter {
        config.display_buf += &format!("{:08X} {LINE_SEP_VERT} ", config.data_idx);
        rd_data(data, config)?;
        if config.len != 0 && config.first_iter {
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
        } else {
            config.display_buf += &format!("(no data){:40}", "");
        }
        if config.chars {
            config.display_buf += &format!("{LINE_SEP_VERT} ");
            if config.len != 0 && config.first_iter {
                config.display_buf += CHAR_BORDER;
                for i in 0..config.len {
                    config.display_buf +=
                        &format!("{}", mask_chars(config.buf[config.alt_buf][i] as char));
                }
                config.display_buf += CHAR_BORDER;
            } else {
                config.display_buf += &format!("(no data)");
            }
        }
        config.display();

        // loop breaker logic
        if config.stop || config.len == 0 {
            break;
        }

        // after line logic
        config.alt_buf ^= 1; // toggle the alt buf
        if config.buf[0] == config.buf[1] && config.len == BYTES_PER_LINE && !config.show_identical
        {
            trace!(
                buf = format!("{:?}", config.buf),
                "found a duplicating line"
            );
            let start_line = config.data_idx;
            while config.buf[0] == config.buf[1] && config.len == BYTES_PER_LINE {
                rd_data(data, config)?;
            }
            config.display_buf += &format!(
                "^^^^^^^^ {LINE_SEP_VERT} (repeats {} lines){:32}",
                (config.data_idx - start_line) / (BYTES_PER_LINE * 2),
                ""
            );
            if config.chars {
                config.display_buf += &format!("{LINE_SEP_VERT}");
            }
            trace!(
                buf = format!("{:X?}", config.buf),
                "dumping buf after line skip"
            );
            config.display();
            config.first_iter = false;
        }
        // switch to the second half of the buf, the original half is stored the old buffer
        // We detect duplicate lines with this
        config.alt_buf ^= 1; // toggle the alt buf
    }
    config.data_idx += config.len;

    config.sep();
    config.display_buf += &format!(
        "{:08X} {LINE_SEP_VERT} dumped total:\t{:<8} {:<16}{:3}",
        config.rd_counter,
        humanbytes(config.rd_counter),
        format!("({} B)", config.rd_counter),
        ""
    );
    if config.chars {
        config.display_buf += &format!("{LINE_SEP_VERT}");
    }
    config.display();
    config.display_buf += &format!(
        "{:08X} {LINE_SEP_VERT} read total:\t\t{:<8} {:<16}{:3}",
        config.data_idx,
        humanbytes(config.data_idx),
        format!("({} B)", config.data_idx),
        ""
    );
    if config.chars {
        config.display_buf += &format!("{LINE_SEP_VERT}");
    }
    config.display();
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
    config.data_idx -= config.data_idx % BYTES_PER_LINE;
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
