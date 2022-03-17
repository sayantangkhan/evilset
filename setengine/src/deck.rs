use cardgen::{generate_random_attributes, generate_standard_attributes, CardVisualAttr};
use rand::prelude::*;
use std::marker::PhantomData;

use crate::{CardCoordinates, GeneralizedSetGame};

pub struct Deck {
    pub cards: Vec<(CardCoordinates, CardVisualAttr)>,
}

#[derive(Clone)]
pub struct ActiveDeck<T> {
    pub in_play: Vec<(CardCoordinates, CardVisualAttr)>,
    pub in_deck: Vec<(CardCoordinates, CardVisualAttr)>,
    _game_type: PhantomData<T>,
}

pub enum PlayResponse {
    InvalidPlay,
    ValidPlay,
    GameOver,
}

impl<T: GeneralizedSetGame> ActiveDeck<T> {
    pub fn start_play(deck: &Deck) -> Self {
        let mut initial_cards = 12;
        while !T::contains_generalized_set(&deck.cards[0..initial_cards]) {
            initial_cards += 3;
        }
        let in_play = deck.cards[..initial_cards].to_vec();
        let in_deck = deck.cards[initial_cards..].to_vec();

        Self {
            in_play,
            in_deck,
            _game_type: PhantomData::default(),
        }
    }

    pub fn play_selection(&mut self, selection: &[usize]) -> Option<PlayResponse> {
        if selection.len() != T::NUM_CARDS {
            None
        } else {
            let mut selected_cards = Vec::new();
            for index in selection {
                selected_cards.push(*self.in_play.get(*index)?);
            }
            // TODO: Check removal code carefully
            if T::is_generalized_set(&selected_cards) {
                if self.in_deck.len() >= T::NUM_CARDS {
                    // Enough cards in deck to replace
                    if self.in_play.len() <= 12 {
                        // Needs replacement
                        for index in selection {
                            self.in_play[*index] = self.in_deck.pop().unwrap();
                        }
                    } else {
                        for index in selection {
                            self.in_play.remove(*index);
                        }
                    }

                    // Add more cards until in_play has generalized set
                    while !T::contains_generalized_set(&self.in_play) {
                        if self.in_deck.is_empty() {
                            return Some(PlayResponse::GameOver);
                        }
                        for _ in 0..T::NUM_CARDS {
                            if let Some(card) = self.in_deck.pop() {
                                self.in_play.push(card);
                            }
                        }
                    }
                    Some(PlayResponse::ValidPlay)
                } else {
                    // Not enough cards to replace
                    for index in selection {
                        self.in_play.remove(*index);
                    }
                    while !T::contains_generalized_set(&self.in_play) {
                        if self.in_deck.is_empty() {
                            return Some(PlayResponse::GameOver);
                        }
                        for _ in 0..T::NUM_CARDS {
                            if let Some(card) = self.in_deck.pop() {
                                self.in_play.push(card);
                            }
                        }
                    }
                    Some(PlayResponse::ValidPlay)
                }
            } else {
                Some(PlayResponse::InvalidPlay)
            }
        }
    }
}

impl Deck {
    #[must_use]
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

                        cards.push((coordinates, visual_attr));
                    }
                }
            }
        }

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Self { cards }
    }

    #[must_use]
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

                        cards.push((coordinates, visual_attr));
                    }
                }
            }
        }

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Self { cards }
    }
}
