// Structs to represent the articles themselves
// Will be the return values of the scrapers and DB reads, and will be the arguments into the DB writes and shell IO writes.

// TODO: implement

use chrono::{DateTime, Utc};
use url::Url;

use crate::formatter;

pub struct Article {
    title: String,
    author: String,
    categories: Vec<String>,
    description: String,
    body: String,
    link: Url,
    publish_date: DateTime<Utc>,
}

impl From<rss::Item> for Article {
    fn from(value: rss::Item) -> Self {
        let formatted_body = formatter::format_body(value.content().unwrap());
        let formatted_categories = formatter::format_categories(value.categories());
        Article::new_builder()
            .with_title(value.title().unwrap())
            .with_author(value.author().unwrap())
            .with_categories(formatted_categories)
            .with_description(value.description.as_ref().unwrap())
            .with_body(&formatted_body)
            .with_link(value.link().unwrap())
            .with_publish_date(value.pub_date().unwrap())
            .build()
    }
}

impl Article {
    pub fn new_builder() -> ArticleBuilder {
        ArticleBuilder::default()
    }
}

#[derive(Default)]
pub struct ArticleBuilder {
    title: Option<String>,
    author: Option<String>,
    categories: Option<Vec<String>>,
    description: Option<String>,
    body: Option<String>,
    link: Option<Url>,
    publish_date: Option<DateTime<Utc>>,
}

impl ArticleBuilder {
    pub fn with_title(mut self, title: &str) -> Self {
        todo!()
    }

    pub fn with_author(mut self, author: &str) -> Self {
        todo!()
    }

    pub fn with_categories(mut self, categories: Vec<String>) -> Self {
        todo!()
    }

    pub fn with_description(mut self, description: &str) -> Self {
        todo!()
    }

    pub fn with_body(mut self, body: &str) -> Self {
        todo!()
    }

    pub fn with_link(mut self, link: &str) -> Self {
        todo!()
    }

    pub fn with_publish_date(mut self, publish_date: &str) -> Self {
        todo!()
    }

    pub fn build(self) -> Article {
        todo!()
    }
}
