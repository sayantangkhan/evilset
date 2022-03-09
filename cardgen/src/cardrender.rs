use resvg::ScreenSize;
use tiny_skia::Pixmap;

use crate::{colorandfill::color_shape, filling_nodes::FillingNodes, CardVisualAttr};

pub const WIDTH: u32 = 292;
pub const HEIGHT: u32 = 174;

pub fn render_card(card: CardVisualAttr, filling_nodes: &FillingNodes) -> Pixmap {
    let pixmap_size = ScreenSize::new(WIDTH, HEIGHT).unwrap();

    let single_element = color_shape(card.color, card.filling, card.shape, filling_nodes);

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    let mut bg_color = tiny_skia::Color::WHITE;
    bg_color.apply_opacity(0.5);
    pixmap.fill(bg_color);

    match card.num {
        crate::SetNum::One => {
            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.7, 0.7)
                    .post_translate((WIDTH as f32) * 0.38, (HEIGHT as f32) * 0.13),
                pixmap.as_mut(),
            )
            .unwrap();
        }
        crate::SetNum::Two => {
            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.7, 0.7)
                    .post_translate((WIDTH as f32) * 0.23, (HEIGHT as f32) * 0.13),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.7, 0.7)
                    .post_translate((WIDTH as f32) * 0.53, (HEIGHT as f32) * 0.13),
                pixmap.as_mut(),
            )
            .unwrap();
        }
        crate::SetNum::Three => {
            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.7, 0.7)
                    .post_translate((WIDTH as f32) * 0.08, (HEIGHT as f32) * 0.13),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.7, 0.7)
                    .post_translate((WIDTH as f32) * 0.38, (HEIGHT as f32) * 0.13),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.7, 0.7)
                    .post_translate((WIDTH as f32) * 0.68, (HEIGHT as f32) * 0.13),
                pixmap.as_mut(),
            )
            .unwrap();
        }
        crate::SetNum::Four => {
            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.55, 0.55)
                    .post_translate((WIDTH as f32) * 0.10, (HEIGHT as f32) * 0.10),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.55, 0.55)
                    .post_translate((WIDTH as f32) * 0.50, (HEIGHT as f32) * 0.10),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.55, 0.55)
                    .post_translate((WIDTH as f32) * 0.30, (HEIGHT as f32) * 0.36),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.55, 0.55)
                    .post_translate((WIDTH as f32) * 0.70, (HEIGHT as f32) * 0.36),
                pixmap.as_mut(),
            )
            .unwrap();
        }
        crate::SetNum::Five => {
            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.45, 0.45)
                    .post_translate((WIDTH as f32) * 0.03, (HEIGHT as f32) * 0.10),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.45, 0.45)
                    .post_translate((WIDTH as f32) * 0.43, (HEIGHT as f32) * 0.10),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.45, 0.45)
                    .post_translate((WIDTH as f32) * 0.23, (HEIGHT as f32) * 0.36),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.45, 0.45)
                    .post_translate((WIDTH as f32) * 0.63, (HEIGHT as f32) * 0.36),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.45, 0.45)
                    .post_translate((WIDTH as f32) * 0.83, (HEIGHT as f32) * 0.10),
                pixmap.as_mut(),
            )
            .unwrap();
        }
        crate::SetNum::Six => {
            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.4, 0.4)
                    .post_translate((WIDTH as f32) * 0.05, (HEIGHT as f32) * 0.15),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.4, 0.4)
                    .post_translate((WIDTH as f32) * 0.20, (HEIGHT as f32) * 0.45),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.4, 0.4)
                    .post_translate((WIDTH as f32) * 0.35, (HEIGHT as f32) * 0.15),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.4, 0.4)
                    .post_translate((WIDTH as f32) * 0.50, (HEIGHT as f32) * 0.45),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.4, 0.4)
                    .post_translate((WIDTH as f32) * 0.65, (HEIGHT as f32) * 0.15),
                pixmap.as_mut(),
            )
            .unwrap();

            resvg::render(
                &single_element,
                usvg::FitTo::Height(pixmap_size.height()),
                tiny_skia::Transform::from_scale(0.4, 0.4)
                    .post_translate((WIDTH as f32) * 0.80, (HEIGHT as f32) * 0.45),
                pixmap.as_mut(),
            )
            .unwrap();
        }
    }

    pixmap
}
