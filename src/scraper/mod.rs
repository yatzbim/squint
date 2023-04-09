use std::collections::HashSet;

// Module containing traits specifying specific scraping behavior and specific integrations to implement that behavior.
// We'll start with RSS and see how it goes.
#[path = "rss-layer.rs"]
mod rss_layer;

pub trait Scraper {
    fn scrape_title(&self) -> String;
    fn scrape_creator(&self) -> String;
    fn scrape_description(&self) -> String;
    fn scrape_date(&self) -> String;
    fn scrape_categories(&self) -> HashSet<String>;
    fn scrape_link(&self) -> String;
    fn scrape_body(&self) -> String;
}
