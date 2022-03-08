use cardgen::CardVisualAttr;
use itertools::Itertools;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardCoordinates {
    num: u8,
    color: u8,
    shape: u8,
    filling: u8,
}

// Empty types to indicate whether Set or Ultraset is being played
pub trait GeneralizedSetGame {
    const NUM_CARDS: usize;

    fn is_generalized_set(cards_picked: &[(CardCoordinates, CardVisualAttr)]) -> bool;
    fn contains_generalized_set(cards_in_play: &[(CardCoordinates, CardVisualAttr)]) -> bool;
}
pub enum SetGame {}
impl GeneralizedSetGame for SetGame {
    const NUM_CARDS: usize = 3;

    fn is_generalized_set(cards_picked: &[(CardCoordinates, CardVisualAttr)]) -> bool {
        let card1 = (cards_picked[0]).0;
        let card2 = (cards_picked[1]).0;
        let card3 = (cards_picked[2]).0;

        is_set(card1, card2, card3)
    }

    fn contains_generalized_set(cards_in_play: &[(CardCoordinates, CardVisualAttr)]) -> bool {
        for triple in cards_in_play.iter().combinations(3) {
            let card1 = (*triple[0]).0;
            let card2 = (*triple[1]).0;
            let card3 = (*triple[2]).0;
            if is_set(card1, card2, card3) {
                return true;
            }
        }
        false
    }
}

pub enum UltrasetGame {}
impl GeneralizedSetGame for UltrasetGame {
    const NUM_CARDS: usize = 4;

    fn is_generalized_set(cards_picked: &[(CardCoordinates, CardVisualAttr)]) -> bool {
        let card1 = (cards_picked[0]).0;
        let card2 = (cards_picked[1]).0;
        let card3 = (cards_picked[2]).0;
        let card4 = (cards_picked[3]).0;

        is_ultraset(card1, card2, card3, card4)
    }

    fn contains_generalized_set(cards_in_play: &[(CardCoordinates, CardVisualAttr)]) -> bool {
        for quadruple in cards_in_play.iter().combinations(4) {
            let card1 = (*quadruple[0]).0;
            let card2 = (*quadruple[1]).0;
            let card3 = (*quadruple[2]).0;
            let card4 = (*quadruple[3]).0;
            if is_ultraset(card1, card2, card3, card4) {
                return true;
            }
        }
        false
    }
}

pub fn is_set(card1: CardCoordinates, card2: CardCoordinates, card3: CardCoordinates) -> bool {
    (card1 + card2 + card3) == CardCoordinates::new(0, 0, 0, 0)
}

fn complete_set(card1: CardCoordinates, card2: CardCoordinates) -> CardCoordinates {
    card1 + card1 - card2
}

pub fn is_ultraset(
    card1: CardCoordinates,
    card2: CardCoordinates,
    card3: CardCoordinates,
    card4: CardCoordinates,
) -> bool {
    if complete_set(card1, card2) == complete_set(card3, card4) {
        true
    } else if complete_set(card1, card3) == complete_set(card2, card4) {
        true
    } else if complete_set(card1, card4) == complete_set(card2, card3) {
        true
    } else {
        false
    }
}

impl CardCoordinates {
    pub fn new(num: u8, color: u8, shape: u8, filling: u8) -> Self {
        Self {
            num: num % 3,
            color: color % 3,
            shape: shape % 3,
            filling: filling % 3,
        }
    }
}

impl Add for CardCoordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self {
            num: lhs_num,
            color: lhs_color,
            shape: lhs_shape,
            filling: lhs_filling,
        } = self;
        let Self {
            num: rhs_num,
            color: rhs_color,
            shape: rhs_shape,
            filling: rhs_filling,
        } = rhs;

        Self::new(
            lhs_num + rhs_num,
            lhs_color + rhs_color,
            lhs_shape + rhs_shape,
            lhs_filling + rhs_filling,
        )
    }
}

impl Sub for CardCoordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let Self {
            num: lhs_num,
            color: lhs_color,
            shape: lhs_shape,
            filling: lhs_filling,
        } = self;
        let Self {
            num: rhs_num,
            color: rhs_color,
            shape: rhs_shape,
            filling: rhs_filling,
        } = rhs;

        Self::new(
            lhs_num + rhs_num + rhs_num,
            lhs_color + rhs_color + rhs_color,
            lhs_shape + rhs_shape + rhs_shape,
            lhs_filling + rhs_filling + rhs_filling,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::complete_set;
    use crate::{is_set, CardCoordinates};
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for CardCoordinates {
        fn arbitrary(g: &mut Gen) -> Self {
            CardCoordinates::new(
                u8::arbitrary(g),
                u8::arbitrary(g),
                u8::arbitrary(g),
                u8::arbitrary(g),
            )
        }
    }

    #[quickcheck]
    fn complete_set_gives_a_set(card1: CardCoordinates, card2: CardCoordinates) -> bool {
        let card3 = complete_set(card1, card2);
        is_set(card1, card2, card3)
    }
}
