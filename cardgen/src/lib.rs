use resvg::ScreenSize;
use std::path::Path;
use usvg::{Color, Fill, NodeKind, Paint};

enum Filling {
    Hollow,
    Solid,
    HorizontalDashed,
    VerticalDashed,
    Checkerboard,
    PolkaDots,
}

enum SetColor {}

fn generate_filling_data(color: SetColor, filling: Filling) -> Fill {
    todo!()
}

pub fn render_diamond(pixmap_size: &ScreenSize, output_file: &Path) {
    let svg_data = include_bytes!("../assets/diamond.svg");

    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(svg_data, &opt.to_ref()).unwrap();

    let mut path_node = rtree.root().first_child().unwrap().next_sibling().unwrap();

    {
        let mut node_value = path_node.borrow_mut();

        match &mut *node_value {
            NodeKind::Path(path) => {
                dbg!(&path.fill);
                path.fill = Some(Fill::from_paint(Paint::Color(Color::black())));
                path.stroke.as_mut().unwrap().paint = Paint::Color(Color::white());
            }
            _ => (),
        }
    }

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render_node(
        &rtree,
        &path_node,
        usvg::FitTo::Height(pixmap_size.height()),
        tiny_skia::Transform::from_scale(0.7, 0.7),
        pixmap.as_mut(),
    )
    .unwrap();
    resvg::render_node(
        &rtree,
        &path_node,
        usvg::FitTo::Height(pixmap_size.height()),
        tiny_skia::Transform::from_translate(30.0, 30.0),
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.save_png(output_file).unwrap();
}

pub fn render_squiggle(pixmap_size: &ScreenSize, output_file: &Path) {
    let svg_data = include_bytes!("../assets/squiggle-striped.svg");

    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(svg_data, &opt.to_ref()).unwrap();

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
