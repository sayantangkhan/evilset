use crate::filling_nodes::{get_filling_node, FillingNodes};
use crate::{Filling, SetColor, Shape};
use usvg::{Color, Fill, NodeKind, Paint, Tree};

impl Into<Color> for SetColor {
    fn into(self) -> Color {
        match self {
            SetColor::Purple => Color::new_rgb(128, 0, 128),
            SetColor::Red => Color::new_rgb(255, 1, 1),
            SetColor::Green => Color::new_rgb(0, 128, 2),
            SetColor::Black => Color::new_rgb(0, 0, 0),
            SetColor::Yellow => Color::new_rgb(255, 215, 0),
            SetColor::Blue => Color::new_rgb(0, 255, 255),
        }
    }
}

pub(crate) fn color_shape(
    setcolor: SetColor,
    filling: Filling,
    shape: Shape,
    filling_nodes: &FillingNodes,
) -> Tree {
    match shape {
        Shape::Squiggle => color_squiggle(setcolor, filling, filling_nodes),
        Shape::Diamond => color_diamond(setcolor, filling, filling_nodes),
        _ => {
            todo!()
        }
    }
}

fn color_squiggle(setcolor: SetColor, filling: Filling, filling_nodes: &FillingNodes) -> Tree {
    let svg_data = include_bytes!("../assets/shapes/squiggle.svg");

    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(svg_data, &opt.to_ref()).unwrap();

    let mut defs_node = rtree.root().first_child().unwrap();
    let filling_node = get_filling_node(filling, filling_nodes);

    let mut interior_path = rtree.root().first_child().unwrap().next_sibling().unwrap();
    let mut stroke_path = interior_path.next_sibling().unwrap();

    // Color boundary first
    {
        let mut node_value = stroke_path.borrow_mut();

        match &mut *node_value {
            NodeKind::Path(path) => {
                path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
            }
            _ => (),
        }
    }

    // Color and fill interior
    {
        let mut node_value = interior_path.borrow_mut();
        match &mut *node_value {
            NodeKind::Path(path) => match filling {
                Filling::Hollow => {
                    path.fill = None;
                }
                Filling::Solid => {
                    path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
                }
                _ => {
                    path.fill = Some(Fill::from_paint(Paint::Link("pattern".to_string())));
                }
            },
            _ => (),
        }
    }

    // Populate filling pattern
    match filling {
        Filling::HorizontalStriped | Filling::VerticalStriped | Filling::Wavy => {
            let filling_node = filling_node.unwrap();
            let mut filling_node_child = filling_node.first_child().unwrap();
            let mut node_value = filling_node_child.borrow_mut();
            match &mut *node_value {
                NodeKind::Path(path) => {
                    path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
                }
                _ => (),
            }

            defs_node.prepend(filling_node);
        }
        Filling::Checkerboard => {
            let filling_node = filling_node.unwrap();
            let mut filling_node_first_child = filling_node.first_child().unwrap();
            let mut filling_node_second_child =
                filling_node.first_child().unwrap().next_sibling().unwrap();

            let mut first_child_value = filling_node_first_child.borrow_mut();
            match &mut *first_child_value {
                NodeKind::Path(path) => {
                    path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
                }
                _ => (),
            }

            let mut second_child_value = filling_node_second_child.borrow_mut();
            match &mut *second_child_value {
                NodeKind::Path(path) => {
                    path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
                }
                _ => (),
            }

            defs_node.prepend(filling_node);
        }
        _ => (),
    }

    return rtree;
}

fn color_diamond(setcolor: SetColor, filling: Filling, filling_nodes: &FillingNodes) -> Tree {
    let svg_data = include_bytes!("../assets/shapes/diamond.svg");

    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(svg_data, &opt.to_ref()).unwrap();

    let mut defs_node = rtree.root().first_child().unwrap();
    let filling_node = get_filling_node(filling, filling_nodes);

    let mut interior_path = rtree.root().first_child().unwrap().next_sibling().unwrap();
    let mut stroke_path = interior_path.next_sibling().unwrap();

    // Color boundary first
    {
        let mut node_value = stroke_path.borrow_mut();

        match &mut *node_value {
            NodeKind::Path(path) => {
                path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
            }
            _ => (),
        }
    }

    // Color and fill interior
    {
        let mut node_value = interior_path.borrow_mut();
        match &mut *node_value {
            NodeKind::Path(path) => match filling {
                Filling::Hollow => {
                    path.fill = None;
                }
                Filling::Solid => {
                    path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
                }
                _ => {
                    path.fill = Some(Fill::from_paint(Paint::Link("pattern".to_string())));
                }
            },
            _ => (),
        }
    }

    // Populate filling pattern
    match filling {
        Filling::HorizontalStriped | Filling::VerticalStriped | Filling::Wavy => {
            let filling_node = filling_node.unwrap();
            let mut filling_node_child = filling_node.first_child().unwrap();
            let mut node_value = filling_node_child.borrow_mut();
            match &mut *node_value {
                NodeKind::Path(path) => {
                    path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
                }
                _ => (),
            }

            defs_node.prepend(filling_node);
        }
        Filling::Checkerboard => {
            let filling_node = filling_node.unwrap();
            let mut filling_node_first_child = filling_node.first_child().unwrap();
            let mut filling_node_second_child =
                filling_node.first_child().unwrap().next_sibling().unwrap();

            let mut first_child_value = filling_node_first_child.borrow_mut();
            match &mut *first_child_value {
                NodeKind::Path(path) => {
                    path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
                }
                _ => (),
            }

            let mut second_child_value = filling_node_second_child.borrow_mut();
            match &mut *second_child_value {
                NodeKind::Path(path) => {
                    path.fill = Some(Fill::from_paint(Paint::Color(setcolor.into())));
                }
                _ => (),
            }

            defs_node.prepend(filling_node);
        }
        _ => (),
    }

    return rtree;
}
