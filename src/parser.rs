
struct DefinitionTable {
    section_name: String,
    entries: Vec<DefinitionTableEntry>,
}

#[derive(Debug)]
pub struct DefinitionTableEntry {
    word: String,
    spanish_definition: String,
    english_definitions: Vec<String>,
    examples: Examples,
}

pub type DefinitionTableSection<'a> = Vec<scraper::ElementRef<'a>>;

#[derive(Debug)]
struct Examples {
    spanish: Vec<String>,
    english: Vec<String>,
}

// next processes a WordReference table into groupings of related rows. WordReference
// splits table entries accross multiple HTML elements, grouped by their class names.
// For table contents, these classes are alternating "even" and "odd".
pub fn tokenize_table<'a, 'b>(selection: html::Select<'a, 'b>) -> Vec<DefinitionTableSection<'a>> {
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
pub fn extract_table_entry(section: DefinitionTableSection) -> Option<DefinitionTableEntry> {
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
fn extract_examples(section: &DefinitionTableSection) -> Examples {
    let td_selector = Selector::parse("td").unwrap();
    let all_tds = section.iter()
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

fn extract_spanish_word_and_definition(section: &DefinitionTableSection) -> Option<(String, String)> {
    // Count how many <td> elements match the From Word "FrWrd" class. Ensure there is only one 
    // match per section.
    let mut count_fr_wrd = 0;
    let mut word_and_definition = None;

    for tr in section {
        for (td_1, td_2) in tr.select(&Selector::parse("td").unwrap()).tuple_windows() {
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
fn extract_english_definitions(section: &DefinitionTableSection) -> Vec<String> {
    let mut result = vec!();

    for tr in section {
        let mut definitions = tr.select(&Selector::parse("td").unwrap())
            .tuple_windows()
            .filter(|(_, td_2)| td_2.attr("class") == Some("ToWrd"))
            .map(|(td_1, td_2)| extract_english_definition(td_1, td_2))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect::<Vec<String>>();

        result.append(&mut definitions);
    };

    result
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
