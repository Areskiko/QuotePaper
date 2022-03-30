//! Modules for Quote sources

use serde::{Serialize, Deserialize};

use crate::input::structs::QuoteSource;
use crate::input::structs::QuoteSourceBuilder;

mod random_red_rising;
mod basic;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Source {
    Basic,
    RR,
}

pub fn use_source(source_type: Source) -> Box<dyn QuoteSource> {
    match source_type {
        Source::Basic => basic::Wrapper::build(),
        Source::RR => random_red_rising::Wrapper::build(),
    }
}