#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all)]

mod backend_interface;
mod utility_functions;

// Platform specific imports
#[cfg(not(target_arch = "wasm32"))]
use background_render as render;

#[cfg(target_arch = "wasm32")]
use foreground_render as render;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
use instant::Instant;

// Platform independent imports
use crate::themes::AppTheme;
use backend_interface as backend;
use cardgen::CardVisualAttr;
use eframe::{
    egui::{Button, FontId, ImageButton, Key, Layout, RichText},
    epaint::TextureHandle,
    epi,
};
use setengine::{CardCoordinates, Deck, GameDeck, PlayResponse};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};
use utility_functions as util;

const TIMES_TO_DISPLAY: usize = 15;
const APP_KEY: &str = "evilset_app";
const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    static ref KEYBINDINGS: HashMap<Key, usize> = [
        (Key::Num1, 0),
        (Key::Num2, 1),
        (Key::Num3, 2),
        (Key::Q, 3),
        (Key::W, 4),
        (Key::E, 5),
        (Key::A, 6),
        (Key::S, 7),
        (Key::D, 8),
        (Key::Z, 9),
        (Key::X, 10),
        (Key::C, 11),
        (Key::Num4, 12),
        (Key::Num5, 13),
        (Key::Num6, 14),
        (Key::R, 15),
        (Key::T, 16),
        (Key::Y, 17),
        (Key::F, 18),
        (Key::G, 19),
        (Key::H, 20),
    ]
    .iter()
    .cloned()
    .collect();
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PersistentGameData {
    // Light and dark themes
    theme: AppTheme,
    // Keeps a track of TIMES_TO_DISPLAY best times in each category
    times: Times,
}

#[derive(serde::Deserialize, serde::Serialize)]
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

enum AppState {
    Menu,
    Help,
    Set,
    EvilSet,
    UltraSet,
    EvilUltraSet,
}

struct ActiveGameData {
    active_deck: GameDeck,
    card_textures: Option<TextureMap>,
    selected: HashSet<usize>,
    game_started: Option<Instant>,
    game_ended: Option<Instant>,
    prev_frame: Option<PlayResponse>,
    asked_for_hint: bool,
    updated_times: bool,
}

pub(crate) type TextureMap = HashMap<(CardCoordinates, CardVisualAttr), TextureHandle>;

#[derive(Default)]
struct RenderingPromises {
    standard_deck: Option<render::Promise<TextureMap>>,
    randomized_deck: Option<render::Promise<TextureMap>>,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EvilSetApp {
    // Game data and preferences that can be loaded from a file
    persistent_data: PersistentGameData,
    // Tracks whether the app is in the selection menu or one of the four games.
    #[serde(skip)]
    app_state: AppState,
    // Tracks previous state to perform cleanups safely
    #[serde(skip)]
    previous_state: Option<AppState>,
    // State of currently active game
    #[serde(skip)]
    game_data: Option<ActiveGameData>,
    // Background rendering promises
    #[serde(skip)]
    background_rendering: RenderingPromises,
}

impl Default for EvilSetApp {
    fn default() -> Self {
        Self {
            persistent_data: PersistentGameData::default(),
            app_state: AppState::Menu,
            previous_state: None,
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
        storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = storage {
            *self = epi::get_value(storage, APP_KEY).unwrap_or_default()
        }

        self.background_rendering.standard_deck = Some(render::standard_deck_texture_promise(ctx));
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, APP_KEY, self);
    }

    /// Save every second
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(1)
    }

    /// Increasing the size of web canvas
    fn max_size_points(&self) -> egui::Vec2 {
        (2160., 1620.).into()
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            persistent_data,
            app_state,
            previous_state,
            game_data,
            background_rendering,
        } = self;

        ctx.set_visuals(crate::themes::generate_base_theme(&persistent_data.theme));

        match *app_state {
            AppState::Menu => self.update_menu(ctx, frame),
            AppState::Set => self.play_set(ctx, frame),
            AppState::EvilSet => self.play_evilset(ctx, frame),
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
            app_state,
            previous_state,
            game_data,
            background_rendering,
        } = self;

        match previous_state {
            Some(_) => {
                *game_data = None;
                *previous_state = None;
            }
            None => {}
        };

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
                            ui.monospace(util::standard_format(time.clone()));
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
                            ui.monospace(util::standard_format(time.clone()));
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
                            ui.monospace(util::standard_format(time.clone()));
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
                            ui.monospace(util::standard_format(time.clone()));
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
                    *app_state = AppState::Set;
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
                    *app_state = AppState::EvilSet;
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
                    *app_state = AppState::UltraSet;
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
                    *app_state = AppState::EvilUltraSet;
                    println!("Evil Ultra Set selected");
                }
            })
        });
    }

    fn play_set(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let Self {
            persistent_data,
            app_state,
            previous_state,
            game_data,
            background_rendering,
        } = self;

        if game_data.is_none() {
            let rendering_promise = background_rendering.standard_deck.as_mut().unwrap();
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
                    let active_deck = GameDeck::start_set_play(&deck);

                    // For debugging purposes
                    // let mut active_deck = GameDeck::start_set_play(&deck);
                    // active_deck.in_deck_mut().clear();

                    *game_data = Some(ActiveGameData {
                        active_deck,
                        card_textures: Some(card_textures.clone()),
                        selected: HashSet::new(),
                        game_started: Some(Instant::now()),
                        game_ended: None,
                        prev_frame: None,
                        asked_for_hint: false,
                        updated_times: false,
                    });
                }
            }
        } else {
            // Checking if 3 cards have been selected, and if so, evaluating them for correctness
            backend::evaluate_selection(game_data.as_mut().unwrap());

            let game_still_running = match game_data.as_ref().unwrap().prev_frame {
                Some(PlayResponse::GameOver) => false,
                _ => true,
            };

            let best_times_updated = game_data.as_ref().unwrap().updated_times;
            if !game_still_running
                && !best_times_updated
                && !game_data.as_ref().unwrap().asked_for_hint
            {
                let times = &mut persistent_data.times.set_times;
                let elapsed_time = game_data.as_ref().unwrap().game_started.unwrap().elapsed()
                    - game_data.as_ref().unwrap().game_ended.unwrap().elapsed();
                times.push(elapsed_time);
                times.sort();
                let end_index = std::cmp::min(TIMES_TO_DISPLAY, times.len());
                *times = times[0..end_index].to_vec();
                game_data.as_mut().unwrap().updated_times = true;
            }

            // Handling the keyboard events if nothing happened previous frame
            if game_data.as_ref().unwrap().prev_frame.is_none() && game_still_running {
                keyboard_card_select(&ctx, game_data.as_mut().unwrap());
            }

            egui::CentralPanel::default().show(ctx, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's

                ui.horizontal(|ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        let close_button = ui.add(Button::new(RichText::new("❌").size(25.0)));
                        if close_button.clicked() {
                            *app_state = AppState::Menu;
                            *previous_state = Some(AppState::Set);
                        }

                        let hint_button = ui.add(Button::new(RichText::new("❓").size(25.0)));
                        if hint_button.clicked() && game_still_running {
                            backend::show_hint(game_data);
                        }
                    });

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
                                util::standard_format(if game_still_running {
                                    game_data.as_ref().unwrap().game_started.unwrap().elapsed()
                                } else {
                                    game_data.as_ref().unwrap().game_started.unwrap().elapsed()
                                        - game_data.as_ref().unwrap().game_ended.unwrap().elapsed()
                                })
                            ))
                            .font(FontId::proportional(28.0)),
                        );

                        let cards_left = game_data.as_ref().unwrap().active_deck.in_deck().len();

                        ui.label(
                            RichText::new(format!("{} cards left", cards_left))
                                .font(FontId::proportional(23.0)),
                        );
                        ctx.request_repaint();
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(20.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let ActiveGameData {
                        active_deck,
                        card_textures,
                        selected,
                        game_started,
                        game_ended,
                        prev_frame,
                        asked_for_hint,
                        updated_times,
                    } = game_data.as_mut().unwrap();

                    *ui.visuals_mut() =
                        crate::themes::generate_card_theme(&persistent_data.theme, prev_frame);

                    let available_width = ui.available_width();
                    let available_height = ui.available_height();

                    let card_textures = card_textures.as_ref().unwrap();

                    ui.columns(3, |columns| {
                        for (index, card) in active_deck.in_play().iter().enumerate() {
                            let texture = card_textures.get(card).unwrap();

                            let mut button = ImageButton::new(
                                texture,
                                util::scale_card(available_width, available_height),
                            );

                            if selected.contains(&index) {
                                button = button.selected(true);
                            }

                            let response = &mut columns[index % 3].add(button);

                            if response.clicked() {
                                if prev_frame.is_none() {
                                    backend::select_index(index, active_deck, selected);
                                }
                            }
                        }
                    });

                    if !game_still_running {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Game over").font(FontId::proportional(23.0)));

                            let close_button = ui
                                .add(Button::new(RichText::new("Return to main menu").size(23.0)));
                            if close_button.clicked() {
                                *app_state = AppState::Menu;
                                *previous_state = Some(AppState::Set);
                            }
                        });
                    }
                })
            });
        }
    }

    fn play_evilset(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let Self {
            persistent_data,
            app_state,
            previous_state,
            game_data,
            background_rendering,
        } = self;

        if game_data.is_none() {
            let deck = Deck::new_random_deck();
            let active_deck = GameDeck::start_set_play(&deck);

            // For debugging purposes
            // let mut active_deck = GameDeck::start_set_play(&deck);
            // active_deck.in_deck_mut().clear();

            *game_data = Some(ActiveGameData {
                active_deck,
                card_textures: None,
                selected: HashSet::new(),
                game_started: None,
                game_ended: None,
                prev_frame: None,
                asked_for_hint: false,
                updated_times: false,
            });

            let rendering_promise = render::deck_texture_promise(deck, ctx);
            background_rendering.randomized_deck = Some(rendering_promise);
        } else {
            match &background_rendering.randomized_deck {
                Some(rendering_promise) => {
                    match rendering_promise.ready() {
                        None => {
                            egui::CentralPanel::default().show(ctx, |ui| {
                                ui.vertical_centered_justified(|ui| {
                                    ui.label(
                                        RichText::new("Rendering cards")
                                            .font(FontId::proportional(18.0)),
                                    );
                                    ui.add(egui::Spinner::new()); // still loading
                                });
                            });
                        }
                        Some(card_textures) => {
                            game_data.as_mut().unwrap().card_textures = Some(card_textures.clone());
                            game_data.as_mut().unwrap().game_started = Some(Instant::now());
                            background_rendering.randomized_deck = None;
                        }
                    }
                }
                None => {
                    // Checking if 3 cards have been selected, and if so, evaluating them for correctness
                    backend::evaluate_selection(game_data.as_mut().unwrap());

                    let game_still_running = match game_data.as_ref().unwrap().prev_frame {
                        Some(PlayResponse::GameOver) => false,
                        _ => true,
                    };

                    let best_times_updated = game_data.as_ref().unwrap().updated_times;
                    if !game_still_running
                        && !best_times_updated
                        && !game_data.as_ref().unwrap().asked_for_hint
                    {
                        let times = &mut persistent_data.times.evilset_times;
                        let elapsed_time =
                            game_data.as_ref().unwrap().game_started.unwrap().elapsed()
                                - game_data.as_ref().unwrap().game_ended.unwrap().elapsed();
                        times.push(elapsed_time);
                        times.sort();
                        let end_index = std::cmp::min(TIMES_TO_DISPLAY, times.len());
                        *times = times[0..end_index].to_vec();
                        game_data.as_mut().unwrap().updated_times = true;
                    }

                    // Handling the keyboard events if nothing happened previous frame
                    if game_data.as_ref().unwrap().prev_frame.is_none() && game_still_running {
                        keyboard_card_select(&ctx, game_data.as_mut().unwrap());
                    }

                    egui::CentralPanel::default().show(ctx, |ui| {
                        // The central panel the region left after adding TopPanel's and SidePanel's

                        ui.horizontal(|ui| {
                            ui.with_layout(Layout::left_to_right(), |ui| {
                                let close_button =
                                    ui.add(Button::new(RichText::new("❌").size(25.0)));
                                if close_button.clicked() {
                                    *app_state = AppState::Menu;
                                    *previous_state = Some(AppState::Set);
                                }

                                let hint_button =
                                    ui.add(Button::new(RichText::new("❓").size(25.0)));
                                if hint_button.clicked() && game_still_running {
                                    backend::show_hint(game_data);
                                }
                            });

                            ui.vertical_centered(|ui| {
                                ui.heading(
                                    RichText::new("Set").font(FontId::proportional(28.0)).color(
                                        crate::themes::thematic_blue(&persistent_data.theme),
                                    ),
                                );
                            });

                            ui.with_layout(Layout::right_to_left(), |ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "⏱ {}",
                                        util::standard_format(if game_still_running {
                                            game_data
                                                .as_ref()
                                                .unwrap()
                                                .game_started
                                                .unwrap()
                                                .elapsed()
                                        } else {
                                            game_data
                                                .as_ref()
                                                .unwrap()
                                                .game_started
                                                .unwrap()
                                                .elapsed()
                                                - game_data
                                                    .as_ref()
                                                    .unwrap()
                                                    .game_ended
                                                    .unwrap()
                                                    .elapsed()
                                        })
                                    ))
                                    .font(FontId::proportional(28.0)),
                                );

                                let cards_left =
                                    game_data.as_ref().unwrap().active_deck.in_deck().len();

                                ui.label(
                                    RichText::new(format!("{} cards left", cards_left))
                                        .font(FontId::proportional(23.0)),
                                );
                                ctx.request_repaint();
                            });
                        });

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(20.0);

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            let ActiveGameData {
                                active_deck,
                                card_textures,
                                selected,
                                game_started,
                                game_ended,
                                prev_frame,
                                asked_for_hint,
                                updated_times,
                            } = game_data.as_mut().unwrap();

                            *ui.visuals_mut() = crate::themes::generate_card_theme(
                                &persistent_data.theme,
                                prev_frame,
                            );

                            let available_width = ui.available_width();
                            let available_height = ui.available_height();

                            let card_textures = card_textures.as_ref().unwrap();

                            ui.columns(3, |columns| {
                                for (index, card) in active_deck.in_play().iter().enumerate() {
                                    let texture = card_textures.get(card).unwrap();

                                    let mut button = ImageButton::new(
                                        texture,
                                        util::scale_card(available_width, available_height),
                                    );

                                    if selected.contains(&index) {
                                        button = button.selected(true);
                                    }

                                    let response = &mut columns[index % 3].add(button);

                                    if response.clicked() {
                                        if prev_frame.is_none() {
                                            backend::select_index(index, active_deck, selected);
                                        }
                                    }
                                }
                            });

                            if !game_still_running {
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        RichText::new("Game over").font(FontId::proportional(23.0)),
                                    );

                                    let close_button = ui.add(Button::new(
                                        RichText::new("Return to main menu").size(23.0),
                                    ));
                                    if close_button.clicked() {
                                        *app_state = AppState::Menu;
                                        *previous_state = Some(AppState::Set);
                                    }
                                });
                            }
                        })
                    });
                }
            }
        }
    }

    fn play_ultraset(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {}

    fn play_evilultraset(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {}
}

fn keyboard_card_select(context: &egui::Context, game_data: &mut ActiveGameData) {
    let events = &context.input().events;
    let active_deck = &mut game_data.active_deck;
    let selected_cards = &mut game_data.selected;
    let next_frames = &mut game_data.prev_frame;

    for event in events {
        if let egui::Event::Key {
            key,
            pressed,
            modifiers: _,
        } = event
        {
            if *pressed {
                if let Some(index) = KEYBINDINGS.get(key) {
                    backend::select_index(*index, active_deck, selected_cards);
                }
            }
        }
    }
}

fn generate_deck_textures(
    deck: &setengine::Deck,
    filling_nodes: &Option<cardgen::FillingNodes>,
    ctx: &egui::Context,
) -> TextureMap {
    // Generate the images for a deck
    let mut card_textures = HashMap::new();

    for (coord, visattr) in &deck.cards {
        let pixmap = cardgen::render_card(*visattr, filling_nodes.as_ref().unwrap());

        let image = egui::ColorImage::from_rgba_unmultiplied(
            [pixmap.width() as _, pixmap.height() as _],
            pixmap.data(),
        );

        let texture = ctx.load_texture(format!("{:?}", visattr), image);
        card_textures.insert((*coord, *visattr), texture);
    }

    card_textures
}

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
mod background_render {
    use super::{generate_deck_textures, TextureMap};
    pub use poll_promise::Promise;
    use setengine::Deck;

    pub(super) fn standard_deck_texture_promise(ctx: &egui::Context) -> Promise<TextureMap> {
        let deck = setengine::Deck::new_standard_deck();

        deck_texture_promise(deck.clone(), ctx)
    }

    pub(super) fn deck_texture_promise(deck: Deck, ctx: &egui::Context) -> Promise<TextureMap> {
        let cloned_context = ctx.clone();

        let rendering_func = move || {
            let filling_nodes = cardgen::generate_filling_nodes();
            generate_deck_textures(&deck, &filling_nodes, &cloned_context)
        };
        Promise::spawn_thread("Background deck rendering", rendering_func)
    }
}

// When compiling for the web
#[cfg(target_arch = "wasm32")]
mod foreground_render {
    use super::{generate_deck_textures, TextureMap};
    use setengine::Deck;

    pub(super) struct Promise<T> {
        deck: Deck,
        context: egui::Context,
        closure: fn(Deck, egui::Context) -> T,
        polled_once: bool,
        result: Option<T>,
    }

    impl<T> Promise<T> {
        fn create(
            deck: &Deck,
            context: &egui::Context,
            closure: fn(Deck, egui::Context) -> T,
        ) -> Promise<T> {
            Promise {
                deck: deck.clone(),
                context: context.clone(),
                closure,
                polled_once: false,
                result: None,
            }
        }

        pub(super) fn ready(&mut self) -> &Option<T> {
            if self.polled_once {
                if self.result.is_none() {
                    let function = self.closure;
                    self.result = Some(function(self.deck.clone(), self.context.clone()));
                }

                &self.result
            } else {
                self.polled_once = true;
                &self.result
            }
        }
    }

    pub(super) fn standard_deck_texture_promise(ctx: &egui::Context) -> Promise<TextureMap> {
        let deck = setengine::Deck::new_standard_deck();

        deck_texture_promise(deck, ctx)
    }

    pub(super) fn deck_texture_promise(deck: Deck, ctx: &egui::Context) -> Promise<TextureMap> {
        let rendering_func = |deck, context| {
            let filling_nodes = cardgen::generate_filling_nodes();
            generate_deck_textures(&deck, &filling_nodes, &context)
        };

        Promise::create(&deck, ctx, rendering_func)
    }
}
