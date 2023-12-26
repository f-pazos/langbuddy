use std::error::Error;
use std::time::{SystemTime, self};

use headless_chrome::Browser;
use headless_chrome::protocol::cdp::Page;

fn main() -> anyhow::Result<()>{

    let last_timestamp = SystemTime::now();

    let browser = Browser::default()?;
    println!("Browser::default(): {}", last_timestamp.elapsed()?.as_millis());

    let tab = browser.new_tab()?;
    println!("browser.new_tab(): {}", last_timestamp.elapsed()?.as_millis());

    // Navigate to wikipedia
    let content = tab.navigate_to("https://www.collinsdictionary.com/us/dictionary/spanish-english/casa")?.get_content()?;
    println!("navigate_to: {}", last_timestamp.elapsed()?.as_millis());

    std::thread::sleep(time::Duration::from_millis(100));

    std::fs::write("out.html", content)?;


    Ok(())
}