use std::{sync::Arc, time::Duration};

use headless_chrome::{Browser, LaunchOptions};

use crate::word_reference_scraper;
use scraper::{Html, Selector};


// A BrowserSession represents a session controlling a Chrome browser window.
pub struct BrowserSession {
    _browser: headless_chrome::Browser,
    live_tab: Arc<headless_chrome::Tab>,
}

impl BrowserSession {
    /** 
     * navigate_to goes to the given URL
     */
    pub fn navigate_to(&self, url: &str) -> anyhow::Result<()> {
        self.live_tab.navigate_to(url)?;
        Ok(())
    }

    /** 
     * new returns a new BrowserSession. The browser exists until the returned object
     * is dropped.
     */
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

pub struct WordReferenceSpEnSession {
    url: String,
    session: BrowserSession,
}

impl WordReferenceSpEnSession {
    /** 
     * new returns a new spanish-english session for word reference.com.
     */
    pub fn new(url: &str) -> anyhow::Result<Self> {
        let session = BrowserSession::new()?;
        session.navigate_to(url)?;
        return Ok(Self { session: session, url: url.to_string()});
    }

    /**
     * lookup navigates to the entry for the given word in the dictionary.
     */
    pub fn lookup(&self, word: &str) -> anyhow::Result<()>{
        self.session.navigate_to(&self.word_query_url(word))?;

        let _ = self.session.live_tab.wait_for_element_with_custom_timeout("table.WRD.clickTranslate.noTapHighlight", Duration::new(2, 0))?;
        Ok(())
    }

    /**
     * word_query_url builds the URL to search for the given word.
     */
    fn word_query_url(&self, word: &str) -> String {
        format!("{}{}", self.url, word)
    }

    /**
     * navigate_and_scrape_page attempts to navigate the browser session to the WordReference page
     * associated with the given word. 
     */
    pub fn navigate_and_scrape_page(&self, word: &str) -> anyhow::Result<word_reference_scraper::WordReferencePage> {
        self.lookup(word)?;

        self.session.live_tab.wait_for_element_with_custom_timeout("table.WRD.clickTranslate.noTapHighlight", Duration::new(15,0))?;
        let page_html = self.session.live_tab.get_content()?;
        return word_reference_scraper::scrape_page_html(page_html);
    }
}
