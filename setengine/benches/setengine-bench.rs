use criterion::{criterion_group, criterion_main, Criterion};
use setengine::*;

fn detect_set(n: usize) {
    let deck = Deck::new_standard_deck();
    let cards = &deck.cards[0..n];
    selection_contains_set(cards);
}
fn detect_ultraset(n: usize) {
    let deck = Deck::new_standard_deck();
    let cards = &deck.cards[0..n];
    selection_contains_ultraset(cards);
}

fn generalized_set_detection_benchmark(c: &mut Criterion) {
    c.bench_function("Detect set in 12 random cards", |b| {
        b.iter(|| detect_set(12))
    });

    c.bench_function("Detect set in 15 random cards", |b| {
        b.iter(|| detect_set(15))
    });

    c.bench_function("Detect set in 18 random cards", |b| {
        b.iter(|| detect_set(18))
    });

    c.bench_function("Detect set in 21 random cards", |b| {
        b.iter(|| detect_set(21))
    });

    c.bench_function("Detect ultraset in 12 random cards", |b| {
        b.iter(|| detect_ultraset(12))
    });

    c.bench_function("Detect ultraset in 15 random cards", |b| {
        b.iter(|| detect_ultraset(15))
    });

    c.bench_function("Detect ultraset in 18 random cards", |b| {
        b.iter(|| detect_ultraset(18))
    });

    c.bench_function("Detect ultraset in 21 random cards", |b| {
        b.iter(|| detect_ultraset(21))
    });
}

criterion_group!(benches, generalized_set_detection_benchmark);
criterion_main!(benches);
