use egui::{Color32, Key, TextEdit};

use crate::helper::{Demo, View};

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Dates {
    dates: Vec<String>,
    input: String,
    input_field_color: Color32,
    pub number_of_consecutive_months: usize,
    has_dates_changed: bool,
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
                    self.has_dates_changed = true;
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
                self.has_dates_changed = true;
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
            self.has_dates_changed = true;
            self.dates.push(self.input.clone());
            self.input.clear();
        });

        ui.label(format!(
            "Number of consecutive months: {}",
            self.number_of_consecutive_months
        ));

        if self.has_dates_changed {
            self.calculate_how_many_consecutive_months();
            self.has_dates_changed = false;
        }
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

impl Dates {
    /// Calculates in how many consecutive months the user has been active
    fn calculate_how_many_consecutive_months(&mut self) {
        self.sort_by_date();

        let mut conesecutive_months = 0;

        let dates_length = self.dates.len();

        for date in self.dates.iter().rev() {
            let date = date.split('-').collect::<Vec<&str>>();
            let year = date[0].parse::<i32>().unwrap();
            let month = date[1].parse::<i32>().unwrap();

            let previous_date = self
                .dates
                .get(dates_length - conesecutive_months - 1)
                .unwrap()
                .split('-')
                .collect::<Vec<&str>>();

            let previous_year = previous_date[0].parse::<i32>().unwrap();
            let previous_month = previous_date[1].parse::<i32>().unwrap();

            if year == previous_year && month == previous_month {
                conesecutive_months += 1;
            } else {
                break;
            }
        }

        self.number_of_consecutive_months = conesecutive_months;
    }

    fn sort_by_date(&self) {
        let mut dates = self.dates.clone();
        dates.sort_by(|a, b| {
            let a = a.split('-').collect::<Vec<&str>>();
            let b = b.split('-').collect::<Vec<&str>>();

            let a_year = a[0].parse::<i32>().unwrap();
            let a_month = a[1].parse::<i32>().unwrap();
            let a_day = a[2].parse::<i32>().unwrap();

            let b_year = b[0].parse::<i32>().unwrap();
            let b_month = b[1].parse::<i32>().unwrap();
            let b_day = b[2].parse::<i32>().unwrap();

            if a_year < b_year {
                return std::cmp::Ordering::Less;
            } else if a_year > b_year {
                return std::cmp::Ordering::Greater;
            }

            if a_month < b_month {
                return std::cmp::Ordering::Less;
            } else if a_month > b_month {
                return std::cmp::Ordering::Greater;
            }

            if a_day < b_day {
                return std::cmp::Ordering::Less;
            } else if a_day > b_day {
                return std::cmp::Ordering::Greater;
            }

            std::cmp::Ordering::Equal
        });
    }
}
