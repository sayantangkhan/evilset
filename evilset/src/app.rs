#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all)]

use crate::themes::AppTheme;
use cardgen::{render_card, CardVisualAttr, FillingNodes};
use eframe::{
    egui::{Button, FontId, ImageButton, Layout, RichText},
    epaint::TextureHandle,
    epi,
};
use poll_promise::Promise;
use setengine::{ActiveDeck, CardCoordinates, Deck, SetGame, UltrasetGame};
use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

const TIMES_TO_DISPLAY: usize = 15;

struct PersistentGameData {
    // Light and dark themes
    theme: AppTheme,
    // Keeps a track of TIMES_TO_DISPLAY best times in each category
    times: Times,
}

struct Times {
    set_times: Vec<Duration>,
    evilset_times: Vec<Duration>,
    ultraset_times: Vec<Duration>,
    evilultraset_times: Vec<Duration>,
}

impl Default for PersistentGameData {
    fn default() -> Self {
        Self {
            theme: AppTheme::Light,
            times: Times {
                set_times: Vec::new(),
                evilset_times: Vec::new(),
                ultraset_times: Vec::new(),
                evilultraset_times: Vec::new(),
            },
        }
    }
}

enum GameState {
    Menu,
    Help,
    Set,
    EvilSet,
    UltraSet,
    EvilUltraSet,
}

#[derive(Clone)]
enum GameDeck {
    Set(ActiveDeck<SetGame>),
    UltraSet(ActiveDeck<UltrasetGame>),
}

struct ActiveGameData {
    active_deck: GameDeck,
    card_textures: Option<TextureMap>,
    selected: HashSet<usize>,
    game_started: Option<Instant>,
}

type TextureMap = HashMap<(CardCoordinates, CardVisualAttr), TextureHandle>;

#[derive(Default)]
struct RenderingPromises {
    standard_deck: Option<Promise<TextureMap>>,
    randomized_deck: Option<Promise<TextureMap>>,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct EvilSetApp {
    // Game data and preferences that can be loaded from a file
    persistent_data: PersistentGameData,
    // Tracks whether the app is in the selection menu or one of the four games.
    game_state: GameState,
    // State of currently active game
    game_data: Option<ActiveGameData>,
    // Background rendering promises
    background_rendering: RenderingPromises,
}

impl Default for EvilSetApp {
    fn default() -> Self {
        Self {
            persistent_data: PersistentGameData::default(),
            game_state: GameState::Menu,
            game_data: None,
            background_rendering: RenderingPromises::default(),
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
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        self.background_rendering.standard_deck = Some(standard_deck_texture_promise(ctx));
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
            persistent_data,
            game_state,
            game_data,
            background_rendering,
        } = self;

        ctx.set_visuals(crate::themes::generate_base_theme(&persistent_data.theme));

        match *game_state {
            GameState::Menu => self.update_menu(ctx, frame),
            GameState::Set => self.play_set(ctx, frame),
            // &mut GameState::ShowDeck => self.show_deck(ctx, frame),
            _ => todo!(),
        }
    }
}

impl EvilSetApp {
    /// Called whenever app is in the initial menu stage
    fn update_menu(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let Self {
            persistent_data,
            game_state,
            game_data,
            background_rendering,
        } = self;

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
                                .color(crate::themes::thematic_blue(&persistent_data.theme)),
                        );
                        for time in persistent_data
                            .times
                            .set_times
                            .iter()
                            .take(TIMES_TO_DISPLAY)
                        {
                            ui.monospace(time.as_secs().to_string());
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Evil Set")
                                .font(FontId::proportional(18.0))
                                .color(crate::themes::thematic_red(&persistent_data.theme)),
                        );
                        for time in persistent_data
                            .times
                            .evilset_times
                            .iter()
                            .take(TIMES_TO_DISPLAY)
                        {
                            ui.monospace(time.as_secs().to_string());
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Ultra Set")
                                .font(FontId::proportional(18.0))
                                .color(crate::themes::thematic_blue(&persistent_data.theme)),
                        );
                        for time in persistent_data
                            .times
                            .ultraset_times
                            .iter()
                            .take(TIMES_TO_DISPLAY)
                        {
                            ui.monospace(time.as_secs().to_string());
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Evil Ultra Set")
                                .font(FontId::proportional(18.0))
                                .color(crate::themes::thematic_red(&persistent_data.theme)),
                        );
                        for time in persistent_data
                            .times
                            .evilultraset_times
                            .iter()
                            .take(TIMES_TO_DISPLAY)
                        {
                            ui.monospace(time.as_secs().to_string());
                        }
                    });
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.1;
                        ui.label("Source code on ");
                        ui.hyperlink_to("Github", "https://github.com/sayantangkhan/evilset");
                        egui::warn_if_debug_build(ui);
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    *ui.visuals_mut() = match persistent_data.theme {
                        AppTheme::Dark => egui::Visuals::dark(),
                        AppTheme::Light => egui::Visuals::light(),
                    };
                    let theme_btn = ui.add(Button::new({
                        match persistent_data.theme {
                            AppTheme::Dark => RichText::new("🌞").size(25.0),
                            AppTheme::Light => RichText::new("🌙").size(25.0),
                        }
                    }));
                    if theme_btn.clicked() {
                        persistent_data.theme.switch();
                    }
                });

                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("Evil Set")
                            .font(FontId::proportional(28.0))
                            .color(crate::themes::thematic_red(&persistent_data.theme)),
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
                            .color(crate::themes::thematic_blue(&persistent_data.theme)),
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
                            .color(crate::themes::thematic_red(&persistent_data.theme)),
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
                            .color(crate::themes::thematic_blue(&persistent_data.theme)),
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
                            .color(crate::themes::thematic_red(&persistent_data.theme)),
                    ))
                    .clicked()
                {
                    *game_state = GameState::EvilUltraSet;
                    println!("Evil Ultra Set selected");
                }
            })
        });
    }

    fn play_set(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let Self {
            persistent_data,
            game_state,
            game_data,
            background_rendering,
        } = self;

        if game_data.is_none() {
            let rendering_promise = &background_rendering.standard_deck.as_ref().unwrap();
            match rendering_promise.ready() {
                None => {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.vertical_centered_justified(|ui| {
                            ui.label(
                                RichText::new("Rendering cards").font(FontId::proportional(18.0)),
                            );
                            ui.add(egui::Spinner::new()); // still loading
                        });
                    });
                }
                Some(card_textures) => {
                    let deck = Deck::new_standard_deck();
                    let active_set_deck: ActiveDeck<SetGame> = ActiveDeck::start_play(&deck);
                    let active_deck = GameDeck::Set(active_set_deck);

                    *game_data = Some(ActiveGameData {
                        active_deck,
                        card_textures: Some(card_textures.clone()),
                        selected: HashSet::new(),
                        game_started: Some(Instant::now()),
                    });
                }
            }
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's

                ui.horizontal(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            RichText::new("Set")
                                .font(FontId::proportional(28.0))
                                .color(crate::themes::thematic_blue(&persistent_data.theme)),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(), |ui| {
                        ui.label(
                            RichText::new(format!(
                                "⏱ {}",
                                standard_format(
                                    game_data.as_ref().unwrap().game_started.unwrap().elapsed()
                                )
                            ))
                            .font(FontId::proportional(28.0)),
                        );
                        ctx.request_repaint();
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(20.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    *ui.visuals_mut() = crate::themes::generate_card_theme(&persistent_data.theme);

                    let ActiveGameData {
                        active_deck,
                        card_textures,
                        selected,
                        game_started,
                    } = game_data.as_mut().unwrap();
                    if let GameDeck::Set(active_deck) = active_deck {
                        let available_width = ui.available_width();
                        let available_height = ui.available_height();

                        let card_textures = card_textures.as_ref().unwrap();

                        ui.columns(3, |columns| {
                            for (index, card) in active_deck.in_play.iter().enumerate() {
                                let texture = card_textures.get(card).unwrap();
                                let button = &mut columns[index % 3].add(
                                    ImageButton::new(
                                        texture,
                                        scale_card(available_width, available_height),
                                    ), // .selected(*selected),
                                );
                                if button.clicked() {
                                    dbg!(card);
                                    // *selected = !(*selected);
                                }
                            }
                        });
                    } else {
                        unreachable!()
                    }
                })
            });
        }
    }
}

// TODO: Also have a minimum height
fn scale_card(frame_width: f32, frame_height: f32) -> (f32, f32) {
    let scaling_with_width = {
        let new_width = frame_width / 4.0;
        let new_height = (cardgen::CARDHEIGHT as f32) * (new_width / (cardgen::CARDWIDTH as f32));
        (new_width, new_height)
    };

    let scaling_with_height = {
        let new_height = frame_height / 5.0;
        let new_width = (cardgen::CARDWIDTH as f32) * (new_height / (cardgen::CARDHEIGHT as f32));
        (new_width, new_height)
    };

    if scaling_with_height.0 < scaling_with_width.0 {
        scaling_with_height
    } else {
        scaling_with_width
    }
}

fn generate_deck_textures(
    deck: &Deck,
    filling_nodes: &Option<FillingNodes>,
    ctx: &egui::Context,
) -> TextureMap {
    // Generate the images for a deck
    let mut card_textures = HashMap::new();

    for (coord, visattr) in &deck.cards {
        let pixmap = render_card(*visattr, filling_nodes.as_ref().unwrap());

        let image = egui::ColorImage::from_rgba_unmultiplied(
            [pixmap.width() as _, pixmap.height() as _],
            pixmap.data(),
        );

        let texture = ctx.load_texture(format!("{:?}", visattr), image);
        card_textures.insert((*coord, *visattr), texture);
    }

    card_textures
}

// When compiling natively.
#[cfg(not(target_arch = "wasm32"))]
fn standard_deck_texture_promise(ctx: &egui::Context) -> Promise<TextureMap> {
    let deck = Deck::new_standard_deck();

    let cloned_context = ctx.clone();

    let rendering_func = move || {
        let filling_nodes = cardgen::generate_filling_nodes();
        generate_deck_textures(&deck, &filling_nodes, &cloned_context)
    };
    Promise::spawn_thread("Background standard deck rendering", rendering_func)
}

// When compiling for web. Wasm does not have thread support yet, so the operation blocks.
#[cfg(target_arch = "wasm32")]
fn standard_deck_texture_promise(ctx: &egui::Context) -> Promise<TextureMap> {
    let deck = Deck::new_standard_deck();

    let cloned_context = ctx.clone();

    let (sender, promise) = Promise::new();

    let filling_nodes = cardgen::generate_filling_nodes();
    sender.send(generate_deck_textures(
        &deck,
        &filling_nodes,
        &cloned_context,
    ));

    promise
}

fn standard_format(duration: Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;

    format!("{:02}:{:02}", minutes, seconds)
}
