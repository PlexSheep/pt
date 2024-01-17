//! # Dump data
//!
//! This crate is part of [`pt`](../libpt/index.html), but can also be used as a standalone
//! module.
//!
//! This crate is currently empty.

use crate::display::humanbytes;
use anyhow::{bail, Result};
use libpt_log::{debug, error, trace, warn};
use std::io::{prelude::*, Read, SeekFrom};

const BYTES_PER_LINE: usize = 16;
const LINE_SEP_HORIZ: char = '─';
const LINE_SEP_VERT: char = '│';

pub struct HeduConfig {
    pub chars: bool,
    pub skip: usize,
    pub show_identical: bool,
    pub limit: usize,
    stop: bool,
    len: usize,
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
        }
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
    let mut buf: [[u8; BYTES_PER_LINE]; 2] = [[0; BYTES_PER_LINE]; 2];
    let mut alt_buf = 0usize;
    let mut byte_counter: usize = 0;

    // skip a given number of bytes
    if config.skip > 0 {
        data.skip(config.skip)?;
        byte_counter += config.skip;
        debug!("Skipped {}", humanbytes(config.skip));
    }

    // print the head
    print!("DATA IDX {LINE_SEP_VERT} DATA AS HEX");
    if config.chars {
        print!("{:width$} {LINE_SEP_VERT} FOO", "", width = 37);
    }
    println!();
    if config.chars {
        println!("{}", format!("{LINE_SEP_HORIZ}").repeat(80));
    } else {
        println!("{}", format!("{LINE_SEP_HORIZ}").repeat(59));
    }

    // data dump loop
    rd_data(data, &mut buf, &mut alt_buf, &mut byte_counter, &mut config)?;
    while config.len > 0 {
        print!("{:08X} {LINE_SEP_VERT} ", byte_counter);
        for i in 0..config.len {
            if i as usize % BYTES_PER_LINE == BYTES_PER_LINE / 2 {
                print!(" ");
            }
            print!("{:02X} ", buf[alt_buf][i]);
        }
        if config.len == BYTES_PER_LINE / 2 {
            print!(" ")
        }
        for i in 0..(BYTES_PER_LINE - config.len) {
            if i as usize % BYTES_PER_LINE == BYTES_PER_LINE / 2 {
                print!(" ");
            }
            print!("   ");
        }
        if config.chars {
            print!("{LINE_SEP_VERT} |");
            for i in 0..config.len {
                print!("{}", mask_chars(buf[alt_buf][i] as char));
            }
            print!("|");
        }
        println!();

        // loop breaker logic
        if config.stop {
            break;
        }

        // after line logic
        rd_data(data, &mut buf, &mut alt_buf, &mut byte_counter, &mut config)?;
        alt_buf ^= 1; // toggle the alt buf
        if buf[0] == buf[1] && config.len == BYTES_PER_LINE && !config.show_identical {
            trace!(buf = format!("{:?}", buf), "found a duplicating line");
            let start_line = byte_counter;
            while buf[0] == buf[1] && config.len == BYTES_PER_LINE {
                rd_data(data, &mut buf, &mut alt_buf, &mut byte_counter, &mut config)?;
                byte_counter += BYTES_PER_LINE;
            }
            println!(
                "^^^^^^^^ {LINE_SEP_VERT} (repeats {} lines)",
                byte_counter - start_line
            );
        }
        // switch to the second half of the buf, the original half is stored the old buffer
        // We detect duplicate lines with this
        alt_buf ^= 1; // toggle the alt buf
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

fn rd_data(
    data: &mut dyn DataSource,
    buf: &mut [[u8; BYTES_PER_LINE]; 2],
    alt_buf: &mut usize,
    byte_counter: &mut usize,
    config: &mut HeduConfig,
) -> Result<()> {
    *byte_counter += config.len;
    match data.read(&mut buf[*alt_buf]) {
        Ok(mut len) => {
            if config.limit != 0 && *byte_counter >= config.limit {
                trace!(
                    byte_counter,
                    limit = config.limit,
                    len,
                    nlen = (config.limit % BYTES_PER_LINE),
                    "byte counter is farther than limit"
                );
                len = config.limit % BYTES_PER_LINE;
                config.stop = true;
            }
            config.len = len;
            return Ok(());
        }
        Err(err) => {
            error!("error while reading data: {err}");
            bail!(err)
        }
    }
}
