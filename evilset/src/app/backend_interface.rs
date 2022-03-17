use std::collections::HashSet;

use super::GameDeck;

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
