#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(rust_2018_idioms)]
#![warn(clippy::all)]

mod cards;
mod deck;

pub use cards::{
    selection_contains_set, selection_contains_ultraset, selection_is_set, selection_is_ultraset,
    CardCoordinates,
};
pub use deck::{Deck, GameDeck, PlayResponse};

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
