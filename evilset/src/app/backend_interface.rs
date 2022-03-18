#[cfg(not(target_arch = "wasm32"))]
use std::thread::sleep;

use super::GameDeck;
use cardgen::CardVisualAttr;
use setengine::{selection_is_set, selection_is_ultraset, CardCoordinates, PlayResponse};
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
    let in_play = active_deck.in_play();
    let selection_size = active_deck.selection_size();

    if selected_index < in_play.len() {
        if selected_cards.len() == selection_size && !selected_cards.contains(&selected_index) {
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
    todo!()
}

pub(super) fn evaluate_selection(game_data: &mut super::ActiveGameData) {
    let super::ActiveGameData {
        active_deck,
        card_textures: _,
        selected,
        game_started: _,
        prev_frame,
    } = game_data;

    let num_selections = active_deck.selection_size();

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

            let result = active_deck.play_selection(selected_indices);
            dbg!(&result);
            if let PlayResponse::GameOver = result {
                *prev_frame = Some(PlayResponse::GameOver);
                return ();
            }
            *prev_frame = None;
        }
        Some(PlayResponse::InvalidPlay) => {
            selected.clear();
            *prev_frame = None;
        }
        None => {
            let selected_cards: Vec<(CardCoordinates, CardVisualAttr)> =
                selected.iter().map(|p| active_deck.in_play()[*p]).collect();

            let result = match active_deck {
                GameDeck::Set(_) => selection_is_set(&selected_cards),
                GameDeck::UltraSet(_) => selection_is_ultraset(&selected_cards),
            };

            if result {
                *prev_frame = Some(PlayResponse::ValidPlay);
            } else {
                *prev_frame = Some(PlayResponse::InvalidPlay);
            }
        }
    }
}
