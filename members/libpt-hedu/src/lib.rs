//! # Dump data
//!
//! This crate is part of [`pt`](../libpt/index.html), but can also be used as a standalone
//! module.
//!
//! This crate is currently empty.

use anyhow::{bail, Result};
use libpt_bintols::display::{bytes_to_bin, humanbytes};
use libpt_log::{error, info, trace, warn};
use std::io::{prelude::*, BufReader, Read, SeekFrom};

const BYTES_PER_LINE: usize = 16;
const LINE_SEP_HORIZ: char = '─';
const LINE_SEP_VERT: char = '│';

pub struct DumpConfig {
    pub chars: bool,
    pub skip: usize,
    pub show_identical: bool,
    pub len: usize,
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

pub fn dump(data: &mut dyn DataSource, config: DumpConfig) -> Result<()> {
    // skip a given number of bytes
    if config.skip > 0 {
        data.skip(config.skip)?;
        info!("Skipped {}", humanbytes(config.skip));
    }
    let mut buf: [[u8; BYTES_PER_LINE]; 2] = [[0; BYTES_PER_LINE]; 2];
    let mut alt_buf = 0usize;
    let mut line_counter: usize = 0;
    let mut len: usize;
    // print the head
    print!("LINE IDX {LINE_SEP_VERT} DATA AS HEX");
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
    len = rd_data(data, &mut buf, &mut alt_buf).unwrap();
    while len > 0 {
        print!("{:08X} {LINE_SEP_VERT} ", line_counter);
        for i in 0..len {
            if i as usize % BYTES_PER_LINE == BYTES_PER_LINE / 2 {
                print!(" ");
            }
            print!("{:02X} ", buf[alt_buf][i]);
        }
        if len == BYTES_PER_LINE / 2 {
            print!(" ")
        }
        for i in 0..(BYTES_PER_LINE - len) {
            if i as usize % BYTES_PER_LINE == BYTES_PER_LINE / 2 {
                print!(" ");
            }
            print!("   ");
        }
        if config.chars {
            print!("{LINE_SEP_VERT} |");
            for i in 0..len {
                print!("{}", mask_chars(buf[alt_buf][i] as char));
            }
            print!("|");
        }
        line_counter += 1;
        println!();
        len = rd_data(data, &mut buf, &mut alt_buf).unwrap();
        if buf[0] == buf[1] && len == BYTES_PER_LINE && !config.show_identical {
            trace!(buf = format!("{:?}", buf), "found a duplicating line");
            let start_line = line_counter;
            while buf[0] == buf[1] && len == BYTES_PER_LINE {
                len = rd_data(data, &mut buf, &mut alt_buf).unwrap();
                line_counter += 1;
            }
            println!(
                "^^^^^^^^ {LINE_SEP_VERT} (repeats {} lines)",
                line_counter - start_line
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
) -> Result<usize> {
    match data.read(&mut buf[*alt_buf]) {
        Ok(len) => {
            return Ok(len);
        }
        Err(err) => {
            error!("error while reading data: {err}");
            bail!(err)
        }
    }
}
