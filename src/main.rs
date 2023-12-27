use std::error::Error;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::{io, fs};
use std::path::PathBuf;
use std::io::prelude::*;
use std::time::{SystemTime, self};

use chrono::Local;
use headless_chrome::types::Bounds;
use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::protocol::cdp::Page;





const WORD_REFERENCE_SP_EN_QUERY: &str = "https://www.wordreference.com/es/en/translation.asp?spen=";
const OUTPUT_FILE: &str = "words.txt";

static mut HAS_WRITTEN: bool = false;

struct WordEntry {
    word: String, 
    definition: String,
    url: String,
}

struct WordEntries {
    entries: Vec<WordEntry>,
}




fn main() -> anyhow::Result<()>{
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .window_size(Some((1024, 1280)))
            .build()
            .expect("Could not find chrome-executable"),
    )?;

    let tab = browser.new_tab()?;
    tab.set_bounds(Bounds::Normal{left:Some(0), top: Some(0), width: Some(1024.0), height: Some(1280.0)})?;


    let mut last_word = String::new();

    loop { 
        print!(">> ");
        io::stdout().flush()?;

        let word = input();

        match &word {
            Err(e) => {
                println!("Problem with input.");
                continue
            }
            Ok(e) => (),
        }

        let word = word.unwrap();

        if word == "\n" {
            if last_word.is_empty() || last_word == "\n" {
                println!("no previous word to save");
                continue
            }

            let trimmed = last_word.trim().to_string();

            match save(&trimmed) {
                Err(e) => println!("Error writing {} to {}: {}", trimmed, OUTPUT_FILE, e),
                Ok(e) => println!("{} added to {}", trimmed, OUTPUT_FILE),
            }

            last_word.clear();

            continue
        }

        tab.navigate_to(&format!("{}{}", WORD_REFERENCE_SP_EN_QUERY, word))?;

        last_word = word;

        // tab.wait_until_navigated()?;
        // let html = tab.get_content()?;
        // std::fs::write("out.html", html)?;

    }
}


fn input() -> anyhow::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    return Ok(s);
}

fn save(word: &String) -> anyhow::Result<()> {
    let mut file = OpenOptions::new().append(true).open(OUTPUT_FILE)?; 
    unsafe {
        if !HAS_WRITTEN {
            let date = Local::now();
            writeln!(file, "\n{}\n{}",date.format("%Y-%m-%d. %H:%M:%S") , "--------------------");
            HAS_WRITTEN = true;
        }
    }
    writeln!(file, "{}", word)?;
    Ok(())
}







