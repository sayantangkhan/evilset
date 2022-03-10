use egui::style::Visuals;
use egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AppTheme {
    Light,
    Dark,
}

impl AppTheme {
    pub(crate) fn switch(&mut self) {
        match self {
            &mut AppTheme::Light => *self = AppTheme::Dark,
            &mut AppTheme::Dark => *self = AppTheme::Light,
        }
    }
}

pub(crate) fn generate_card_theme(app_theme: &AppTheme) -> Visuals {
    let mut theme = match app_theme {
        AppTheme::Light => Visuals::light(),
        AppTheme::Dark => Visuals::dark(),
    };

    // Setting widget colors for dark mode
    let widget = &mut theme.widgets;
    match app_theme {
        AppTheme::Light => {
            widget.inactive.bg_fill = Color32::from_rgb(230, 230, 230);
            widget.active.bg_fill = Color32::from_rgb(210, 210, 210);
            widget.hovered.bg_fill = Color32::from_rgb(190, 190, 190);
        }
        AppTheme::Dark => {
            widget.inactive.bg_fill = Color32::LIGHT_GRAY;
            widget.active.bg_fill = Color32::from_rgb(230, 230, 230);
            widget.hovered.bg_fill = Color32::from_rgb(250, 250, 250);
        }
    }

    // Setting selection and scrollbar colors for dark mode
    let selection_theme = &mut theme.selection;
    if let AppTheme::Dark = app_theme {
        selection_theme.bg_fill = Color32::LIGHT_BLUE;
    }

    theme
}

pub(crate) fn generate_base_theme(app_theme: &AppTheme) -> Visuals {
    match app_theme {
        AppTheme::Light => Visuals::light(),
        AppTheme::Dark => Visuals::dark(),
    }
}

pub(crate) fn thematic_red(app_theme: &AppTheme) -> Color32 {
    match app_theme {
        AppTheme::Light => Color32::RED,
        AppTheme::Dark => Color32::LIGHT_RED,
    }
}

pub(crate) fn thematic_blue(app_theme: &AppTheme) -> Color32 {
    match app_theme {
        AppTheme::Light => Color32::BLUE,
        AppTheme::Dark => Color32::LIGHT_BLUE,
    }
}
