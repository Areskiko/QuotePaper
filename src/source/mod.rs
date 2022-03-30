//! Modules for Quote sources

use crate::input::structs::QuoteSource;
use crate::input::structs::QuoteSourceBuilder;

mod random_red_rising;

pub enum Source {
    RR,
}

pub fn use_source(source_type: Source) -> Box<dyn QuoteSource> {
    match source_type {
        Source::RR => random_red_rising::Wrapper::build(),
    }
}