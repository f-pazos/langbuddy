use std::{sync::Arc, time::Duration};

use headless_chrome::{Browser, LaunchOptions};
use scraper::{html, ElementRef, Html, Selector};
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

struct DefinitionTable {
    section_name: String,
    entries: Vec<DefinitionTableEntry>,
}
struct DefinitionTableEntry {
    word: String,
    spanish_definition: String,
    english_definitions: Vec<String>,
    examples: Vec<String>,
}

// next processes a WordReference table into groupings of related rows. WordReference
// splits table entries accross multiple HTML elements, grouped by their class names.
// For table contents, these classes are alternating "even" and "odd".
fn tokenize_table<'a, 'b>(selection: html::Select<'a, 'b>) -> Vec<Vec<scraper::ElementRef<'a>>>{
    let mut result = vec!();

    let mut current_class: Option<&str> = None;
    let mut current_elems = vec!();

    for tr in selection {
        let tr_class = tr.attr("class");
        if tr_class != current_class {
            current_class = tr_class;
            result.push(current_elems);
            current_elems = vec!(tr);
            continue;
        };
        current_elems.push(tr);
    };

    return result;
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
        todo!();
    }

    fn split_even_and_odd_rows(rows: Vec<scraper::ElementRef>) -> Vec<Vec<scraper::ElementRef>>{  
        let entries = Vec::<Vec<scraper::ElementRef>>::new();


        entries
    }

    // get_definition returns the definition for the word on the given page. 
    pub fn get_definition(&self) -> anyhow::Result<String>{
        let definition_table = self.session.live_tab.wait_for_element("table.WRD.clickTranslate.noTapHighlight");
        if definition_table.is_err() {
            return Err(definition_table.unwrap_err());
        }
        let definition_table = definition_table.unwrap();
        let html = definition_table.get_content()?;
        let document = Html::parse_fragment(&html);

        let selector = Selector::parse("tr").unwrap();

        let sections = tokenize_table(document.select(&selector));
        for section in sections {
            for element in section {
                println!("{}", element.html());
            }
            println!();
        }
        return Ok("ok!".to_string());
    }
}

// TODO: implement tokenizer as a iterator. Use iterator windows to do so.

// struct TableTokenizer<'a, 'b>{
//     iter: html::Select<'a, 'b>,
//     cursor: Option<scraper::ElementRef<'a>>,
// }

// impl <'a, 'b> Iterator for TableTokenizer<'a,'b>{
//     type Item = Vec<ElementRef<'a>>;

//     // next processes a WordReference table into groupings of related rows. WordReference
//     // splits table entries accross multiple HTML elements, grouped by their class names.
//     // For table contents, these classes are alternating "even" and "odd".
//     fn next(&mut self) -> Option<Vec<scraper::ElementRef<'a>>>{
//         // let next = self.iter.by_ref().peekable().peek();

//         let next = self.iter.by_ref().next();
//         if next.is_none() {
//             return None;
//         };
//         let next = next.unwrap();

//         let tr_class = next.attr("class");
//         if tr_class.is_none() {
//             return Some(vec!(next));
//         };
//         let tr_class = tr_class.unwrap();

//         return Some(
//             self.iter.by_ref().take_while(
//                 |tr| {
//                     match tr.attr("class"){
//                         None => return false,
//                         Some(class) => return (class == tr_class),
//                     };
//             })
//         .collect());
//     }
// }
