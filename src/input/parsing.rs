//! Parsing of quotes

use std::io::Read;

use reqwest::{blocking};


use crate::Settings;
use super::structs::{QuoteSource, Quote, SourceType};

/// Get a quote as described by the settings, using the source as a format
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

/// Request a quote from the internet
fn request<T: QuoteSource>(client: &blocking::Client, settings: &Settings, source: &mut T) -> Quote {
    let rb = client.get(settings.location());
    let res;
    if let Some(head) = source.headers() {
        res = rb.headers(head).send();
    } else {
        res = rb.send();
    }
    if let Ok(res) = res {
        let body = res.text().unwrap();
        let quote: Quote = source.from_source(&body).get_quote();
        quote
    } else {
        Quote::empty()
    }
}

/// Read a quote from a file
fn read<T: QuoteSource>(settings: &Settings, source: &mut T) -> Quote {
    let mut file = std::fs::File::open(settings.location()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    source.from_source(&contents);
    source.get_quote()
}