use setengine::*;

fn main() {
    let deck = Deck::new_random_deck();
    for p in deck.cards {
        println!("{:?}", p);
    }
}
