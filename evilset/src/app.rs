use std::time::Duration;

use eframe::{egui, epi};
use egui::{Color32, FontId, RichText};
// use egui::{containers::Frame, Stroke};

const TIMES_TO_DISPLAY: usize = 15;

enum GameState {
    Menu,
    Set,
    EvilSet,
    UltraSet,
    EvilUltraSet,
}

struct Times {
    set_times: Vec<Duration>,
    evilset_times: Vec<Duration>,
    ultraset_times: Vec<Duration>,
    evilultraset_times: Vec<Duration>,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct EvilSetApp {
    // Tracks whether the app is in the selection menu or one of the four games.
    game_state: GameState,
    times: Times,
}

impl Default for EvilSetApp {
    fn default() -> Self {
        Self {
            game_state: GameState::Menu,
            times: Times {
                set_times: Vec::new(),
                evilset_times: Vec::new(),
                ultraset_times: Vec::new(),
                evilultraset_times: Vec::new(),
            },
        }
    }
}

impl epi::App for EvilSetApp {
    fn name(&self) -> &str {
        "Evil Set"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { game_state, times } = self;

        // let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
        //     let image = load_image(include_bytes!("../assets/sample.png")).unwrap();
        //     ctx.load_texture("sample", image)
        // });
        match game_state {
            &mut GameState::Menu => self.update_menu(ctx, frame),
            _ => todo!(),
        }
    }
}

impl EvilSetApp {
    /// Called whenever app is in the initial menu stage
    fn update_menu(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { game_state, times } = self;

        // let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
        //     let image = load_image(include_bytes!("../assets/sample.png")).unwrap();
        //     ctx.load_texture("sample", image)
        // });

        egui::SidePanel::right("side_panel")
            // .default_width(160.0)
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Best Times");
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Set")
                                .font(FontId::proportional(18.0))
                                .color(Color32::LIGHT_BLUE),
                        );
                        for time in times.set_times.iter().take(TIMES_TO_DISPLAY) {
                            ui.monospace(time.as_secs().to_string());
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Evil Set")
                                .font(FontId::proportional(18.0))
                                .color(Color32::LIGHT_RED),
                        );
                        for time in times.evilset_times.iter().take(TIMES_TO_DISPLAY) {
                            ui.monospace(time.as_secs().to_string());
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Ultra Set")
                                .font(FontId::proportional(18.0))
                                .color(Color32::LIGHT_BLUE),
                        );
                        for time in times.ultraset_times.iter().take(TIMES_TO_DISPLAY) {
                            ui.monospace(time.as_secs().to_string());
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Evil Ultra Set")
                                .font(FontId::proportional(18.0))
                                .color(Color32::LIGHT_RED),
                        );
                        for time in times.evilultraset_times.iter().take(TIMES_TO_DISPLAY) {
                            ui.monospace(time.as_secs().to_string());
                        }
                    });
                });

                // if ui
                //     .add(egui::ImageButton::new(texture, texture.size_vec2()))
                //     .clicked()
                // {
                //     // *value -= 1.0;
                // }
            });

        // let new_frame = Frame::default().stroke(Stroke::new(1.0, Color32::RED));

        // egui::CentralPanel::default()
        //     .frame(new_frame)
        //     .show(ctx, |ui| {

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.vertical_centered(|ui| {
                ui.heading(
                    RichText::new("Evil Set")
                        .font(FontId::proportional(28.0))
                        .color(Color32::LIGHT_RED),
                );
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(20.0);

            ui.vertical_centered(|ui| {
                if ui
                    .add(egui::Button::new(
                        RichText::new("          Set          ")
                            .font(FontId::proportional(23.0))
                            .color(Color32::LIGHT_BLUE),
                    ))
                    .clicked()
                {
                    *game_state = GameState::Set;
                    println!("Set selected");
                }

                if ui
                    .add(egui::Button::new(
                        RichText::new("      Evil Set       ")
                            .font(FontId::proportional(23.0))
                            .color(Color32::LIGHT_RED),
                    ))
                    .clicked()
                {
                    *game_state = GameState::EvilSet;
                    println!("Evil Set selected");
                }

                if ui
                    .add(egui::Button::new(
                        RichText::new("     Ultra Set     ")
                            .font(FontId::proportional(23.0))
                            .color(Color32::LIGHT_BLUE),
                    ))
                    .clicked()
                {
                    *game_state = GameState::UltraSet;
                    println!("Ultra Set selected");
                }

                if ui
                    .add(egui::Button::new(
                        RichText::new(" Evil Ultra Set  ")
                            .font(FontId::proportional(23.0))
                            .color(Color32::LIGHT_RED),
                    ))
                    .clicked()
                {
                    *game_state = GameState::EvilUltraSet;
                    println!("Evil Ultra Set selected");
                }
            })

            // ui.hyperlink("https://github.com/emilk/eframe_template");
            // ui.add(egui::github_link_file!(
            // "https://github.com/emilk/eframe_template/blob/master/",
            // "Source code."
            // ));
            // egui::warn_if_debug_build(ui);
        });
    }
}

fn load_image(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    use image::GenericImageView as _;
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
