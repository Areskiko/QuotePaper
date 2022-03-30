//! Red Rising quotes from https://www.redrisingquotes.com/api/v1/random/

use reqwest::header::{HeaderMap, ACCEPT, HeaderValue};
use serde::{Serialize, Deserialize};

use crate::input::structs::{QuoteSource, Quote, QuoteSourceBuilder};

/// Quote response from API
#[derive(Serialize, Deserialize, Debug)]
pub struct RRQuote {
    count: i32,
    results: Vec<Results>
}

/// Actual quote
#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    id: String,
    text: String,
    book: String,
    character: String,
    chapter: String,
    page_no: i32,
    tags: Vec<String>
}

/// Wrapper to build an empty Quote
pub struct Wrapper {}

impl QuoteSourceBuilder for Wrapper {
    fn build() -> Box<dyn QuoteSource> {
        Box::new(RRQuote {
            count: 0,
            results: vec![Results {
                id: "".to_string(),
                text: "".to_string(),
                book: "".to_string(),
                character: "".to_string(),
                chapter: "".to_string(),
                page_no: 0,
                tags: vec![]
            }]
        })
    }
}

impl QuoteSource for RRQuote {

    /// Convert from a RRQuote to a generic Quote
    fn get_quote(&self) -> Quote {
        Quote::new(self.results[0].text.clone(), self.results[0].character.clone())
    }
    /// Parse a source string to a RRQuote
    fn from_source(&mut self, source: &str) {
        let quote_src: RRQuote = serde_json::from_str(source).unwrap();
        self.count = quote_src.count;
        self.results = quote_src.results;
    }

    /// Get headers for the request
    fn headers(&self) -> Option<reqwest::header::HeaderMap> {
        let mut head = HeaderMap::new();
        head.insert(ACCEPT, HeaderValue::from_static("application/json"));
        head.insert("X-CSRFToken", HeaderValue::from_static("3vQuv67jvdushlvF624wOcLfKBN4NqzScVKRcg9yfPBYvEpisma3FQ1mIKx4FVbf"));
        Some(head)
    }
}