use std::fs::{self, OpenOptions};

use std::io::{self, Result, Write};

// A struct that allows one to preserve data in a file.
#[derive(Debug, Clone)]
pub struct Preserver {
    output_file: String,
    has_written: bool,
    buffer: Vec<u8>,
}

impl Preserver {
    pub fn new(file: &String) -> Self {
        Preserver {
            output_file: file.to_owned(),
            has_written: false,
            buffer: Vec::new(),
        }
    }

    pub fn read_from_file(filename: &String) -> Result<Self> {
        let mut p = Self::new(filename);
        p.read_buffer()?;
        Ok(p)
    }

    // read_buffer reads the contents of the Preserver's file into the
    // preserver buffer.
    fn read_buffer(&mut self) -> io::Result<usize> {
        let contents = fs::read_to_string(&self.output_file)?;
        self.buffer = contents.clone().into_bytes();
        Ok(contents.len())
    }

    // write saves the contents of the preserver to the output file.
    pub fn write(&self) -> io::Result<usize> {
        let mut f = OpenOptions::new().append(true).open(&self.output_file)?;
        f.write(&self.buffer)
    }

    // add_string takes a string and appends it to the Preserver's buffer
    // on a new line.
    pub fn add_string(&mut self, s: &String) {
        self.buffer.push(b'\n');
        self.buffer.append(&mut s.to_owned().into_bytes());
    }
}
