#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(rust_2018_idioms)]
#![warn(clippy::all)]

mod cards;
mod deck;

pub use cards::{is_set, is_ultraset, CardCoordinates, GeneralizedSetGame, SetGame, UltrasetGame};
pub use deck::{ActiveDeck, Deck, PlayResponse};

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
