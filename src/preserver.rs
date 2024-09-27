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
    pub fn new(file: &str) -> Self {
        Preserver {
            output_file: file.to_owned(),
            has_written: false,
            buffer: Vec::new(),
        }
    }

    pub fn read_from_file(filename: &str) -> Result<Self> {
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
    pub fn add_string(&mut self, s: &str) {
        self.buffer.push(b'\n');
        self.buffer.append(&mut s.to_owned().into_bytes());
    }
}


/**
 * Development Skeleton
 *
 * type EntryID String
 *  
 * type EntryCache struct {
 *      words_to_pages: Map<String, EntryID>
 *      pages: Map<EntryID, Entry>
 * } 
 *
 * type Version int;
 *  
 * type Entry {
 *      word: String,
 *      synonyms: Vec<String>,
 *      also_appears: Vec<String>, 
 *      principal_translations: Vec<SpEnDefinition>, 
 *      additional_translations: Vec<SpEnDefinition>, 
 *      compound_forms: Vec<Definition>
 * 
 * 
 *      data_version: Version,
 *      date_retrieved: Time,
 * }    
 *      // ? maybe consider these.
 *      alternate_forms: 
 * }
 * 
 * 
 * 
 */
pub fn dorp(){

}
