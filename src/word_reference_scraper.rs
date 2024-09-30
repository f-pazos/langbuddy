use itertools::Itertools;
use scraper::{Html, html, ElementRef, Selector};
use std::{fmt, collections::HashMap};

/**
 * A WordReferencePage stores the information for a single page
 * of the WordReference Spanish/English dictionary. Each WordReferencePage
 * often has several different forms of hte word that map to it (as an example,
 * different conjugations of the same verb). As such, one should not think of the
 * relationship between WordReferencePage and words as a bijection. Instead, each 
 * word uniquely maps to a page in the WordReference dictionary.
 */
#[derive(Debug)]
pub struct WordReferencePage {  
    pub principal_translations: Vec<DefinitionEntry>,
    pub additional_translations: Vec<DefinitionEntry>,
    pub compound_forms: Vec<DefinitionEntry>,
    pub also_appears_in_spanish: Vec<String>,
    pub also_appears_in_english: Vec<String>,
}

fn fmt_page_section(section_name: &str, content: &Vec<DefinitionEntry>) -> String{    
    format!("{}:\n\n{}", section_name, content.iter().join("\n\n"))
}

impl fmt::Display for WordReferencePage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n\n{}\n\n{}\n\n{}\n\n{}", 
            fmt_page_section("PRINCIPAL TRANSLATIONS", &self.principal_translations),
            fmt_page_section("ADDITIONAL TRANSLATIONS", &self.additional_translations), 
            fmt_page_section("COMPOUND FORMS", &self.compound_forms),
            "TODO: ALSO APPEARS SPAN",
            "TODO: ALSO APPEARS ENG",
        )
    }
}

type DefinitionEntryID = String;

/**
 * A DefinitionEntry represents a WordReference definition block. These comprise
 * of a single definition in spanish, corresponding definitions in English, parts
 * of speech, as well as several examples in both English and Spanish. 
 */
#[derive(Debug)]
pub struct DefinitionEntry {
    pub id: DefinitionEntryID,
    pub word: String,
    pub spanish_definition: DefinitionAndPOS,
    pub english_definitions: Vec<DefinitionAndPOS>,
    pub spanish_examples: Vec<String>,
    pub english_examples: Vec<String>,
}

impl fmt::Display for DefinitionEntry{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
"id: {}, 
word: {}, 
spanish_definition: {},
english_definitions: {},
spanish_examples: {},
english_examples: {}", 
            self.id, 
            self.word, 
            self.spanish_definition, 
            self.english_definitions.iter().join("\n\t"),
            self.english_examples.iter().join("\n"), 
            self.spanish_examples.iter().join("\n"),
        )
    }
}

/**
 * A DefinitionAndPOS comprises of the text of a word's definition as well
 * as its part of speech. 
 */
#[derive(Debug)]
pub struct DefinitionAndPOS {
    pub text: String,
    pub part_of_speech: String,
}

impl fmt::Display for DefinitionAndPOS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}> {}", self.part_of_speech, self.text)
    }
}

pub type DefinitionTableRow<'a> = Vec<scraper::ElementRef<'a>>;

#[derive(Debug)]
struct Examples {
    spanish: Vec<String>,
    english: Vec<String>,
}

// split_table_into_rows processes a WordReference HTML table the rows that they represent.
// WordReference uses colors to associate multiple <tr> elements together so that they can
// act as a row with multiple lines. These multi-row groupings alternate the CSS classes of
// "even" and "odd". This function splits the table into those groupings and returns a Vec of
// those variable-width rows.
pub fn split_table_into_entries(fragment: scraper::ElementRef) -> Vec<DefinitionTableRow> {
    let mut result = vec!();

    let mut last_class: Option<&str> = None;
    let mut current_elems = vec!();

    let selector = Selector::parse("tr").unwrap();
    let entries = fragment.select(&selector).filter(
        |tr| {
            let class = tr.attr("class");
            return class == Some("odd") || class == Some("even");
        });

    for tr in entries { 
        let tr_class = tr.attr("class");
        if last_class == None {
            last_class = tr_class;
        }

        if last_class != tr_class {
            result.push(current_elems);
            current_elems = vec!();
            last_class = tr_class;
        };
        current_elems.push(tr);
    };
    result.push(current_elems);

    return result;
}

// scrape_examples parses the rows and returns a list of examples, both spanish and english.
fn scrape_examples(section: &DefinitionTableRow) -> Examples {
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
        spanish: collect_language_examples("ToEx"),
        english: collect_language_examples("FrEx"),
    };
}

// scrape_english_definitions extracts all english definitions associated with the section
// of the table.
fn scrape_english_definitions(section: &DefinitionTableRow) -> Vec<DefinitionAndPOS> {
    let mut result = vec!();

    for tr in section {
        let mut definitions = tr.select(&Selector::parse("td").unwrap())
            .tuple_windows()
            .filter(|(_, td_2)| td_2.attr("class") == Some("ToWrd"))
            .map(|(td_1, td_2)| scrape_english_definition(td_1, td_2))
            .filter(|o| o.is_ok())
            .map(|o| o.unwrap())
            .collect::<Vec<DefinitionAndPOS>>();

        result.append(&mut definitions);
    };

    result
}

// scrape_english_definition extracts a single english definition associated with the section
// of the table.
fn scrape_english_definition(td_1: scraper::ElementRef, td_2: scraper::ElementRef) -> anyhow::Result<DefinitionAndPOS> {
    let mut prefix = String::new();
    let span = match_exactly_one_element(td_1, "span.dsense");
    if span.is_some() { 
        prefix = format!("{} ", span.unwrap().text().join(" "));
    }

    let suffix = td_2.text().next();
    if suffix.is_none(){
        return Err(anyhow::anyhow!("could not extract english definition information from second <td> element"))
    }

    let pos = extract_pos_from_td(td_2).unwrap_or("no_POS".to_string());
    
    Ok(
        DefinitionAndPOS{
            text: format!("{}{}", prefix, suffix.unwrap()),
            part_of_speech: pos,
        }
    )
}

// extract_pos_from_td parses the part of speech from within the <td> element.
fn extract_pos_from_td(td: scraper::ElementRef) -> Option<String> {
    Some(match_exactly_one_element(td, "em.POS2")?.text().join(" "))
}

// parse_spanish_definition puts together a spanish definition from two adjacent <td> elements.
fn parse_spanish_definition(td_1: scraper::ElementRef, td_2: scraper::ElementRef) -> anyhow::Result<DefinitionAndPOS> {
    let word = match_exactly_one_element(td_1, "strong");
    if word.is_none() {
        return Err(anyhow::anyhow!("couldn't find a single <strong> element in spanish definition <td> element"));
    };
     
    let word = word.unwrap().text().next();
    if word.is_none() {
        return Err(anyhow::anyhow!("didn't find expected second text element in spanish definition."))
    };

    let pos = extract_pos_from_td(td_1).unwrap_or("no_POS".to_string());

    let definition = td_2.text().next();
    if definition.is_none() {
        return Err(anyhow::anyhow!("failed to extract spanish definition"))
    }

    // The definition is within parenthesis so we need to trim everything that comes after them. 
    // In some cases the same <td> is reused for a portion of the english definition.
    let mut level = 0;
    let mut taken = 0;

    let scrubbed = definition.unwrap().trim().chars().take_while(|c|{
        taken += 1;
        match c {
            '(' => level += 1,
            ')' => level -= 1, 
            _ => (),
        };
        return !(level == 0 && taken > 1);
    }).collect::<String>();

    Ok(
        DefinitionAndPOS{
            text: scrubbed,
            part_of_speech: pos,
        },
    )
}

/** 
 * match_exactly_one_element applies the selector to ElementRef, and returns the value if there is exactly
 *  one match.
 */
fn match_exactly_one_element<'a>(element: ElementRef<'a>, selector: &str) -> Option<ElementRef<'a>> {
    let mut results = element.select(&Selector::parse(selector).unwrap()).collect::<Vec<ElementRef>>();
    if results.len() != 1 {
        return None
    };
    return results.pop();
}

/**
 * scrape_page_html attempts to marshall the given page into a WordReferencePage object. 
 * Returns an error if the scraper fails to marshall the HTML into a word reference
 * object. 
 */
pub fn scrape_page_html(html: String) -> anyhow::Result<WordReferencePage> {
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
        let (section_name, entries) = scrape_page_section(section)?;
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


/** 
 * scrape_page_section scrapes a single table of entries from the WordReference page. These are either 
 * "Principal Translations", "Additional Translations", or "Compound Forms".
 */
fn scrape_page_section(section: scraper::ElementRef) -> anyhow::Result<(PageSection, Vec<DefinitionEntry>)>{
    let header = match_exactly_one_element(section, "tr.wrtopsection td");
    if header.is_none() {
        return Err(anyhow::anyhow!("could not find single WordReferenceSection header match"));
    }

    let section_name = match header.unwrap().attr("title") {
        None => return Err(anyhow::anyhow!("could not find attribute in PageSection header")),
        Some("Principal Translations") => PageSection::PrincipalTranslations,
        Some("Additional Translations") => PageSection::AdditionalTranslations,
        Some("Compound Forms") => PageSection::CompoundForms,
        Some(s) => return Err(anyhow::anyhow!("could not match title attribute {} for PageSection header", s))
    };

    let mut definitions = vec!();

    let entries = split_table_into_entries(section);

    for entry in entries {
        let definition = scrape_table_entry(&entry)?;
        definitions.push(definition);
    }

    return Ok((section_name, definitions))
}

// scrape_table_entry scrapes a single table entry. The entry is supplied as a vec
// of consecutive <tr> elements that form a single dictionary entry on a
// WordReference page. 
fn scrape_table_entry(entry: &DefinitionTableRow) -> anyhow::Result<DefinitionEntry> {
    let examples = scrape_examples(entry);

    if entry.len() == 0 {
        return Err(anyhow::anyhow!("entry did not contain any <tr> elements"));
    }
    let first_tr = entry.get(0).unwrap();

    let id = first_tr.attr("id");
    if id.is_none() {
        return Err(anyhow::anyhow!("first tr had no id attribute"));
    }
    let id = id.unwrap().to_string();

    let word = scrape_word(first_tr)?;

    Ok(
        DefinitionEntry {
            id: id,
            word: word,
            spanish_definition: scrape_spanish_definition(entry)?,
            english_definitions: scrape_english_definitions(entry),
            spanish_examples: examples.spanish,
            english_examples: examples.english,
        }
    )
}

/**
 * scrape_word attempts to scrape the word for the given DefinitionTableRow entry object.
 */
fn scrape_word(tr: &ElementRef) -> anyhow::Result<String>{
    let strong_element = match_exactly_one_element(*tr, "td.FrWrd strong");
    if strong_element.is_none() {
        return Err(anyhow::anyhow!("problem selecting strong element from first member of DefinitionTableRow"))
    }

    let text = strong_element.unwrap().text().next();
    if text.is_none() {
        return Err(anyhow::anyhow!("DefintionTableRow object did not have any child text elements"));
    }

    Ok(text.unwrap().to_string())
}

fn scrape_spanish_definition(entry: &DefinitionTableRow) -> anyhow::Result<DefinitionAndPOS>{
    // Count how many <td> elements match the From Word "FrWrd" class. Ensure there is only one 
    // match per section.
    let mut count_fr_wrd = 0;
    let mut result = None; 

    for tr in entry {
        for (td_1, td_2) in tr.select(&Selector::parse("td").unwrap()).tuple_windows() {
            if td_1.attr("class") == Some("FrWrd") && td_2.attr("class") == None {
                count_fr_wrd += 1;
                result = Some(parse_spanish_definition(td_1, td_2)?);
            }
        };
    };

    if count_fr_wrd != 1 {
        return Err(anyhow::anyhow!("failed to scrape spanish definition; could not find any <td> elements of class FrWrd"));
    };

    Ok(result.unwrap())
}

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
