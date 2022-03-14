use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub text: String,
    pub author: String,
}

impl Quote {
    pub fn new(text: String, author: String) -> Quote {
        Quote {
            text,
            author,
        }
    }
    pub fn empty() -> Quote {
        Quote {
            text: "".to_string(),
            author: "".to_string(),
        }
    }
}

pub trait QuoteSource {
    fn get_quote(&self) -> Quote;
    fn from_source(&self, source: &str) -> Self;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SourceType {
    FILE,
    URL,
}

pub enum SourceNumber {
    SINGLE,
    MULTIPLE,
}
