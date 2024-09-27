use itertools::Itertools;
use scraper::{Html, html, ElementRef, Selector};
use std::{fmt, collections::HashMap};

/**
 * A WordReferencePage stores the information for a single page
 * of the WordReference Spanish/English dictionary. Each WordReferencePage
 * often has several different forms of hte word that map to it (as an example,
 * different conjugations of the same verb). As such, one should not think of
 * WordReferencePage as being a bijection to words. Instead, each word uniquely
 * maps to a page in the WordReference dictionary.
 */
pub struct WordReferencePage {  
    principal_translations: Vec<DefinitionEntry>,
    additional_translations: Vec<DefinitionEntry>,
    compound_forms: Vec<DefinitionEntry>,
    also_appears_in_spanish: Vec<String>,
    also_appears_in_english: Vec<String>,
}

type DefinitionEntryID = String;

/**
 * A WordReferenceEntry represents a single row within
 * a WordReference page section. Each of these sections
 * are unique entries and have unique IDs.
 */
struct DefinitionEntry {
    id: DefinitionEntryID,
    word: String,
    spanish_definition: DefinitionAndPOS,
    english_definition: Vec<DefinitionAndPOS>,
    spanish_examples: Vec<String>,
    english_examples: Vec<String>,
}


/**
 * A Definition comprises of a string that defines a word as well as a
 * part of speech for that given word sense.
 */
struct DefinitionAndPOS {
    text: String,
    part_of_speech: String,
}


/**
 * TODO: DEPECATE
 */

struct DefinitionTable {
    entries: Vec<DefinitionTableEntry>,
}

#[derive(Debug)]
pub struct DefinitionTableEntry {
    word: TaggedWord,
    spanish_definition: String,
    english_definitions: Vec<TaggedWord>,
    examples: Examples,
}

impl fmt::Display for TaggedWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.word, self.part_of_speech)
    }
}

impl fmt::Display for DefinitionTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let new_line_split = format!("\n{:>9}\t", "");
        write!(f, 
            "    word:\t{}\nspan_def:\t{}\n  en_def:\t{}\n span_ex:\t{}\n   en_ex:\t{}", 
            self.word, 
            self.spanish_definition, 
            self.english_definitions.iter().map(|tw| format!("{}", tw)).join(&new_line_split), 
            self.examples.spanish.join(&new_line_split),
            self.examples.english.join(&new_line_split),
        )
    }
}

pub type DefinitionTableRow<'a> = Vec<scraper::ElementRef<'a>>;

#[derive(Debug)]
struct Examples {
    spanish: Vec<String>,
    english: Vec<String>,
}

#[derive(Debug)]
struct TaggedWord {
    word: String,
    part_of_speech: String,
}

// split_table_into_rows processes a WordReference HTML table the rows that they represent.
// WordReference uses colors to associate multiple <tr> elements together so that they can
// act as a row with multiple lines. These multi-row groupings alternate the CSS classes of
// "even" and "odd". This function splits the table into those groupings and returns a Vec of
// those variable-width rows.
pub fn split_table_into_entries<'a>(fragment: scraper::ElementRef) -> Vec<DefinitionTableRow<'a>> {
    let mut result = vec!();

    let mut current_class: Option<&str> = None;
    let mut current_elems = vec!();

    let selector = Selector::parse("tr").unwrap();

    for tr in fragment.select(&selector) {
        let tr_class = tr.attr("class");
        if tr_class != current_class {
            current_class = tr_class;
            result.push(current_elems);
            current_elems = vec!(tr);
            continue;
        };
        current_elems.push(tr);
    };

    result.push(current_elems);
    return result;
}

// extract_table_entry parses the information found in a single definition table section.
pub fn extract_table_entry(section: &DefinitionTableRow) -> Option<DefinitionTableEntry> {
    let (word, definition) = extract_spanish_word_and_definition(&section)?;
    Some(
        DefinitionTableEntry{ 
            word: word, 
            spanish_definition: definition, 
            english_definitions: extract_english_definitions(&section),
            examples: extract_examples(&section),
        })
}

// extract_examples parses the rows and returns a list of examples, both spanish and english.
fn extract_examples(section: &DefinitionTableRow) -> Examples {
    let td_selector = Selector::parse("td").unwrap();
    let all_tds = section.iter()
        .flat_map(|e| e.select(&td_selector));

    let collect_language_examples = |lang: &str| {
        all_tds.clone()
            .filter(|e| e.attr("class") == Some(lang))
            .map(|e| e.text().join(" "))
            .collect::<Vec<String>>()
    };

    return Examples{
        spanish: collect_language_examples("FrEx"),
        english: collect_language_examples("ToEx"),
    };
}

fn extract_spanish_word_and_definition(section: &DefinitionTableRow) -> Option<(TaggedWord, String)> {
    // Count how many <td> elements match the From Word "FrWrd" class. Ensure there is only one 
    // match per section.
    let mut count_fr_wrd = 0;
    let mut word_and_definition = None;

    for tr in section {
        for (td_1, td_2) in tr.select(&Selector::parse("td").unwrap()).tuple_windows() {
            if td_1.attr("class") == Some("FrWrd") && td_2.attr("class") == None {
                count_fr_wrd += 1;
                word_and_definition = parse_word_and_definition(td_1, td_2);
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
fn extract_english_definitions(section: &DefinitionTableRow) -> Vec<TaggedWord> {
    let mut result = vec!();

    for tr in section {
        let mut definitions = tr.select(&Selector::parse("td").unwrap())
            .tuple_windows()
            .filter(|(_, td_2)| td_2.attr("class") == Some("ToWrd"))
            .map(|(td_1, td_2)| extract_english_definition(td_1, td_2))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect::<Vec<TaggedWord>>();

        result.append(&mut definitions);
    };

    result
}

fn extract_english_definition(td_1: scraper::ElementRef, td_2: scraper::ElementRef) -> Option<TaggedWord> {
    let mut prefix = String::new();
    let span = single_selection_match(td_1, "span.dsense");
    if span.is_some() { 
        prefix = format!("{} ", span.unwrap().text().join(" "));
    }
    let suffix = td_2.text().next()?;

    let pos = extract_pos_from_td(td_2).unwrap_or("no_POS".to_string());
    
    Some(
        TaggedWord{
            word: format!("{}{}", prefix, suffix),
            part_of_speech: pos,
        },
    )
}

// extract_pos_from_td parses the part of speech from within the <td> element.
// 
fn extract_pos_from_td(td: scraper::ElementRef) -> Option<String> {
    Some(single_selection_match(td, "em.POS2")?.text().join(" "))
}

// parse_word_and_definition removes extraneous information that might exist in the <td> elements for a word
// and its definition.
fn parse_word_and_definition(td_1: scraper::ElementRef, td_2: scraper::ElementRef) -> Option<(TaggedWord, String)> {
    let word = single_selection_match(td_1, "strong")?.text().next()?;

    let pos = extract_pos_from_td(td_1).unwrap_or("no_POS".to_string());

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
 
    Some(
        (
            TaggedWord{
                word: word.to_string(), 
                part_of_speech: pos
            }, 
            scrubbed.chars().skip(1).collect(),
        ),
    )
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

/**
 * scrape_html attempts to marshall the given page into a WordReferencePage object. 
 * Returns an error if the scraper fails to marshall the HTML into a word reference
 * object. 
 */
pub fn scrape_html(html: String) -> anyhow::Result<WordReferencePage> {
    let document = Html::parse_fragment(&html);
    let selector = Selector::parse("table.WRD.clickTranslate.noTapHighlight").unwrap();
    let sections = document.select(&selector);

    let mut word_reference_page = WordReferencePage{
        principal_translations: vec!(),
        additional_translations: vec!(),
        compound_forms: vec!(),
        also_appears_in_spanish: vec!(), 
        also_appears_in_english: vec!(),
    };


    let mut results = HashMap::new();
    for section in sections {
        let (section_name, entries) = scrape_word_reference_section(section)?;
        results.insert(section_name, entries);
    }

    if results.contains_key(&PageSection::PrincipalTranslations){
        word_reference_page.principal_translations = results.remove(&PageSection::PrincipalTranslations).unwrap();
    };
    if results.contains_key(&PageSection::CompoundForms){
        word_reference_page.additional_translations = results.remove(&PageSection::AdditionalTranslations).unwrap();
    };
    if results.contains_key(&PageSection::CompoundForms){
        word_reference_page.compound_forms = results.remove(&PageSection::CompoundForms).unwrap();
    };

    let also_appears = scrape_also_appears_in(document)?;
    word_reference_page.also_appears_in_english = also_appears.english;
    word_reference_page.also_appears_in_spanish = also_appears.spanish;


    Ok(word_reference_page)
}

#[derive(PartialEq, Eq, Hash)]
enum PageSection {
    PrincipalTranslations,
    AdditionalTranslations,
    CompoundForms,
}

fn scrape_word_reference_section(section: scraper::ElementRef) -> anyhow::Result<(PageSection, Vec<DefinitionEntry>)>{
    let header = single_selection_match(section, "tr.wrtopsection td");
    if header.is_none() {
        return Err(anyhow::anyhow!("could not find single WordReferenceSection header match"));
    }

    let section_name = match header.unwrap().attr("title") {
        None => return Err(anyhow::anyhow!("could not find attribute in PageSection header")),
        Some("Principal Translations") => PageSection::PrincipalTranslations,
        Some("Additional Translations") => PageSection::AdditionalTranslations,
        Some("Compound Forms:") => PageSection::CompoundForms,
        Some(s) => return Err(anyhow::anyhow!("could not match title attribute {} for PageSection header", s))
    };



    let selector = Selector::parse("tr").unwrap();
    let section_entries = split_table_into_entries(section);
    

    todo!();
}

fn scrape_table_entry(entry: &DefinitionTableRow) -> anyhow::Result<DefinitionEntry> {
    Ok(
        DefinitionEntry {
            id: scrape_entry_id(entry)?,
            spanish_definition: 
        }
// struct DefinitionEntry {
//     id: DefinitionEntryID,
//     word: String,
//     spanish_definition: DefinitionAndPOS,
//     english_definition: Vec<DefinitionAndPOS>,
//     spanish_examples: Vec<String>,
//     english_examples: Vec<String>,
// }
    )
}

fn scrape_entry_id(entry: ElementRef) -> anyhow::Result<DefinitionEntryID>{
    todo!();
}

fn scrape_spanish_definition(entry: ElementRef) -> anyhow::Result<DefinitionAndPOS>{
    todo!();
}

fn scrape_english_definitions(entry: ElementRef) -> anyhow::Result<Vec<DefinitionAndPOS>>{
    todo!();
}

// // extract_table_entry parses the information found in a single definition table section.
// pub fn extract_table_entry(section: &DefinitionTableRow) -> Option<DefinitionTableEntry> {
//     let (word, definition) = extract_spanish_word_and_definition(&section)?;
//     Some(
//         DefinitionTableEntry{ 
//             word: word, 
//             spanish_definition: definition, 
//             english_definitions: extract_english_definitions(&section),
//             examples: extract_examples(&section),
//         })
// }



/**
 * BIG TOODO
 */
struct AlsoAppears {
    spanish: Vec<String>,
    english: Vec<String>,
}
fn scrape_also_appears_in(html: html::Html) -> anyhow::Result<AlsoAppears>{
    Ok(AlsoAppears { spanish: vec!(), english: vec!() }) 
}
