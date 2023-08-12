use egui::Key;

use crate::helper::{Demo, View};

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Dates {
    dates: Vec<String>,
    input: String,
}

impl Demo for Dates {
    fn name(&self) -> &'static str {
        "Dates"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .open(open)
            .show(ctx, |ui| {
                use View as _;
                self.ui(ui);
            });
    }
}

impl View for Dates {
    fn ui(&mut self, ui: &mut egui::Ui) {
        for (index, date) in &mut self.dates.clone().into_iter().enumerate() {
            ui.horizontal(|ui| {
                let _input = ui.label(format!("{}", date));
                let delete = ui.button("x");
                delete.clicked().then(|| {
                    self.dates.remove(index);
                });
            });
        }

        ui.text_edit_singleline(&mut self.input)
            .ctx
            .input(|i| i.key_pressed(Key::Enter))
            .then(|| {
                self.dates.push(self.input.clone());
                self.input.clear();
            });
        ui.button("+").clicked().then(|| {
            self.dates.push(self.input.clone());
            self.input.clear();
        });
    }
}
