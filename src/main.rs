use std::error::Error;
use std::ffi::OsStr;
use std::{io, fs};
use std::path::PathBuf;
use std::time::{SystemTime, self};

use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::protocol::cdp::Page;
use html_parser::Dom;


const WORD_REFERENCE_SP_EN_QUERY: &str = "https://www.wordreference.com/es/en/translation.asp?spen=";

struct WordEntry {
    word: String, 
    definition: String,
    url: String,
}

struct WordEntries {
    entries: Vec<WordEntry>,
}




fn main() -> anyhow::Result<()>{

    let last_timestamp = SystemTime::now();

    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .build()
            .expect("Could not find chrome-executable"),
    )?;

    let tab = browser.new_tab()?;


    loop { 
        println!("input a word to query");
        let word = input();

        tab.navigate_to(&format!("{}{}", WORD_REFERENCE_SP_EN_QUERY, word))?;
        tab.wait_until_navigated()?;

        let html = tab.get_content()?;
        let dom = Dom::parse(&html)?;


        std::fs::write("out.html", html)?;
    }
}


fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s);
    return s;
}