use crate::content::Article;

// Module containing traits specifying specific scraping behavior and specific integrations to implement that behavior.
// We'll start with RSS and see how it goes.
#[path = "rss-layer.rs"]
mod rss_layer;

pub trait SingleScraper {
    fn title(&self) -> String;
    fn author(&self) -> String;
    fn description(&self) -> String;
    fn publish_date(&self) -> String;
    fn categories(&self) -> Vec<String>;
    fn link(&self) -> String;
    fn body(&self) -> String;
    fn construct_article(&self) -> Article;
}

pub trait MultiScraper {
    fn scrape_articles(&self) -> Vec<Article>;
}
