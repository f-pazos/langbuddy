use std::{
    error,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Lines, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;

pub struct Preserver {
    directory: PathBuf,
    initialized: bool,
    lines: Vec<String>,
}

impl Preserver {
    const FILE: &str = "main.prs";
    const TEMP_FILE: &str = "main.swp";

    pub fn new(directory: &Path) -> anyhow::Result<Self> {
        let mut errors = vec![];
        if !directory.is_absolute() {
            errors.push("path mut be absolute");
        }
        if !directory.exists() {
            errors.push("path must exist");
        }
        if !directory.is_dir() {
            errors.push("path must be directory");
        }
        if !directory.is_absolute() {
            errors.push("path mut be absolute");
        }
        if !errors.is_empty() {
            return Err(anyhow!(errors.join(" ")));
        }

        Ok(Preserver {
            directory: directory.into(),
            initialized: false,
            lines: Vec::new(),
        })
    }

    pub fn read(&mut self) -> anyhow::Result<()> {
        // let mut p = Self::new(filename);
        let f = fs::File::open(self.directory.join(Self::FILE))?;
        let reader = BufReader::new(f);
        self.lines = reader.lines().map(|l| l.unwrap()).collect();
        Ok(())
    }

    /**
     * write saves the contents of the preserver to the output file.
     */
    pub fn write(&self) -> anyhow::Result<()> {
        let tmp_path = self.directory.join(Self::TEMP_FILE);
        let mut tmp_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&tmp_path)?;

        for line in &self.lines {
            writeln!(tmp_file, "{line}")?;
        }
        println!("{:?}", tmp_path);

        fs::rename(tmp_path, self.directory.join(Self::FILE))?;
        Ok(())
    }

    /**
     * add_string takes a string and appends it to the Preserver's buffer
     * on a new line.
     */
    pub fn add_line(&mut self, s: &str) {
        // self.buffer.push(b'\n');
        self.lines.push(s.to_owned());
    }
}
