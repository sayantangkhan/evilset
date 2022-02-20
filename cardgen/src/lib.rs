mod colorandfill;
mod filling_nodes;

use colorandfill::color_shape;
use filling_nodes::generate_filling_nodes;
use resvg::ScreenSize;
use std::path::Path;
use usvg::{Color, Fill, Node, NodeKind, Paint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Filling {
    Hollow,
    Solid,
    HorizontalStriped,
    Wavy,
    Checkerboard,
    VerticalStriped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SetColor {
    Purple,
    Red,
    Green,
    Black,
    Yellow,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SetNum {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Diamond,
    Pill,
    Squiggle,
    Heart,
    Spade,
    Club,
}

pub fn render_diamond(pixmap_size: &ScreenSize, output_file: &Path) {
    let filling_nodes = generate_filling_nodes().unwrap();

    let colored_diamond = color_shape(
        SetColor::Purple,
        Filling::HorizontalStriped,
        Shape::Club,
        &filling_nodes,
    );

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &colored_diamond,
        usvg::FitTo::Height(pixmap_size.height()),
        tiny_skia::Transform::from_scale(0.7, 0.7),
        pixmap.as_mut(),
    )
    .unwrap();

    pixmap.save_png(output_file).unwrap();
}

pub fn render_squiggle(pixmap_size: &ScreenSize, output_file: &Path) {
    let filling_nodes = generate_filling_nodes().unwrap();

    let colored_squiggle = color_shape(
        SetColor::Yellow,
        Filling::Wavy,
        Shape::Squiggle,
        &filling_nodes,
    );

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &colored_squiggle,
        usvg::FitTo::Height(pixmap_size.height()),
        tiny_skia::Transform::from_scale(0.7, 0.7),
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.save_png(output_file).unwrap();
}
