use crate::filling_nodes::{get_filling_node, FillingNodes};
use crate::{Filling, SetColor, Shape};
use usvg::{Color, Fill, NodeKind, Paint, Tree};

impl From<SetColor> for Color {
    fn from(set_color: SetColor) -> Self {
        match set_color {
            SetColor::Purple => Color::new_rgb(128, 0, 128),
            SetColor::Red => Color::new_rgb(255, 1, 1),
            SetColor::Green => Color::new_rgb(0, 128, 2),
            SetColor::Black => Color::new_rgb(0, 0, 0),
            SetColor::Brown => Color::new_rgb(150, 75, 0),
            SetColor::Blue => Color::new_rgb(0, 0, 255),
        }
    }
}

pub(crate) fn color_shape(
    setcolor: SetColor,
    filling: Filling,
    shape: Shape,
    filling_nodes: &FillingNodes,
) -> Tree {
    let svg_data = match shape {
        Shape::Squiggle => include_bytes!("../assets/shapes/squiggle.svg").to_vec(),
        Shape::Diamond => include_bytes!("../assets/shapes/diamond.svg").to_vec(),
        Shape::Pill => include_bytes!("../assets/shapes/pill.svg").to_vec(),
        Shape::Heart => include_bytes!("../assets/shapes/heart.svg").to_vec(),
        Shape::Spade => include_bytes!("../assets/shapes/spade.svg").to_vec(),
        Shape::Club => include_bytes!("../assets/shapes/club.svg").to_vec(),
    };

    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap();

    let mut defs_node = rtree.root().first_child().unwrap();
    let filling_node = get_filling_node(filling, filling_nodes);

    let mut interior_path = rtree.root().first_child().unwrap().next_sibling().unwrap();
    let mut stroke_path = interior_path.next_sibling().unwrap();

    // Color boundary first
    {
        let mut node_value = stroke_path.borrow_mut();

        if let NodeKind::Path(path) = &mut *node_value {
            match shape {
                Shape::Squiggle | Shape::Diamond | Shape::Pill => {
                    path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
                }
                Shape::Heart | Shape::Spade | Shape::Club => {
                    path.stroke.as_mut().unwrap().paint = Paint::Color(setcolor.into());
                }
            }
        }
    }

    // Color and fill interior
    {
        let mut node_value = interior_path.borrow_mut();
        if let NodeKind::Path(path) = &mut *node_value {
            match filling {
                Filling::Hollow => {
                    path.fill = None;
                }
                Filling::Solid => {
                    path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
                }
                _ => {
                    path.fill = Some(Fill::from_paint(Paint::Link("pattern".to_string())));
                }
            }
        }
    }

    // Populate filling pattern
    match filling {
        Filling::HorizontalStriped | Filling::VerticalStriped | Filling::DiagonalStriped => {
            let filling_node = filling_node.unwrap();
            let mut filling_node_child = filling_node.first_child().unwrap();
            let mut node_value = filling_node_child.borrow_mut();
            if let NodeKind::Path(path) = &mut *node_value {
                path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
            }

            defs_node.prepend(filling_node);
        }
        Filling::Checkerboard => {
            let filling_node = filling_node.unwrap();
            let mut filling_node_first_child = filling_node.first_child().unwrap();
            let mut filling_node_second_child =
                filling_node.first_child().unwrap().next_sibling().unwrap();

            let mut first_child_value = filling_node_first_child.borrow_mut();
            if let NodeKind::Path(path) = &mut *first_child_value {
                path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
            }

            let mut second_child_value = filling_node_second_child.borrow_mut();
            if let NodeKind::Path(path) = &mut *second_child_value {
                path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
            }

            defs_node.prepend(filling_node);
        }
        _ => (),
    }

    rtree
}
