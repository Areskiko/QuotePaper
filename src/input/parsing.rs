//! Parsing of quotes

use std::io::Read;

use reqwest::{blocking};


use crate::Settings;
use super::structs::{QuoteSource, Quote, SourceType};

/// Get a quote as described by the settings, using the source as a format
pub fn get(settings: &Settings, source: Box<dyn QuoteSource>) -> Quote {
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
fn request(client: &blocking::Client, settings: &Settings, mut source: Box<dyn QuoteSource>) -> Quote {
    let rb = client.get(settings.location());
    let res;
    if let Some(head) = source.headers() {
        res = rb.headers(head).send();
    } else {
        res = rb.send();
    }
    if let Ok(res) = res {
        let body = res.text().unwrap();
        source.from_source(&body);
        let quote: Quote = source.get_quote();
        quote
    } else {
        Quote::empty()
    }
}

/// Read a quote from a file
fn read(settings: &Settings, mut source: Box<dyn QuoteSource>) -> Quote {
    let mut file = std::fs::File::open(shellexpand::tilde(settings.location()).to_string()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    source.from_source(&contents);
    source.get_quote()
}