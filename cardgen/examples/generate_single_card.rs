use cardgen::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\tminimal <out-png>");
        return;
    }

    let filling_nodes = generate_filling_nodes().unwrap();
    let card = Card {
        num: SetNum::Six,
        color: SetColor::Purple,
        shape: Shape::Squiggle,
        filling: Filling::Solid,
    };

    let pixmap = render_card(card, &filling_nodes);

    pixmap.save_png(&args[1]).unwrap();
}
