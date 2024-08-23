use std::{sync::Arc, time::Duration};

use headless_chrome::{Browser, LaunchOptions};
use scraper::{ElementRef, Html, Selector};
use anyhow::anyhow;

// A BrowserSession represents a session controlling a Chrome browser window.
pub struct BrowserSession {
    _browser: headless_chrome::Browser,
    live_tab: Arc<headless_chrome::Tab>,
}

impl BrowserSession {
    // navigate_to goes to the given URL
    pub fn navigate_to(&self, url: &str) -> anyhow::Result<()> {
        self.live_tab.navigate_to(url)?;
        Ok(())
    }

    // new returns a new BrowserSession. The browser exists until the returned object
    // is dropped.
    pub fn new() -> anyhow::Result<BrowserSession> {
        let browser = Browser::new(
            LaunchOptions::default_builder()
                .headless(false)
                .devtools(false)
                .build()
                .expect("Could not find chrome-executable"),
        )?;

        let tab  = browser.new_tab()?;

        return Ok(BrowserSession {
            _browser: browser,
            live_tab: tab,
        });
    }
}

const WORD_REFERENCE: &str = "https://www.wordreference.com";
const WORD_REFERENCE_SP_EN_QUERY: &str =
    "https://www.wordreference.com/es/en/translation.asp?spen=";

pub struct WordReferenceSpEnSession {
    session: BrowserSession,
}

pub struct WordReferenceSpEnEntry {
    spanish_word: String,
    spanish_definitions: Vec<String>,
    english_definitions: Vec<String>,
    spanish_examples: Vec<String>,
    english_examples: Vec<String>,
}

impl WordReferenceSpEnSession {
    // new returns a new spanish-english session for word reference.com.
    pub fn new() -> anyhow::Result<Self> {
        let session = BrowserSession::new()?;
        session.navigate_to(WORD_REFERENCE)?;
        return Ok(Self { session: session });
    }

    // lookup navigates to the entry for the given word in the dictionary.
    pub fn lookup(&self, word: &str) -> anyhow::Result<()>{
        self.session.navigate_to(&Self::word_query_url(word))
    }

    // word_query_url builds the URL to search for the given word.
    fn word_query_url(word: &str) -> String {
        format!("{}{}", WORD_REFERENCE_SP_EN_QUERY, word)
    }

    // extract_page creates a WordReferenceSpEnEntry object to represent
    // the page that is currently displayed.
    pub fn extract_page(&self) -> anyhow::Result<WordReferenceSpEnEntry> {
        let table = self.session.live_tab
            .wait_for_element_with_custom_timeout(
                "table.WRD.clickTranslate.noTapHighlight tbody",
                Duration::from_millis(500),
            )?;

        let document = Html::parse_fragment(&table.get_content()?);

        for row in document.select(&Selector::parse("tr").unwrap()){

        };
    }

    // split_even_and_odd_rows processes a WordReference dictionary page table
    // to split it into the entries it represents. WordReference represents
    // multiple meanings in a single HTML table element. It uses alternating
    // sections of either "even" or "odd" class rows to differentiate the entries.
    fn split_even_and_odd_rows(rows: Vec<scraper::ElementRef>) -> Vec<Vec<scraper::ElementRef>>{  
        let entries = Vec::<Vec<scraper::ElementRef>>::new();

        entries
    }



    // get_definition returns the definition for the word on the given page. 
    pub fn get_definition(&self) -> anyhow::Result<String>{
        let definition_table = self.session.live_tab.wait_for_element("table.WRD.clickTranslate.noTapHighlight tbody");
        if definition_table.is_err() {
            return Err(definition_table.unwrap_err());
            // return println!("could not extract definition div: {}", definition_table.unwrap_err());
        }
        let definition_table = definition_table.unwrap();
        let html = definition_table.get_content()?;
        println!("{}", html);
        let document = Html::parse_fragment(&html);

        // println!("{:?}", definition_table.get_content());
        // let rows = definition_table.find_elements("tr");
        // let rows = document.select("tr").collect();

        let selector = Selector::parse("tr").unwrap();
        for element in document.select(&selector) {
            println!("{}", element.html())
        };

        return Ok("ok!".to_string());

        // let words = Vec::<WordReferenceSpEnEntry>::new();
        // for row in rows.unwrap() {
        //     let value = row.get_attribute_value("class");
        //     println!("{:?}", value)
        // }
        // let definition_table.get_content().unwrap_or("could not unwrap definition div contents".to_string())



    }
}
