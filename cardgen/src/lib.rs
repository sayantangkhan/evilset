mod cardrender;
mod colorandfill;
mod filling_nodes;
mod randomize_attribute;

pub use cardrender::render_card;
pub use filling_nodes::{generate_filling_nodes, FillingNodes};
pub use randomize_attribute::{generate_random_attributes, generate_standard_attributes};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardVisualAttr {
    pub num: SetNum,
    pub color: SetColor,
    pub shape: Shape,
    pub filling: Filling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Filling {
    Hollow,
    Solid,
    HorizontalStriped,
    Wavy,
    Checkerboard,
    VerticalStriped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetColor {
    Purple,
    Red,
    Green,
    Black,
    Yellow,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetNum {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shape {
    Diamond,
    Pill,
    Squiggle,
    Heart,
    Spade,
    Club,
}

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
            3 => Some(Filling::Wavy),
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
            4 => Some(SetColor::Yellow),
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
