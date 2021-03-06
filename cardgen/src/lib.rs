//! The module renders Set cards with various attributes to bitmaps

#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(rust_2018_idioms)]
#![warn(clippy::all)]

mod cardrender;
mod colorandfill;
mod filling_nodes;
mod randomize_attribute;

pub use cardrender::render_card;
pub use cardrender::HEIGHT as CARDHEIGHT;
pub use cardrender::WIDTH as CARDWIDTH;
pub use filling_nodes::{generate_filling_nodes, FillingNodes};
pub use randomize_attribute::{generate_random_attributes, generate_standard_attributes};

/// The four visual attributes a card can have
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardVisualAttr {
    /// Number of elements on card
    pub num: SetNum,
    /// Color of the elements on card
    pub color: SetColor,
    /// Shape of element on card
    pub shape: Shape,
    /// The filling pattern of the element on card
    pub filling: Filling,
}

/// The six filling patterns we can render. The first three are standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Filling {
    Hollow,
    Solid,
    HorizontalStriped,
    DiagonalStriped,
    Checkerboard,
    VerticalStriped,
}

/// The six colors we can render shapes in. The first three are standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetColor {
    Purple,
    Red,
    Green,
    Black,
    Brown,
    Blue,
}

/// The number of elements we can render on a card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetNum {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

/// The six different shapes we can render. The first three are standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shape {
    Diamond,
    Pill,
    Squiggle,
    Heart,
    Spade,
    Club,
}

/// The attributes we can vary in any given deck
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attributes {
    pub numbers: [SetNum; 3],
    pub colors: [SetColor; 3],
    pub shapes: [Shape; 3],
    pub fillings: [Filling; 3],
}

impl Filling {
    fn index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Filling::Hollow),
            1 => Some(Filling::Solid),
            2 => Some(Filling::HorizontalStriped),
            3 => Some(Filling::DiagonalStriped),
            4 => Some(Filling::Checkerboard),
            5 => Some(Filling::VerticalStriped),
            _ => None,
        }
    }
}

impl SetColor {
    fn index(index: usize) -> Option<Self> {
        match index {
            0 => Some(SetColor::Purple),
            1 => Some(SetColor::Red),
            2 => Some(SetColor::Green),
            3 => Some(SetColor::Black),
            4 => Some(SetColor::Brown),
            5 => Some(SetColor::Blue),
            _ => None,
        }
    }
}

impl SetNum {
    fn index(index: usize) -> Option<Self> {
        match index {
            0 => Some(SetNum::One),
            1 => Some(SetNum::Two),
            2 => Some(SetNum::Three),
            3 => Some(SetNum::Four),
            4 => Some(SetNum::Five),
            5 => Some(SetNum::Six),
            _ => None,
        }
    }
}

impl Shape {
    fn index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Shape::Diamond),
            1 => Some(Shape::Pill),
            2 => Some(Shape::Squiggle),
            3 => Some(Shape::Heart),
            4 => Some(Shape::Spade),
            5 => Some(Shape::Club),
            _ => None,
        }
    }
}
