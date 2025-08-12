use core::panic;
use std::{
    fs,
    io::{self, BufWriter, Read, Write},
};

use crate::arch::MEM_SIZE;

#[allow(dead_code)] // Rust doesn't allow Debug anymore apparently? If only purpose of Debug is logging, prohibited
#[derive(Debug)]
pub struct AsmOutput<'a>(Vec<&'a str>);

impl<'a> From<Vec<&'a str>> for AsmOutput<'a> {
    fn from(value: Vec<&'a str>) -> Self {
        AsmOutput(value)
    }
}

impl<'a> Read for AsmOutput<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}

impl AsmOutput<'_> {
    pub fn to_file(&self, file: &str) -> io::Result<()> {
        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file)?;

        let mut f = BufWriter::new(&file);

        let mut size = MEM_SIZE as u16 * 2; // architectural limitation - fixed to 8 bit address lines

        for w in &self.0 {
            if size == 0 {
                panic!("mem too small!");
            }

            // ugh retro idiosyncracies
            for b in w.as_bytes().iter().rev() {
                f.write(&[*b])?;
            }
            // f.write(w.as_bytes())?;

            size -= 1;
        }
        for _ in 0..size {
            f.write("--".as_bytes())?;
        }
        f.flush()?;
        Ok(())
    }
}
