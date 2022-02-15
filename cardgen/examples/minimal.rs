fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage:\n\tminimal <in-svg> <out-png>");
        return;
    }

    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    opt.resources_dir = std::fs::canonicalize(&args[1]).ok().and_then(|p| p.parent().map(|p| p.to_path_buf()));
    opt.fontdb.load_system_fonts();

    let svg_data = std::fs::read(&args[1]).unwrap();
    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap();

    dbg!(&rtree.root());

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render_node(&rtree, &rtree.root(), usvg::FitTo::Original, tiny_skia::Transform::from_scale(0.5, 0.5), pixmap.as_mut()).unwrap();
    resvg::render_node(&rtree, &rtree.root(), usvg::FitTo::Original, tiny_skia::Transform::from_translate(30.0, 30.0), pixmap.as_mut()).unwrap();
    pixmap.save_png(&args[2]).unwrap();
}
