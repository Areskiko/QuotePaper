//! Modules for Quote sources

use crate::input::structs::QuoteSource;

mod random_red_rising;

pub enum Source {
    RR,
}

pub fn use_source(source_type: Source) {
    let source:
    match source_type {
        Source::RR => random_red_rising::RRQuote::new(),
    }
}