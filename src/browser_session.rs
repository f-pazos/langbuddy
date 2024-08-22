use std::sync::Arc;

use headless_chrome::{Browser, LaunchOptions};

const WORD_REFERENCE: &str = "https://www.wordreference.com";
const WORD_REFERENCE_SP_EN_QUERY: &str =
    "https://www.wordreference.com/es/en/translation.asp?spen=";

pub struct BrowserSession {
    browser: headless_chrome::Browser,
    live_tab: Arc<headless_chrome::Tab>,
}

impl BrowserSession {
    pub fn navigate_to(&self, url: &str) -> anyhow::Result<()> {
        self.live_tab.navigate_to(url)?;
        Ok(())
    }

    pub fn new() -> anyhow::Result<BrowserSession> {
        let browser = Browser::new(
            LaunchOptions::default_builder()
                .headless(false)
                .devtools(false)
                // .disable_default_args(true)
                // .window_size(Some((1024, 1280)))
                .enable_logging(true)
                .build()
                .expect("Could not find chrome-executable"),
        )?;

        let tabs = browser.get_tabs();
        let tabs = tabs.lock().expect("Poisoned lock").clone();

        for t in tabs.iter().skip(1) {
            let closed = t.close_with_unload()?;
            if !closed {
                return Err(anyhow::anyhow!(
                    "failed to close default tab with URL {}",
                    t.get_url()
                ));
            }
        }

        let tab = tabs.first().expect("no tabs");

        return Ok(BrowserSession {
            browser: browser,
            live_tab: tab.clone(),
        });
    }
}

// "https://www.wordreference.com/es/en/translation.asp?spen=";
pub struct WordReferenceSpEnSession {
    session: BrowserSession,
}

impl WordReferenceSpEnSession {
    // new returns a new spanish-english session for word reference.com.
    pub fn new() -> anyhow::Result<Self> {
        let session = BrowserSession::new()?;
        session.navigate_to(WORD_REFERENCE)?;
        return Ok(Self { session: session });
    }

    // lookup navigates to the entry for the given word in the dictionary.
    pub fn lookup(&self, word: &str) {
        self.session.navigate_to(&Self::word_query_url(word));
    }

    // word_query_url builds the URL to search for the given word.
    fn word_query_url(word: &str) -> String {
        format!("{}{}", WORD_REFERENCE_SP_EN_QUERY, word)
    }
}
