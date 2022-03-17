#[cfg(not(target_arch = "wasm32"))]
use std::thread::sleep;

use super::GameDeck;
use cardgen::CardVisualAttr;
use setengine::{CardCoordinates, GeneralizedSetGame, PlayResponse, SetGame, UltrasetGame};
use std::collections::HashSet;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
fn sleep(duration: Duration) {
    use instant::Instant;
    let start = Instant::now();
    let mut current = Instant::now();
    while current - start < duration {
        current = Instant::now();
    }
}

pub(super) fn select_index(
    selected_index: usize,
    active_deck: &GameDeck,
    selected_cards: &mut HashSet<usize>,
) {
    let in_play = match &active_deck {
        GameDeck::Set(ad) => &ad.in_play,
        GameDeck::UltraSet(ad) => &ad.in_play,
    };

    let max_selected_size = match &active_deck {
        GameDeck::Set(_) => 3,
        GameDeck::UltraSet(_) => 4,
    };

    if selected_index < in_play.len() {
        if selected_cards.len() == max_selected_size && !selected_cards.contains(&selected_index) {
            return ();
        }

        if selected_cards.contains(&selected_index) {
            selected_cards.remove(&selected_index);
        } else {
            selected_cards.insert(selected_index);
        }
    }
}

pub(super) fn get_hint(active_deck: &GameDeck) -> Vec<usize> {
    let mut generalized_set_indices = Vec::new();

    generalized_set_indices
}

pub(super) fn evaluate_selection(game_data: &mut super::ActiveGameData) {
    let super::ActiveGameData {
        active_deck,
        card_textures: _,
        selected,
        game_started: _,
        prev_frame,
    } = game_data;

    let num_selections = match active_deck {
        GameDeck::Set(_) => 3,
        GameDeck::UltraSet(_) => 4,
    };

    if selected.len() != num_selections {
        return ();
    }

    if prev_frame.is_some() {
        sleep(Duration::from_millis(200));
    }

    match prev_frame {
        Some(PlayResponse::GameOver) => {
            println!("Game over");
        }
        Some(PlayResponse::ValidPlay) => {
            let selected_indices: Vec<usize> = selected.iter().map(|p| *p).collect();
            selected.clear();

            match active_deck {
                GameDeck::Set(ad) => {
                    if let Some(PlayResponse::GameOver) = ad.play_selection(&selected_indices) {
                        *prev_frame = Some(PlayResponse::GameOver);
                        return ();
                    }
                }
                GameDeck::UltraSet(ad) => {
                    if let Some(PlayResponse::GameOver) = ad.play_selection(&selected_indices) {
                        *prev_frame = Some(PlayResponse::GameOver);
                        return ();
                    }
                }
            }

            *prev_frame = None;
        }
        Some(PlayResponse::InvalidPlay) => {
            selected.clear();
            *prev_frame = None;
        }
        None => match active_deck {
            GameDeck::Set(ad) => {
                let selected_cards: Vec<(CardCoordinates, CardVisualAttr)> =
                    selected.iter().map(|p| ad.in_play[*p]).collect();

                if SetGame::is_generalized_set(&selected_cards) {
                    *prev_frame = Some(PlayResponse::ValidPlay);
                } else {
                    *prev_frame = Some(PlayResponse::InvalidPlay);
                }
            }
            GameDeck::UltraSet(ad) => {
                let selected_cards: Vec<(CardCoordinates, CardVisualAttr)> =
                    selected.iter().map(|p| ad.in_play[*p]).collect();

                if UltrasetGame::is_generalized_set(&selected_cards) {
                    *prev_frame = Some(PlayResponse::ValidPlay);
                } else {
                    *prev_frame = Some(PlayResponse::InvalidPlay);
                }
            }
        },
    }
}
