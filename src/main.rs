use std::fs::OpenOptions;
use std::io::prelude::*;

use std::io;

use chrono::Local;

use headless_chrome::types::Bounds;
use headless_chrome::{Browser, LaunchOptions};

mod preserver;
use preserver::Preserver;

mod word;
use word::FlashCard;

const WORD_REFERENCE_SP_EN_QUERY: &str =
    "https://www.wordreference.com/es/en/translation.asp?spen=";
const WORD_REFERENCE_DE_EN_QUERY: &str = "https://www.wordreference.com/deen/";

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

fn main() -> anyhow::Result<()> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .window_size(Some((1024, 1280)))
            .build()
            .expect("Could not find chrome-executable"),
    )?;

    let tab = browser.new_tab()?;
    tab.set_bounds(Bounds::Normal {
        left: Some(0),
        top: Some(0),
        width: Some(1024.0),
        height: Some(1280.0),
    })?;

    let mut last_word = String::new();

    let mut p = Preserver::read_from_file(&OUTPUT_FILE.to_string())?;

    loop {
        io::stdout().flush()?;

        let word = input();

        match &word {
            Err(_e) => {
                println!("Problem with input.");
                continue;
            }
            Ok(_e) => (),
        }

        let word = word.unwrap();

        if word == "\n" {
            if last_word.is_empty() || last_word == "\n" {
                println!("no previous word to save");
                continue;
            }

            let trimmed = last_word.trim().to_string();

            p.add_string(&trimmed);
            match p.write() {
                Err(e) => println!("Error writing {} to {}: {}", trimmed, OUTPUT_FILE, e),
                Ok(_e) => println!("{} added to {}", trimmed, OUTPUT_FILE),
            }

            last_word.clear();

            continue;
        }

        tab.navigate_to(&format!("{}{}", WORD_REFERENCE_DE_EN_QUERY, word))?;

        last_word = word;
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
            writeln!(
                file,
                "\n{}\n{}",
                date.format("%Y-%m-%d. %H:%M:%S"),
                "--------------------"
            );
            HAS_WRITTEN = true;
        }
    }
    writeln!(file, "{}", word)?;
    Ok(())
}
