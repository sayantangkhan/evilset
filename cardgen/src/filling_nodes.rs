use crate::Filling;
use usvg::Node;

/// Array containing the SVG trees of the six filling patterns
pub struct FillingNodes {
    array: [Option<Node>; 6],
}

/// Generates the FillingNodes from `include!`ed svg data
pub fn generate_filling_nodes() -> Option<FillingNodes> {
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
    let diagonalstriped_svg = include_bytes!("../assets/fillings/diagonalstriped.svg");
    let rtree = usvg::Tree::from_data(diagonalstriped_svg, &opt.to_ref()).ok()?;
    let wavy_node = rtree.root().first_child()?.first_child()?.make_deep_copy();
    array_vec.push(Some(wavy_node));

    Some(FillingNodes {
        array: array_vec.try_into().unwrap(),
    })
}

pub(crate) fn get_filling_node(filling: Filling, nodes: &FillingNodes) -> Option<Node> {
    let filling_node_index = match filling {
        Filling::Hollow => 0,
        Filling::Solid => 1,
        Filling::HorizontalStriped => 2,
        Filling::Checkerboard => 3,
        Filling::VerticalStriped => 4,
        Filling::DiagonalStriped => 5,
    };

    match filling {
        Filling::Hollow | Filling::Solid => None,
        Filling::HorizontalStriped | Filling::VerticalStriped | Filling::DiagonalStriped => {
            let filling_node = nodes.array[filling_node_index].as_ref().unwrap();
            let filling_node_child = filling_node.first_child().unwrap();

            let return_node_data = filling_node.borrow().clone();
            let return_node_child_data = filling_node_child.borrow().clone();

            let mut return_node = Node::new(return_node_data);
            let return_node_child = Node::new(return_node_child_data);
            return_node.append(return_node_child);

            Some(return_node)
        }
        Filling::Checkerboard => {
            let filling_node = nodes.array[filling_node_index].as_ref().unwrap();
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
    }
}
