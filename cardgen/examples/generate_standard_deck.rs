use cardgen::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\tminimal <out-directory>");
        return;
    }

    let filling_nodes = generate_filling_nodes().unwrap();

    let attributes = generate_standard_attributes();
    for num in attributes.numbers {
        for color in attributes.colors {
            for shape in attributes.shapes {
                for filling in attributes.fillings {
                    let card = CardVisualAttr {
                        num,
                        color,
                        shape,
                        filling,
                    };
                    let pixmap = render_card(card, &filling_nodes);

                    let path = format!(
                        "{}/{:?}-{:?}-{:?}-{:?}.png",
                        &args[1], num, color, shape, filling
                    );
                    pixmap.save_png(&path).unwrap();
                }
            }
        }
    }
}
