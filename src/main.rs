use std::error::Error;
use std::ffi::OsStr;
use std::{io, fs};
use std::path::PathBuf;
use std::time::{SystemTime, self};

use headless_chrome::types::Bounds;
use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::protocol::cdp::Page;


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
            .window_size(Some((1024, 1280)))
            .build()
            .expect("Could not find chrome-executable"),
    )?;

    let tab = browser.new_tab()?;
    tab.set_bounds(Bounds::Normal{left:Some(0), top: Some(0), width: Some(1024.0), height: Some(1280.0)})?;

    loop { 
        println!("input a word to query");
        let word = input();

        tab.navigate_to(&format!("{}{}", WORD_REFERENCE_SP_EN_QUERY, word))?;
        tab.wait_until_navigated()?;

        let html = tab.get_content()?;


        std::fs::write("out.html", html)?;
    }
}


fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s);
    return s;
}