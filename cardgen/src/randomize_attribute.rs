use crate::{Attributes, Filling, SetColor, SetNum, Shape};
use rand::prelude::*;

/// # Panics
///
/// Will not actually panic
#[must_use]
pub fn generate_standard_attributes() -> Attributes {
    let indices = [0, 1, 2];

    let set_nums: [SetNum; 3] = (0..3)
        .into_iter()
        .map(|i| SetNum::index(indices[i]).unwrap())
        .collect::<Vec<SetNum>>()
        .try_into()
        .unwrap();

    let set_colors: [SetColor; 3] = (0..3)
        .into_iter()
        .map(|i| SetColor::index(indices[i]).unwrap())
        .collect::<Vec<SetColor>>()
        .try_into()
        .unwrap();

    let shapes: [Shape; 3] = (0..3)
        .into_iter()
        .map(|i| Shape::index(indices[i]).unwrap())
        .collect::<Vec<Shape>>()
        .try_into()
        .unwrap();

    let fillings: [Filling; 3] = (0..3)
        .into_iter()
        .map(|i| Filling::index(indices[i]).unwrap())
        .collect::<Vec<Filling>>()
        .try_into()
        .unwrap();

    Attributes {
        numbers: set_nums,
        colors: set_colors,
        shapes,
        fillings,
    }
}

/// # Panics
///
/// Will not actually panic
#[must_use]
pub fn generate_random_attributes() -> Attributes {
    let mut rng = thread_rng();
    let mut indices = [0, 1, 2, 3, 4, 5];

    // Randomizing SetNum
    indices.shuffle(&mut rng);
    let set_nums: [SetNum; 3] = (0..3)
        .into_iter()
        .map(|i| SetNum::index(indices[i]).unwrap())
        .collect::<Vec<SetNum>>()
        .try_into()
        .unwrap();

    // Randomizing SetColor
    indices.shuffle(&mut rng);
    let set_colors: [SetColor; 3] = (0..3)
        .into_iter()
        .map(|i| SetColor::index(indices[i]).unwrap())
        .collect::<Vec<SetColor>>()
        .try_into()
        .unwrap();

    // Randomizing Shape
    indices.shuffle(&mut rng);
    let shapes: [Shape; 3] = (0..3)
        .into_iter()
        .map(|i| Shape::index(indices[i]).unwrap())
        .collect::<Vec<Shape>>()
        .try_into()
        .unwrap();

    // Randomizing Filling
    indices.shuffle(&mut rng);
    let fillings: [Filling; 3] = (0..3)
        .into_iter()
        .map(|i| Filling::index(indices[i]).unwrap())
        .collect::<Vec<Filling>>()
        .try_into()
        .unwrap();

    Attributes {
        numbers: set_nums,
        colors: set_colors,
        shapes,
        fillings,
    }
}
