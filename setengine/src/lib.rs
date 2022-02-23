mod cards;
mod deck;

pub use cards::{is_set, is_ultraset, CardCoordinates};
pub use deck::Deck;
pub use deck::{GeneralizedSetGame, SetGame, UltrasetGame};

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
