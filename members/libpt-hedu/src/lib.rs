//! # Dump data
//!
//! This crate is part of [`pt`](../libpt/index.html), but can also be used as a standalone
//! module.
//!
//! This crate is currently empty.

use anyhow::{bail, Result};
use std::io::{prelude::*, BufReader};
use libpt_log::error;

const BYTES_PER_LINE: usize = 16;
const LINE_SEP_HORIZ: char = '─';
const LINE_SEP_VERT: char = '│';

pub fn dump<T>(mut data: BufReader<T>, chars: bool) -> Result<()>
where
    T: Read,
{
    let mut buf: [u8; BYTES_PER_LINE] = [0; BYTES_PER_LINE];
    let mut line_counter: usize = 0;
    let mut len: usize;
    // print the head
    print!("LINE IDX {LINE_SEP_VERT} DATA AS HEX");
    if chars {
        print!("{:width$} {LINE_SEP_VERT} FOO", "", width = 37);
    }
    println!();
    if chars {
        println!("{}", format!("{LINE_SEP_HORIZ}").repeat(78));
    } else {
        println!("{}", format!("{LINE_SEP_HORIZ}").repeat(59));
    }
    // data dump loop
    len = rd_data(&mut data, &mut buf).unwrap();
    while len > 0 {
        print!("{:08X} {LINE_SEP_VERT} ", line_counter);
        for i in 0..len {
            if i as usize % BYTES_PER_LINE == BYTES_PER_LINE / 2 {
                print!(" ");
            }
            print!("{:02X} ", buf[i]);
        }
        for i in 0..(BYTES_PER_LINE - len) {
            if i as usize % BYTES_PER_LINE == BYTES_PER_LINE / 2 {
                print!(" ");
            }
            print!("   ");
        }
        if chars {
            print!("{LINE_SEP_VERT} ");
            for i in 0..len {
                print!("{}", mask_chars(buf[i] as char));
            }
        }
        line_counter += 1;
        println!();
        len = rd_data(&mut data, &mut buf).unwrap();
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

fn rd_data<T>(data: &mut BufReader<T>, mut buf: &mut [u8]) -> Result<usize>
where
    T: Read,
{
    match data.read(&mut buf) {
        Ok(len) => Ok(len),
        Err(err) => {
            error!("error while reading data: {err}");
            bail!(err)
        }
    }
}
