use egui::{CentralPanel, Context, Modifiers, Ui};

use crate::{
    about::About,
    helper::{is_mobile, Demo, View},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct MainWindows {
    about_is_open: bool,
    about: About,
}

impl Default for MainWindows {
    fn default() -> Self {
        Self {
            about_is_open: true,
            about: Default::default(),
        }
    }
}

impl MainWindows {
    /// Show the app ui (menu bar and windows).
    pub fn ui(&mut self, ctx: &Context) {
        if is_mobile(ctx) {
            self.mobile_ui(ctx);
        } else {
            self.desktop_ui(ctx);
        }
    }

    fn mobile_ui(&mut self, ctx: &Context) {
        let screen_size = ctx.input(|i| i.screen_rect.size());
        let default_width = (screen_size.x - 20.0).min(400.0);

        self.show_main_gui(ctx);
        self.show_about(ctx);
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        self.show_main_gui(ctx);
        self.show_about(ctx);
    }

    /// Show the about window.
    fn show_about(&mut self, ctx: &Context) {
        self.about.show(ctx, &mut self.about_is_open);
    }

    fn show_main_gui(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Welcome to the Egui demo app!");
                let button = ui
                    .button("Quit")
                    .on_hover_cursor(egui::CursorIcon::PointingHand);

                button.clicked().then(|| std::process::exit(0));
            });
        });
    }
}
