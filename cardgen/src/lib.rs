use resvg::ScreenSize;
use std::{ops::Index, path::Path};
use usvg::{Color, Fill, Node, NodeKind, Paint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Filling {
    Hollow,
    Solid,
    HorizontalDashed,
    VerticalDashed,
    Checkerboard,
    PolkaDots,
}

struct FillingNodes {
    array: [Option<Node>; 6],
}

impl Index<Filling> for FillingNodes {
    type Output = Option<Node>;

    fn index(&self, index: Filling) -> &Self::Output {
        match index {
            Filling::Hollow => &self.array[0],
            Filling::Solid => &self.array[1],
            Filling::HorizontalDashed => &self.array[2],
            Filling::VerticalDashed => &self.array[3],
            Filling::Checkerboard => &self.array[4],
            Filling::PolkaDots => &self.array[5],
        }
    }
}

fn generate_filling_nodes() -> FillingNodes {
    let mut array_vec = Vec::new();
    let opt = usvg::Options::default();

    // Hollow pattern. Does not need a def node
    array_vec.push(None);
    // Solid pattern. Does not need a def node
    array_vec.push(None);

    // Loading horizontal stripe pattern
    let horizontal_striped_svg = include_bytes!("../assets/diamond-striped.svg");
    let rtree = usvg::Tree::from_data(horizontal_striped_svg, &opt.to_ref()).unwrap();
    let horizontal_striped_node = rtree.root().first_child().unwrap().first_child().unwrap().make_deep_copy();
    array_vec.push(Some(horizontal_striped_node));

    // Remove this later
    array_vec.push(None);
    array_vec.push(None);
    array_vec.push(None);

    FillingNodes { array: array_vec.try_into().unwrap()}
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
    let svg_data = include_bytes!("../assets/diamond-striped.svg");

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
    let svg_data = include_bytes!("../assets/squiggle-striped.svg");
    let filling_nodes = generate_filling_nodes();

    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(svg_data, &opt.to_ref()).unwrap();

    // Adding stripes

    let mut defs_node = rtree.root().first_child().unwrap();
    let horizontal_striped_node = filling_nodes[Filling::HorizontalDashed].as_ref().unwrap().make_deep_copy();
    defs_node.prepend(horizontal_striped_node);

    // Remove later

    let stroke_path = rtree.root().first_child().unwrap().next_sibling().unwrap();
    let interior_path = stroke_path.next_sibling().unwrap();
    dbg!(stroke_path);

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
