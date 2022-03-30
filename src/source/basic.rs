//! Basic Quote format

use serde::{Serialize, Deserialize};

use crate::input::structs::{QuoteSource, Quote, QuoteSourceBuilder};


pub struct Wrapper {}

impl QuoteSourceBuilder for Wrapper {
    fn build() -> Box<dyn QuoteSource> {
        Box::new(BQuote {
            text: "".to_string(),
            author: "".to_string(),
        })
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BQuote {
    text: String,
    author: String,
}

impl QuoteSource for BQuote {

    fn get_quote(&self) -> Quote {
        Quote::new(self.text.clone(), self.author.clone())
    }
    fn from_source(&mut self, source: &str) {
        let quote_src: BQuote = serde_json::from_str(source).unwrap();
        self.text = quote_src.text;
        self.author = quote_src.author;
    }

    fn headers(&self) -> Option<reqwest::header::HeaderMap> {
        None
    }
}

