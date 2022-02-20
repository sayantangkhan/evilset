use cardgen::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn render_random_deck() {
    let filling_nodes = generate_filling_nodes().unwrap();

    let attributes = generate_random_attributes();
    for num in attributes.numbers {
        for color in attributes.colors {
            for shape in attributes.shapes {
                for filling in attributes.fillings {
                    let card = CardVisualAttr {
                        num,
                        color,
                        shape,
                        filling,
                    };
                    let _pixmap = render_card(card, &filling_nodes);
                }
            }
        }
    }
}

fn render_standard_deck() {
    let filling_nodes = generate_filling_nodes().unwrap();

    let attributes = generate_standard_attributes();
    for num in attributes.numbers {
        for color in attributes.colors {
            for shape in attributes.shapes {
                for filling in attributes.fillings {
                    let card = CardVisualAttr {
                        num,
                        color,
                        shape,
                        filling,
                    };
                    let _pixmap = render_card(card, &filling_nodes);
                }
            }
        }
    }
}

fn random_deck_generation_benchmark(c: &mut Criterion) {
    c.bench_function("Random deck generation benchmark", |b| {
        b.iter(|| render_random_deck())
    });
}

fn standard_deck_generation_benchmark(c: &mut Criterion) {
    c.bench_function("Standard deck generation benchmark", |b| {
        b.iter(|| render_standard_deck())
    });
}

criterion_group!(
    benches,
    random_deck_generation_benchmark,
    standard_deck_generation_benchmark
);
criterion_main!(benches);
