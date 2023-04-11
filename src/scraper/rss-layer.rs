// Represents the content aggregator's RSS integration.
// RSS is probably the only way I'll pull info off the web, but who knows? Maybe I'll build it out.

// TODO: Implement

use eyre::Result;

use rss::Channel;

use crate::content::Article;

use super::MultiScraper;

struct RssScraper {
    channel: Channel,
}

impl MultiScraper for RssScraper {
    fn scrape_articles(&self) -> Vec<Article> {
        todo!()
    }
}

impl RssScraper {
    pub fn new(rss_url: &str) -> Result<Self> {
        todo!()
    }
}
