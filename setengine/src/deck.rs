use crate::{
    cards::{selection_contains_set, selection_contains_ultraset},
    selection_is_set, selection_is_ultraset,
};
use cardgen::{generate_random_attributes, generate_standard_attributes, CardVisualAttr};
use itertools::Itertools;
use rand::prelude::*;

use crate::CardCoordinates;

/// A shuffled deck
#[derive(Clone)]
pub struct Deck {
    pub cards: Vec<(CardCoordinates, CardVisualAttr)>,
}

/// A deck from which cards have been dealt out of
#[derive(Clone)]
pub struct ActiveDeck {
    in_play: Vec<(CardCoordinates, CardVisualAttr)>,
    in_deck: Vec<(CardCoordinates, CardVisualAttr)>,
}

/// Enum wrapping `ActiveDeck` marking whether Set or UltraSet is being played
#[derive(Clone)]
pub enum GameDeck {
    Set(ActiveDeck),
    UltraSet(ActiveDeck),
}

/// The three possible responses to playing a triple/quadruple.
#[derive(Debug)]
pub enum PlayResponse {
    InvalidPlay,
    ValidPlay,
    GameOver,
}

impl GameDeck {
    /// Returns how many cards need to be selected, depending upon the game
    pub fn selection_size(&self) -> usize {
        match *self {
            GameDeck::Set(_) => 3,
            GameDeck::UltraSet(_) => 4,
        }
    }

    /// Deals out cards, ensuring there is always a set.
    pub fn start_set_play(deck: &Deck) -> Self {
        let mut initial_cards = 12;
        while !selection_contains_set(&deck.cards[0..initial_cards]) {
            initial_cards += 3;
        }
        let in_play = deck.cards[..initial_cards].to_vec();
        let in_deck = deck.cards[initial_cards..].to_vec();

        GameDeck::Set(ActiveDeck { in_play, in_deck })
    }

    /// Deals out cards, ensuring there is always an ultraset.
    pub fn start_ultraset_play(deck: &Deck) -> Self {
        let mut initial_cards = 12;
        while !selection_contains_ultraset(&deck.cards[0..initial_cards]) {
            initial_cards += 3;
        }
        let in_play = deck.cards[..initial_cards].to_vec();
        let in_deck = deck.cards[initial_cards..].to_vec();

        GameDeck::UltraSet(ActiveDeck { in_play, in_deck })
    }

    /// Returns indices of 2/3 cards that form set/ultraset after completing with one more card
    pub fn get_hint(&self) -> Vec<usize> {
        match *self {
            Self::Set(_) => self.get_set_hint(),
            Self::UltraSet(_) => self.get_ultraset_hint(),
        }
    }

    /// Returns indices of two cards that complete to a set in the cards currently in play
    fn get_set_hint(&self) -> Vec<usize> {
        for triple in self.in_play().iter().enumerate().combinations(3) {
            let card1 = (triple[0]).1 .0;
            let card2 = (triple[1]).1 .0;
            let card3 = (triple[2]).1 .0;
            if crate::cards::is_set(card1, card2, card3) {
                return vec![(triple[0]).0, (triple[1]).0];
            }
        }
        unreachable!()
    }

    /// Returns indices of three cards that complete to an ultraset in the cards currently in play
    fn get_ultraset_hint(&self) -> Vec<usize> {
        for quadruple in self.in_play().iter().enumerate().combinations(4) {
            let card1 = (quadruple[0]).1 .0;
            let card2 = (quadruple[1]).1 .0;
            let card3 = (quadruple[2]).1 .0;
            let card4 = (quadruple[3]).1 .0;
            if crate::cards::is_ultraset(card1, card2, card3, card4) {
                return vec![(quadruple[0]).0, (quadruple[1]).0, (quadruple[2]).0];
            }
        }
        unreachable!()
    }

    /// Returns a slice of active cards
    pub fn in_play(&self) -> &[(CardCoordinates, CardVisualAttr)] {
        match self {
            GameDeck::Set(ad) => &ad.in_play,
            GameDeck::UltraSet(ad) => &ad.in_play,
        }
    }

    /// Returns a mutable reference to the Vec of active cards
    pub fn in_play_mut(&mut self) -> &mut Vec<(CardCoordinates, CardVisualAttr)> {
        match self {
            GameDeck::Set(ad) => &mut ad.in_play,
            GameDeck::UltraSet(ad) => &mut ad.in_play,
        }
    }

    /// Returns a slice of cards still in the deck
    pub fn in_deck(&self) -> &[(CardCoordinates, CardVisualAttr)] {
        match self {
            GameDeck::Set(ad) => &ad.in_deck,
            GameDeck::UltraSet(ad) => &ad.in_deck,
        }
    }

    /// Returns a mutable reference to the Vec of cards in deck
    pub fn in_deck_mut(&mut self) -> &mut Vec<(CardCoordinates, CardVisualAttr)> {
        match self {
            GameDeck::Set(ad) => &mut ad.in_deck,
            GameDeck::UltraSet(ad) => &mut ad.in_deck,
        }
    }

    /// Plays the cards with the selected indices from the `in_play` buffer, and deals out new cards. Panics if selection is not the right length.
    pub fn play_selection(&mut self, mut selection: Vec<usize>) -> PlayResponse {
        selection.sort_by(|a, b| b.cmp(a));

        let mut selected_cards = Vec::new();
        for index in &selection {
            selected_cards.push(self.in_play()[*index]);
        }

        // Case when selection is set or ultraset
        match &self {
            GameDeck::Set(_) => {
                if selection_is_set(&selected_cards) {
                    if self.in_deck().len() >= 3 {
                        // Enough cards in deck to replace
                        if self.in_play().len() <= 12 {
                            for index in &selection {
                                self.in_play_mut()[*index] = self.in_deck_mut().pop().unwrap();
                            }
                        } else {
                            // The case when there are 15 or more cards
                            for index in &selection {
                                self.in_play_mut().remove(*index);
                            }
                        }

                        // Add more cards until in_play has set
                        while !selection_contains_set(self.in_play()) {
                            if self.in_deck().is_empty() {
                                return PlayResponse::GameOver;
                            }
                            for _ in 0..3 {
                                if let Some(card) = self.in_deck_mut().pop() {
                                    self.in_play_mut().push(card);
                                }
                            }
                        }

                        return PlayResponse::ValidPlay;
                    } else {
                        // Not enough cards to replace
                        for index in selection {
                            self.in_play_mut().remove(index);
                        }
                        while !selection_contains_set(self.in_play()) {
                            if self.in_deck().is_empty() {
                                return PlayResponse::GameOver;
                            }
                            for _ in 0..3 {
                                if let Some(card) = self.in_deck_mut().pop() {
                                    self.in_play_mut().push(card);
                                }
                            }
                        }
                        return PlayResponse::ValidPlay;
                    }
                }
            }
            GameDeck::UltraSet(_) => {
                if selection_is_ultraset(&selected_cards) {
                    if self.in_deck().len() >= 4 {
                        // Enough cards in deck to replace
                        if self.in_play().len() <= 12 {
                            for index in &selection {
                                self.in_play_mut()[*index] = self.in_deck_mut().pop().unwrap();
                            }
                        } else {
                            // The case when there are more than 12 cards
                            for index in &selection {
                                self.in_play_mut().remove(*index);
                            }
                        }

                        // Add more cards until in_play has set
                        while !selection_contains_set(self.in_play()) {
                            if self.in_deck().is_empty() {
                                return PlayResponse::GameOver;
                            }
                            for _ in 0..4 {
                                if let Some(card) = self.in_deck_mut().pop() {
                                    self.in_play_mut().push(card);
                                }
                            }
                        }

                        return PlayResponse::ValidPlay;
                    } else {
                        // Not enough cards to replace
                        for index in selection {
                            self.in_play_mut().remove(index);
                        }
                        while !selection_contains_set(self.in_play()) {
                            if self.in_deck().is_empty() {
                                return PlayResponse::GameOver;
                            }
                            for _ in 0..4 {
                                if let Some(card) = self.in_deck_mut().pop() {
                                    self.in_play_mut().push(card);
                                }
                            }
                        }
                        return PlayResponse::ValidPlay;
                    }
                }
            }
        }

        PlayResponse::InvalidPlay
    }
}

impl Deck {
    /// Creates a deck with the standard attributes and shuffles it.
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

    /// Creates a deck with random attributes and shuffles it.
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
