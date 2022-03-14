use log::debug;
use serde::{Serialize, Deserialize};

use crate::input::structs::{QuoteSource, Quote};

#[derive(Serialize, Deserialize, Debug)]
pub struct RRQuote {
    count: i32,
    results: Vec<Results>
}

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

impl RRQuote {
    pub fn new() -> RRQuote {
        RRQuote {
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
        }
    }
}

impl QuoteSource for RRQuote {
    fn get_quote(&self) -> Quote {
        Quote::new(self.results[0].text.clone(), self.results[0].character.clone())
    }
    fn from_source(&self, source: &str) -> RRQuote {
        serde_json::from_str(source).unwrap()
    }
}