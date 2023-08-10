use egui::{Context, Modifiers, Ui};

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
        if self.about_is_open {
            let screen_size = ctx.input(|i| i.screen_rect.size());
            let default_width = (screen_size.x - 20.0).min(400.0);

            let mut is_continue_button = false;
            let mut is_open = true;

            egui::Window::new(self.about.name())
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .default_width(default_width)
                .default_height(ctx.available_rect().height() - 46.0)
                .vscroll(true)
                .open(&mut is_open)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    self.about.ui(ui);
                    ui.add_space(12.0);
                    ui.vertical_centered_justified(|ui| {
                        if ui
                            .button(egui::RichText::new("Continue to the demo!").size(20.0))
                            .clicked()
                        {
                            is_continue_button = true;
                        }
                    });
                });

            self.about_is_open &= !is_continue_button & is_open;
        } else {
            self.mobile_top_bar(ctx);
            self.show_windows(ctx);
        }
    }

    fn mobile_top_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let font_size = 16.5;

                ui.menu_button(egui::RichText::new("⏷ demos").size(font_size), |ui| {
                    ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
                    if ui.ui_contains_pointer() && ui.input(|i| i.pointer.any_click()) {
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    use egui::special_emojis::{GITHUB, TWITTER};
                    ui.hyperlink_to(
                        egui::RichText::new(TWITTER).size(font_size),
                        "https://twitter.com/ernerfeldt",
                    );
                    ui.hyperlink_to(
                        egui::RichText::new(GITHUB).size(font_size),
                        "https://github.com/emilk/egui",
                    );
                });
            });
        });
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("egui_demo_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                egui::trace!(ui);
                ui.vertical_centered(|ui| {
                    ui.heading("✒ egui demos");
                });

                ui.separator();

                use egui::special_emojis::{GITHUB, TWITTER};
                ui.hyperlink_to(
                    format!("{} egui on GitHub", GITHUB),
                    "https://github.com/emilk/egui",
                );
                ui.hyperlink_to(
                    format!("{} @ernerfeldt", TWITTER),
                    "https://twitter.com/ernerfeldt",
                );

                ui.separator();
            });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                file_menu_button(ui);
            });
        });

        self.show_windows(ctx);
    }

    /// Show the open windows.
    fn show_windows(&mut self, ctx: &Context) {
        self.about.show(ctx, &mut self.about_is_open);
    }
}

fn file_menu_button(ui: &mut Ui) {
    let organize_shortcut =
        egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::O);
    let reset_shortcut =
        egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::R);

    // NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
    // or else they would only be checked if the "File" menu was actually open!

    if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
        ui.ctx().memory_mut(|mem| mem.reset_areas());
    }

    if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
        ui.ctx().memory_mut(|mem| *mem = Default::default());
    }

    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        // On the web the browser controls the zoom
        #[cfg(not(target_arch = "wasm32"))]
        {
            egui::gui_zoom::zoom_menu_buttons(ui, None);
            ui.separator();
        }

        if ui
            .add(
                egui::Button::new("Organize Windows")
                    .shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)),
            )
            .clicked()
        {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
            ui.close_menu();
        }

        if ui
            .add(
                egui::Button::new("Reset egui memory")
                    .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
            )
            .on_hover_text("Forget scroll, positions, sizes etc")
            .clicked()
        {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
            ui.close_menu();
        }
    });
}
