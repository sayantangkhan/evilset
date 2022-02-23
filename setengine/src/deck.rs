use cardgen::{generate_random_attributes, generate_standard_attributes, CardVisualAttr};
use itertools::Itertools;
use rand::prelude::*;
use std::marker::PhantomData;

use crate::{is_set, is_ultraset, CardCoordinates};

pub struct Deck {
    pub cards: Vec<(CardCoordinates, CardVisualAttr)>,
}

// Empty types to indicate whether Set or Ultraset is being played
pub trait GeneralizedSetGame {
    fn contains_generalized_set(cards_in_play: &[(CardCoordinates, CardVisualAttr)]) -> bool;
    // Not doing the efficient thing here until necessary
}
pub enum SetGame {}
impl GeneralizedSetGame for SetGame {
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

pub struct ActiveDeck<T> {
    in_play: Vec<(CardCoordinates, CardVisualAttr)>,
    in_deck: Vec<(CardCoordinates, CardVisualAttr)>,
    _game_type: PhantomData<T>,
}

impl<T: GeneralizedSetGame> ActiveDeck<T> {}

impl Deck {
    pub fn new_standard_deck() -> Self {
        let standard_attributes = generate_standard_attributes();
        let mut cards = Vec::new();

        for num in 0..3 {
            for color in 0..3 {
                for shape in 0..3 {
                    for filling in 0..3 {
                        let coordinates = CardCoordinates::new(num, color, shape, filling);

                        let actual_num = standard_attributes.numbers[num as usize];
                        let actual_color = standard_attributes.colors[color as usize];
                        let actual_shape = standard_attributes.shapes[shape as usize];
                        let actual_filling = standard_attributes.fillings[filling as usize];

                        let visual_attr = CardVisualAttr {
                            num: actual_num,
                            color: actual_color,
                            shape: actual_shape,
                            filling: actual_filling,
                        };

                        cards.push((coordinates, visual_attr))
                    }
                }
            }
        }

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Self { cards }
    }

    pub fn new_random_deck() -> Self {
        let random_attributes = generate_random_attributes();
        let mut cards = Vec::new();

        for num in 0..3 {
            for color in 0..3 {
                for shape in 0..3 {
                    for filling in 0..3 {
                        let coordinates = CardCoordinates::new(num, color, shape, filling);

                        let actual_num = random_attributes.numbers[num as usize];
                        let actual_color = random_attributes.colors[color as usize];
                        let actual_shape = random_attributes.shapes[shape as usize];
                        let actual_filling = random_attributes.fillings[filling as usize];

                        let visual_attr = CardVisualAttr {
                            num: actual_num,
                            color: actual_color,
                            shape: actual_shape,
                            filling: actual_filling,
                        };

                        cards.push((coordinates, visual_attr))
                    }
                }
            }
        }

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Self { cards }
    }
}
