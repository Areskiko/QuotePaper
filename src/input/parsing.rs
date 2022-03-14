use std::io::Read;

use log::debug;
use reqwest::{blocking, header::{HeaderMap, ACCEPT, HeaderValue}};


use crate::Settings;

use super::structs::{QuoteSource, Quote, SourceType};

pub fn get<T: QuoteSource>(settings: &Settings, source: &mut T) -> Quote {
    match settings.source_type() {
        SourceType::FILE => {
            read(settings, source)
        },
        SourceType::URL => {
            request(&reqwest::blocking::Client::new(), settings, source)
        },
    }
}

pub fn request<T: QuoteSource>(client: &blocking::Client, settings: &Settings, source: &mut T) -> Quote {
    let mut head = HeaderMap::new();
    head.insert(ACCEPT, HeaderValue::from_static("application/json"));
    head.insert("X-CSRFToken", HeaderValue::from_static("3vQuv67jvdushlvF624wOcLfKBN4NqzScVKRcg9yfPBYvEpisma3FQ1mIKx4FVbf"));
    let res = client.get(settings.location())
        .headers(head)
        .send();
    if let Ok(res) = res {
        let body = res.text().unwrap();
        let quote: Quote = source.from_source(&body).get_quote();
        quote
    } else {
        Quote::empty()
    }
}

pub fn read<T: QuoteSource>(settings: &Settings, source: &mut T) -> Quote {
    let mut file = std::fs::File::open(settings.location()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    source.from_source(&contents);
    source.get_quote()
}