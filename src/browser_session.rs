use std::{sync::Arc, time::Duration};

use headless_chrome::{Browser, LaunchOptions};
use scraper::{html, ElementRef, Html, Selector};
use anyhow::anyhow;

use itertools::Itertools;

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

#[derive(Debug)]
struct DefinitionTableEntry {
    word: String,
    spanish_definition: String,
    english_definitions: Vec<String>,
    examples: Examples,
}

#[derive(Debug)]
struct Examples {
    spanish: Vec<String>,
    english: Vec<String>,
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

// extract_table_entry parses the information found in a single definition table section.
fn extract_table_entry(rows: Vec<scraper::ElementRef>) -> Option<DefinitionTableEntry> {
    let (word, definition) = extract_spanish_word_and_definition(&rows)?;
    Some(
        DefinitionTableEntry{ 
            word: word, 
            spanish_definition: definition, 
            english_definitions: extract_english_definitions(&rows),
            examples: extract_examples(&rows),
        })
}

// extract_examples parses the rows and returns a list of examples, both spanish and english.
fn extract_examples(rows: &Vec<scraper::ElementRef>) -> Examples {
    let td_selector = Selector::parse("td").unwrap();
    let all_tds = rows.iter()
        .flat_map(|e| e.select(&td_selector));

    let collect_language_examples = |lang: &str| {
        all_tds.clone()
            .filter(|e| e.attr("class") == Some(lang))
            .map(|e| e.text().next())
            .filter(|o| o.is_some())
            .map(|o| o.unwrap().to_string())
            .collect::<Vec<String>>()
    };

    return Examples{
        spanish: collect_language_examples("FrEx"),
        english: collect_language_examples("ToEx"),
    };
}

fn extract_spanish_word_and_definition(rows: &Vec<scraper::ElementRef>) -> Option<(String, String)> {
    // Counts how many <td> elements match the From Word "FrWrd" class. Used to ensure only
    // one match per table section.
    let mut count_fr_wrd = 0;
    let mut word_and_definition = None;

    for row in rows {
        for (td_1, td_2) in row.select(&Selector::parse("td").unwrap()).tuple_windows() {
            if td_1.attr("class") == Some("FrWrd") && td_2.attr("class") == None {
                count_fr_wrd += 1;
                word_and_definition = sanitize_word_and_definition(td_1, td_2);
            }
        };
    };

    if count_fr_wrd != 1 {
        return None;
    };

    word_and_definition
}

// extract_english_definitions extracts all english definitions associated with the section
// of the table.
fn extract_english_definitions(rows: &Vec<scraper::ElementRef>) -> Vec<String> {
    let mut definitions = vec!();
    for row in rows {
        for (td_1, td_2) in 
            row.select(&Selector::parse("td").unwrap())
                .tuple_windows()
                .filter(|(_, td_2)| td_2.attr("class") == Some("ToWrd")){
            let definition = extract_english_definition(td_1, td_2);
            if definition.is_some() {
                definitions.push(definition.unwrap());
            }
        };
    };

    definitions
}

fn extract_english_definition(td_1: scraper::ElementRef, td_2: scraper::ElementRef) -> Option<String> {
    let mut prefix = String::new();
    let span = single_selection_match(td_1, "span.dsense");
    if span.is_some() { 
        prefix = format!("{} ", span.unwrap().text().join(" "));
    }
    let suffix = td_2.text().next()?;
    
    Some(format!("{}{}", prefix, suffix))
}

// sanitize_word_and_definition removes extraneous information that might exist in the <td> elements for a word
// and its definition.
fn sanitize_word_and_definition(td_1: scraper::ElementRef, td_2: scraper::ElementRef) -> Option<(String, String)> {
    let word = single_selection_match(td_1, "strong")?.text().next()?;
    let definition = td_2.text().next()?;

    // The definition is within parenthesis so we need to trim everything that comes after them. 
    // In some cases the same <td> is reused for a portion of the english definition.
    let mut level = 0;
    let mut taken = 0;

    let scrubbed = definition.trim().chars().take_while(|c|{
        taken += 1;
        match c {
            '(' => level += 1,
            ')' => level -= 1, 
            _ => (),
        };
        return !(level == 0 && taken > 1);
    }).collect::<String>();
 
    Some((word.to_string(), scrubbed.chars().skip(1).collect()))
}

// single_selection_match applies the selector to ElementRef, and returns the value if there is exactly
// one match.
fn single_selection_match<'a>(element: ElementRef<'a>, selector: &str) -> Option<ElementRef<'a>> {
    let mut results = element.select(&Selector::parse(selector).unwrap()).collect::<Vec<ElementRef>>();
    if results.len() != 1 {
        return None
    };
    return results.pop();
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

    // // extract_page creates a WordReferenceSpEnEntry object to represent
    // // the page that is currently displayed.
    // pub fn extract_page(&self) -> anyhow::Result<WordReferenceSpEnEntry> {
    //     let table = self.session.live_tab
    //         .wait_for_element_with_custom_timeout(
    //             "table.WRD.clickTranslate.noTapHighlight tbody",
    //             Duration::from_millis(500),
    //         )?;

    //     let document = Html::parse_fragment(&table.get_content()?);

    //     for row in document.select(&Selector::parse("tr").unwrap()){

    //     };
    //     todo!();
    // }

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
            println!("{:?}", extract_table_entry(section));
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
