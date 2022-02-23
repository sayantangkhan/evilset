use setengine::*;

fn main() {
    let deck = Deck::new_standard_deck();
    for p in deck.cards {
        println!("{:?}", p);
    }
}
