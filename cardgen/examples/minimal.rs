use std::path::Path;

use cardgen::*;
use resvg::ScreenSize;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\tminimal <out-png>");
        return;
    }

    let pixmap_size = ScreenSize::new(1000, 500).unwrap();

    render_diamond(&pixmap_size, &Path::new(&args[1]));
    // render_squiggle(&pixmap_size, &Path::new(&args[1]));
}
