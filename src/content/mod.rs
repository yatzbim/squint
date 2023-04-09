// Structs to represent the articles themselves
// Will be the return values of the scrapers and DB reads, and will be the arguments into the DB writes and shell IO writes.

// TODO: implement

use std::collections::HashSet;

use chrono::{DateTime, Utc};
use url::Url;

pub struct Article {
    title: String,
    creator: String,
    categories: HashSet<String>,
    description: String,
    body: String,
    link: Url,
    date: DateTime<Utc>,
}
