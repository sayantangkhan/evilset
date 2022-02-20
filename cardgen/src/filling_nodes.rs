use crate::Filling;
// use std::ops::{Index, IndexMut};
use usvg::Node;

pub struct FillingNodes {
    array: [Option<Node>; 6],
}

pub(crate) fn generate_filling_nodes() -> Option<FillingNodes> {
    let mut array_vec = Vec::new();
    let opt = usvg::Options::default();

    // Hollow pattern. Does not need a def node
    array_vec.push(None);
    // Solid pattern. Does not need a def node
    array_vec.push(None);

    // Loading horizontal stripe pattern
    let horizontal_striped_svg = include_bytes!("../assets/fillings/striped.svg");
    let rtree = usvg::Tree::from_data(horizontal_striped_svg, &opt.to_ref()).ok()?;
    let horizontal_striped_node = rtree.root().first_child()?.first_child()?.make_deep_copy();
    array_vec.push(Some(horizontal_striped_node));

    // Loading checkerboard pattern
    let checkerboard_svg = include_bytes!("../assets/fillings/checkerboard.svg");
    let rtree = usvg::Tree::from_data(checkerboard_svg, &opt.to_ref()).ok()?;
    let checkerboard_node = rtree.root().first_child()?.first_child()?.make_deep_copy();
    array_vec.push(Some(checkerboard_node));

    // Loading vertical stripe pattern
    let vertical_striped_svg = include_bytes!("../assets/fillings/verticalstriped.svg");
    let rtree = usvg::Tree::from_data(vertical_striped_svg, &opt.to_ref()).ok()?;
    let vertical_striped_node = rtree.root().first_child()?.first_child()?.make_deep_copy();
    array_vec.push(Some(vertical_striped_node));

    // Loading wavy pattern
    let wavy_svg = include_bytes!("../assets/fillings/wavy.svg");
    let rtree = usvg::Tree::from_data(wavy_svg, &opt.to_ref()).ok()?;
    let wavy_node = rtree.root().first_child()?.first_child()?.make_deep_copy();
    array_vec.push(Some(wavy_node));

    Some(FillingNodes {
        array: array_vec.try_into().unwrap(),
    })
}

pub(crate) fn get_filling_node(filling: Filling, nodes: &FillingNodes) -> Option<Node> {
    match filling {
        Filling::Hollow => None,
        Filling::Solid => None,
        Filling::HorizontalStriped => {
            let filling_node = nodes.array[2].as_ref().unwrap();
            let filling_node_child = filling_node.first_child().unwrap();

            let return_node_data = filling_node.borrow().clone();
            let return_node_child_data = filling_node_child.borrow().clone();

            let mut return_node = Node::new(return_node_data);
            let return_node_child = Node::new(return_node_child_data);
            return_node.append(return_node_child);

            Some(return_node)
        }
        Filling::Checkerboard => {
            let filling_node = nodes.array[3].as_ref().unwrap();
            let first_child = filling_node.first_child().unwrap();
            let second_child = first_child.next_sibling().unwrap();

            let filling_node_data = filling_node.borrow().clone();
            let first_child_data = first_child.borrow().clone();
            let second_child_data = second_child.borrow().clone();

            let mut return_node = Node::new(filling_node_data);
            let return_node_first_child = Node::new(first_child_data);
            let return_node_second_child = Node::new(second_child_data);
            return_node.append(return_node_first_child);
            return_node.append(return_node_second_child);

            Some(return_node)
        }
        Filling::VerticalStriped => {
            let filling_node = nodes.array[4].as_ref().unwrap();
            let filling_node_child = filling_node.first_child().unwrap();

            let return_node_data = filling_node.borrow().clone();
            let return_node_child_data = filling_node_child.borrow().clone();

            let mut return_node = Node::new(return_node_data);
            let return_node_child = Node::new(return_node_child_data);
            return_node.append(return_node_child);

            Some(return_node)
        }
        Filling::Wavy => {
            let filling_node = nodes.array[5].as_ref().unwrap();
            let filling_node_child = filling_node.first_child().unwrap();

            let return_node_data = filling_node.borrow().clone();
            let return_node_child_data = filling_node_child.borrow().clone();

            let mut return_node = Node::new(return_node_data);
            let return_node_child = Node::new(return_node_child_data);
            return_node.append(return_node_child);

            Some(return_node)
        }
    }
}
