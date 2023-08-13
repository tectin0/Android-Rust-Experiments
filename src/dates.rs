use egui::{Color32, Key, TextEdit};

use crate::helper::{Demo, View};

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Dates {
    dates: Vec<String>,
    input: String,
    input_field_color: Color32,
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

        let input = TextEdit::singleline(&mut self.input).text_color(self.input_field_color);
        let input_response = ui.add(input);

        input_response.request_focus();

        input_response
            .ctx
            .input(|i| i.key_pressed(Key::Enter))
            .then(|| {
                self.dates.push(self.input.clone());
                self.input.clear();
            });

        input_response.changed().then(|| {
            self.input_field_color = match check_if_valid_date(self.input.clone().as_str()) {
                true => Color32::from_rgb(0, 255, 0),
                false => Color32::from_rgb(255, 0, 0),
            };
            ui.ctx().request_repaint();
        });

        ui.button("+").clicked().then(|| {
            self.dates.push(self.input.clone());
            self.input.clear();
        });
    }
}

fn check_if_valid_date(date: &str) -> bool {
    let date = date.split('-').collect::<Vec<&str>>();
    if date.len() != 3 {
        return false;
    }

    let year = date[0].parse::<i32>();
    let month = date[1].parse::<i32>();
    let day = date[2].parse::<i32>();

    if year.is_err() || month.is_err() || day.is_err() {
        return false;
    }

    let year = year.unwrap();
    let month = month.unwrap();
    let day = day.unwrap();

    if year < 0 || month < 0 || day < 0 {
        return false;
    }

    if month > 12 || day > 31 {
        return false;
    }

    true
}
