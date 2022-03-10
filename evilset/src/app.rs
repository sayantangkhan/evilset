use std::{collections::HashMap, time::Duration};

use cardgen::{render_card, CardVisualAttr};
use eframe::{
    egui::{self, Button, ImageButton, Layout},
    epaint::TextureHandle,
    epi,
};
use egui::{Color32, FontId, RichText};
use setengine::{ActiveDeck, CardCoordinates, Deck, SetGame, UltrasetGame};

use crate::themes::AppTheme;

const TIMES_TO_DISPLAY: usize = 15;

enum GameState {
    Menu,
    Set,
    EvilSet,
    UltraSet,
    EvilUltraSet,
    ShowDeck,
}

struct Times {
    set_times: Vec<Duration>,
    evilset_times: Vec<Duration>,
    ultraset_times: Vec<Duration>,
    evilultraset_times: Vec<Duration>,
}

enum GameDeck {
    Set(ActiveDeck<SetGame>),
    UltraSet(ActiveDeck<UltrasetGame>),
}

struct ActiveGameData {
    active_deck: GameDeck,
    card_textures: HashMap<(CardCoordinates, CardVisualAttr), TextureHandle>,
    selected: bool,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct EvilSetApp {
    // Tracks whether the app is in the selection menu or one of the four games.
    game_state: GameState,
    // Keeps a track of TIMES_TO_DISPLAY best times in each category
    times: Times,
    // Deck and card textures
    game_data: Option<ActiveGameData>,
    // Cached SVG data
    filling_nodes: Option<cardgen::FillingNodes>,
    // Theme
    theme: AppTheme,
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
            game_data: None,
            filling_nodes: None,
            theme: AppTheme::Light,
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

        self.filling_nodes = cardgen::generate_filling_nodes();
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
        let Self {
            game_state,
            times,
            game_data,
            filling_nodes,
            theme,
        } = self;

        ctx.set_visuals(crate::themes::generate_base_theme(&self.theme));
        // *ui.visuals_mut() = match self.theme {
        //     AppTheme::Dark => egui::Visuals::dark(),
        //     AppTheme::Light => egui::Visuals::light(),
        // };
        match game_state {
            &mut GameState::Menu => self.update_menu(ctx, frame),
            &mut GameState::Set => self.play_set(ctx, frame),
            &mut GameState::ShowDeck => self.show_deck(ctx, frame),
            _ => todo!(),
        }
    }
}

impl EvilSetApp {
    /// Called whenever app is in the initial menu stage
    fn update_menu(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            game_state,
            times,
            game_data: _,
            filling_nodes,
            theme: _,
        } = self;

        egui::SidePanel::right("side_panel")
            // .default_width(160.0)
            .resizable(true)
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
                                .color(crate::themes::thematic_blue(&self.theme)),
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
                                .color(crate::themes::thematic_red(&self.theme)),
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
                                .color(crate::themes::thematic_blue(&self.theme)),
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
                                .color(crate::themes::thematic_red(&self.theme)),
                        );
                        for time in times.evilultraset_times.iter().take(TIMES_TO_DISPLAY) {
                            ui.monospace(time.as_secs().to_string());
                        }
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    *ui.visuals_mut() = match self.theme {
                        AppTheme::Dark => egui::Visuals::dark(),
                        AppTheme::Light => egui::Visuals::light(),
                    };
                    let theme_btn = ui.add(Button::new({
                        match self.theme {
                            AppTheme::Dark => RichText::new("🌞").size(25.0),
                            AppTheme::Light => RichText::new("🌙").size(25.0),
                        }
                    }));
                    if theme_btn.clicked() {
                        self.theme.switch();
                    }
                });

                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("Evil Set")
                            .font(FontId::proportional(28.0))
                            .color(crate::themes::thematic_red(&self.theme)),
                    );
                });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(20.0);

            ui.vertical_centered(|ui| {
                if ui
                    .add(egui::Button::new(
                        RichText::new("          Set          ")
                            .font(FontId::proportional(23.0))
                            .color(crate::themes::thematic_blue(&self.theme)),
                    ))
                    .clicked()
                {
                    // Generate the images for a standard deck
                    let deck = Deck::new_standard_deck();
                    let active_set_deck: ActiveDeck<SetGame> = ActiveDeck::start_play(&deck);
                    let active_deck = GameDeck::Set(active_set_deck);

                    let mut card_textures = HashMap::new();

                    for (coord, visattr) in deck.cards {
                        let pixmap = render_card(visattr, filling_nodes.as_ref().unwrap());

                        let image = egui::ColorImage::from_rgba_unmultiplied(
                            [pixmap.width() as _, pixmap.height() as _],
                            pixmap.data(),
                        );

                        let texture = ctx.load_texture(format!("{:?}", visattr), image);
                        card_textures.insert((coord, visattr), texture);
                    }

                    self.game_data = Some(ActiveGameData {
                        active_deck,
                        card_textures,
                        selected: false,
                    });
                    *game_state = GameState::Set;
                    println!("Set selected");
                }

                if ui
                    .add(egui::Button::new(
                        RichText::new("      Evil Set       ")
                            .font(FontId::proportional(23.0))
                            .color(crate::themes::thematic_red(&self.theme)),
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
                            .color(crate::themes::thematic_blue(&self.theme)),
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
                            .color(crate::themes::thematic_red(&self.theme)),
                    ))
                    .clicked()
                {
                    *game_state = GameState::EvilUltraSet;
                    println!("Evil Ultra Set selected");
                }
            })
        });
    }

    fn play_set(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            game_state: _,
            times: _,
            game_data,
            filling_nodes: _,
            theme: _,
        } = self;

        egui::SidePanel::right("side_panel")
            // .default_width(160.0)
            .resizable(false)
            .show(ctx, |ui| {});

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.vertical_centered(|ui| {
                ui.heading(
                    RichText::new("Set")
                        .font(FontId::proportional(28.0))
                        .color(crate::themes::thematic_blue(&self.theme)),
                );
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                *ui.visuals_mut() = crate::themes::generate_card_theme(&self.theme);
                let ActiveGameData {
                    active_deck,
                    card_textures,
                    selected,
                } = game_data.as_mut().unwrap();
                if let GameDeck::Set(active_deck) = active_deck {
                    let available_width = ui.available_width();

                    for card in &active_deck.in_play {
                        let texture = card_textures.get(card).unwrap();
                        let button = ui.add(
                            ImageButton::new(texture, scale_card(available_width))
                                .selected(*selected),
                        );
                        if button.clicked() {
                            dbg!(card);
                            *selected = !(*selected);
                        }
                    }
                } else {
                    unreachable!()
                }
            })
        });
    }

    fn show_deck(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        todo!()
    }
}

fn scale_card(frame_width: f32) -> (f32, f32) {
    let new_width = frame_width / 4.0;
    let new_height = (cardgen::CARDHEIGHT as f32) * (new_width / (cardgen::CARDWIDTH as f32));
    (new_width, new_height)
}
