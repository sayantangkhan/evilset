mod filling_nodes;

use filling_nodes::get_filling_node;
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
    Triangle,
    Rectangle,
    Pentagon,
}

fn fill_diamond(color: SetColor, filling: Filling, svg_root: &mut Node) {
    let defs_node = svg_root.first_child();
    dbg!(defs_node.as_ref().unwrap().first_child());
    dbg!(defs_node
        .as_ref()
        .unwrap()
        .first_child()
        .unwrap()
        .first_child());
}

pub fn render_diamond(pixmap_size: &ScreenSize, output_file: &Path) {
    let svg_data = include_bytes!("../assets/shapes/diamond.svg");

    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(svg_data, &opt.to_ref()).unwrap();

    // let svg_root = &mut rtree.root();
    // fill_diamond(SetColor::Blue, Filling::HorizontalDashed, svg_root);
    let mut path_node = rtree.root().first_child().unwrap().next_sibling().unwrap();

    {
        let mut node_value = path_node.borrow_mut();

        match &mut *node_value {
            NodeKind::Path(path) => {
                dbg!(&path);
                // path.fill = Some(Fill::from_paint(Paint::Color(Color::black())));
                // path.fill = Some(Fill::from_paint(Paint::Link("Checkerboard".to_string())));
                path.stroke.as_mut().unwrap().paint = Paint::Color(Color::white());
            }
            _ => (),
        }
    }

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &rtree,
        usvg::FitTo::Height(pixmap_size.height()),
        tiny_skia::Transform::from_scale(0.7, 0.7),
        pixmap.as_mut(),
    )
    .unwrap();
    resvg::render(
        &rtree,
        usvg::FitTo::Height(pixmap_size.height()),
        tiny_skia::Transform::from_translate(30.0, 30.0),
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.save_png(output_file).unwrap();
}

pub fn render_squiggle(pixmap_size: &ScreenSize, output_file: &Path) {
    let svg_data = include_bytes!("../assets/shapes/squiggle.svg");
    let filling_nodes = filling_nodes::generate_filling_nodes().unwrap();

    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(svg_data, &opt.to_ref()).unwrap();

    let mut defs_node = rtree.root().first_child().unwrap();

    // let horizontal_striped_node =
    // get_filling_node(Filling::HorizontalStriped, filling_nodes).unwrap();
    // defs_node.prepend(horizontal_striped_node);
    // let checkerboard_node = get_filling_node(Filling::Checkerboard, filling_nodes).unwrap();
    // defs_node.prepend(checkerboard_node);
    let vertical_striped_node = get_filling_node(Filling::VerticalStriped, filling_nodes).unwrap();
    defs_node.prepend(vertical_striped_node);

    let mut stroke_path = rtree.root().first_child().unwrap().next_sibling().unwrap();
    // let interior_path = stroke_path.next_sibling().unwrap();
    // dbg!(stroke_path);

    {
        let mut node_value = stroke_path.borrow_mut();

        match &mut *node_value {
            NodeKind::Path(path) => {
                // dbg!(&path);
                // path.fill = Some(Fill::from_paint(Paint::Color(Color::black())));
                path.fill = Some(Fill::from_paint(Paint::Link("pattern".to_string())));
                // path.stroke.as_mut().unwrap().paint = Paint::Color(Color::white());
            }
            _ => (),
        }
    }

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &rtree,
        usvg::FitTo::Height(pixmap_size.height()),
        tiny_skia::Transform::from_scale(0.7, 0.7),
        pixmap.as_mut(),
    )
    .unwrap();
    resvg::render(
        &rtree,
        usvg::FitTo::Height(pixmap_size.height()),
        tiny_skia::Transform::from_translate(30.0, 30.0),
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.save_png(output_file).unwrap();
}
