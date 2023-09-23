use egui::{Button, CentralPanel, Context, FontId, Id, RichText};

use egui::TextStyle::*;

use egui::FontFamily::Proportional;

use crate::helper::Demo;
use crate::{
    about::About,
    dates::Dates,
    helper::{is_mobile, View},
    home::Home,
};

#[derive(Default)]
enum MainWindowState {
    #[default]
    Main,
    Dates,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct MainWindows {
    main_window_state: MainWindowState,
    home: Home,
    dates: Dates,
    about: About,
    is_about_open: bool,
}

impl Default for MainWindows {
    fn default() -> Self {
        Self {
            main_window_state: Default::default(),
            home: Default::default(),
            dates: Default::default(),
            about: Default::default(),
            is_about_open: true,
        }
    }
}

impl MainWindows {
    pub fn ui(&mut self, ctx: &Context) {
        let mut style = (*ctx.style()).clone();

        style.text_styles = [
            (Heading, FontId::new(45.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(32.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(24.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
            (
                Name("DateInputButton".into()),
                FontId::new(45.0, Proportional),
            ),
            (
                Name("BottomBarButton".into()),
                FontId::new(35.0, Proportional),
            ),
        ]
        .into();

        ctx.set_style(style);

        self.home.number_of_consecutive_months = self.dates.number_of_consecutive_months;
        self.home.number_of_events_last_year = self.dates.number_of_events_last_year;

        if is_mobile() {
            self.mobile_ui(ctx);
        } else {
            self.desktop_ui(ctx);
        }
    }

    fn mobile_ui(&mut self, ctx: &Context) {
        // let screen_size = ctx.input(|i| i.screen_rect.size());
        // let default_width = (screen_size.x - 20.0).min(400.0);

        self.show_main_gui(ctx);
        // self.show_about(ctx);
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        self.show_main_gui(ctx);
        // self.show_about(ctx);
    }

    #[allow(dead_code)]
    /// Show the about window.
    fn show_about(&mut self, ctx: &Context) {
        self.about.show(ctx, &mut self.is_about_open);
    }

    fn bottom_bar(&mut self, ctx: &Context) {
        let screen_size = ctx.input(|i| i.screen_rect.size());
        let bottom_panel_height = screen_size.y / 10.0;
        let button_width = screen_size.x / 3.0 - 10.0;

        let bottom_panel =
            egui::TopBottomPanel::bottom(Id::new("bottom_bar")).exact_height(bottom_panel_height);

        bottom_panel.show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                let home_button = Button::new(RichText::new("Home").size(40.0));

                ui.add_sized([button_width, bottom_panel_height / 2.0], home_button)
                    .clicked()
                    .then(|| {
                        self.main_window_state = MainWindowState::Main;
                    });

                let dates_button = Button::new(RichText::new("Dates").size(40.0));

                ui.add_sized([button_width, bottom_panel_height / 2.0], dates_button)
                    .clicked()
                    .then(|| {
                        self.main_window_state = MainWindowState::Dates;
                    });

                let quit_button = Button::new(RichText::new("Quit").size(40.0));

                ui.add_sized([button_width, bottom_panel_height / 2.0], quit_button)
                    .clicked()
                    .then(|| std::process::exit(0));
            });
        });
    }

    fn show_main_gui(&mut self, ctx: &Context) {
        self.bottom_bar(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| match self.main_window_state {
                MainWindowState::Main => self.home.ui(ui),
                MainWindowState::Dates => self.dates.ui(ui),
            });
        });
    }
}
