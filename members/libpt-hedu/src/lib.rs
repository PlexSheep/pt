//! # Dump data
//!
//! This crate is part of [`pt`](../libpt/index.html), but can also be used as a standalone
//! module.
//!
//! This crate is currently empty.

use anyhow::Result;
use std::{
    fmt::Debug,
    io::{prelude::*, BufRead, BufReader, Bytes},
};

pub fn dump<T>(mut data: BufReader<T>) -> Result<()>
where
    T: Read,
{
    for (i, b) in data.bytes().enumerate() {
        if i % 8 == 0 {
            print!("{:08X}\t", i);
        }
        print!("{:02X?} ", b.unwrap());
        if i % 8 == 7 {
            println!();
        }
    }
    Ok(())
}
