use std::fs::{self, OpenOptions};

use chrono::Local;
use std::io::{self, Result, Write};

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

    pub fn hello() -> String {
        "hello world".to_string()
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
    pub fn write(self) -> io::Result<usize> {
        let mut f = OpenOptions::new().append(true).open(self.output_file)?;
        f.write(&self.buffer)
    }

    // pub fn save(self: Self, word: &String) {
    //     let mut file = OpenOptions::new().append(true).open(self.output_file);
    //     if self.has_written {
    //         // let date_string = Local::now().format()
    //         let date_string = Local::now().format("%Y-%m-%d. %H:%M:%S");
    //         writeln!(file, "\n{}\n{}", date_string, "--------------------",)
    //     }
    // }
}

// fn save(word: &String) -> anyhow::Result<()> {
//     // let mut file = OpenOptions::new().append(true).open(OUTPUT_FILE)?;
//     unsafe {
//         if !HAS_WRITTEN {
//             let date = Local::now();
//             writeln!(
//                 file,
//                 "\n{}\n{}",
//                 date.format("%Y-%m-%d. %H:%M:%S"),
//                 "--------------------"
//             );
//             HAS_WRITTEN = true;
//         }
//     }
//     writeln!(file, "{}", word)?;
//     Ok(())
// }
