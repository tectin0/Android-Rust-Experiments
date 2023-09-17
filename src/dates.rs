use std::ops::Mul;

use chrono::{Datelike, Local};
use egui::{
    plot::Text, Color32, ComboBox, Key, Label, RichText, ScrollArea, SelectableLabel, Sense, Style,
    TextEdit, TextStyle, WidgetText,
};
use itertools::Itertools;
use wgpu::RequestAdapterOptions;

use crate::{
    helper::{Demo, View},
    RepaintSignal,
};

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Dates {
    dates: Vec<(i32, u32, u32)>,
    selected_year: i32,
    selected_month: u32,
    selected_day: u32,
    pub number_of_consecutive_months: usize,
    pub number_of_events_last_year: usize,
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
        let screen_size = ui.ctx().input(|i| i.screen_rect.size());

        let mut has_dates_changed = false;
        let mut is_input_add_request = false;

        egui::ScrollArea::vertical()
            .max_height(screen_size.y / 2f32)
            .stick_to_right(true)
            .show(ui, |ui| {
                for (index, date) in &mut self.dates.clone().into_iter().enumerate() {
                    ui.horizontal(|ui| {
                        let label = Label::new(format!("{}-{}-{}", date.0, date.1, date.2));

                        let _input = ui.add_sized(
                            [ui.available_width() - 50.0, ui.spacing().interact_size.y],
                            label,
                        );

                        let delete = ui.button("x");
                        delete.clicked().then(|| {
                            has_dates_changed = true;
                            self.dates.remove(index);
                        });
                    });
                }
            });

        let mut selected_year = if self.selected_year == 0 {
            Local::now().year()
        } else {
            self.selected_year
        };

        let mut selected_month = if self.selected_month == 0 {
            Local::now().month()
        } else {
            self.selected_month
        };

        let mut selected_day = if self.selected_day == 0 {
            Local::now().day()
        } else {
            self.selected_day
        };

        ui.horizontal(|ui| {
            ui.set_height(100.0);

            let year_input = egui::ComboBox::from_id_source(0)
                .selected_text(
                    <WidgetText>::from(selected_year.to_string())
                        .text_style(TextStyle::Name("DateInputButton".into())),
                )
                .show_ui(ui, |ui| {
                    let current_year = Local::now().year();
                    for year in ((current_year - 15)..=current_year).rev() {
                        add_selectable_draggable_label(&mut selected_year, year, ui);
                    }
                });

            self.selected_year = selected_year;

            let month_input = egui::ComboBox::from_id_source(1)
                .selected_text(
                    <WidgetText>::from(selected_month.to_string())
                        .text_style(TextStyle::Name("DateInputButton".into())),
                )
                .show_ui(ui, |ui| {
                    for month in 1..=12 {
                        add_selectable_draggable_label(&mut selected_month, month, ui);
                    }
                });

            self.selected_month = selected_month;

            let day_input = egui::ComboBox::from_id_source(2)
                .selected_text(
                    <WidgetText>::from(selected_day.to_string())
                        .text_style(TextStyle::Name("DateInputButton".into())),
                )
                .show_ui(ui, |ui| {
                    for day in 1..=31 {
                        add_selectable_draggable_label(&mut selected_day, day, ui);
                    }
                });

            self.selected_day = selected_day;
        });

        ui.button("+").clicked().then(|| {
            is_input_add_request = true;
        });

        if is_input_add_request {
            has_dates_changed = true;
            self.dates
                .push((selected_year, selected_month, selected_day));
        }

        if has_dates_changed {
            self.sort_by_date();
            self.calculate_consecutive_months();
            self.calculate_number_of_events_last_year();
        }

        ui.label(format!(
            "Number of consecutive months: {}",
            self.number_of_consecutive_months
        ));

        ui.label(format!(
            "Number of events last year: {}",
            self.number_of_events_last_year
        ));
    }
}

fn add_selectable_draggable_label<T: Copy + Clone + PartialEq + ToString>(
    selected_value: &mut T,
    value: T,
    ui: &mut egui::Ui,
) {
    let selectable_label = SelectableLabel::new(*selected_value == value, value.to_string());

    let mut response = ui.add(selectable_label);

    let _ = response.interact(Sense::click_and_drag());

    if response.clicked() && *selected_value != value {
        *selected_value = value;
        response.mark_changed();
    }

    if response.dragged() {
        let drag_delta = response.drag_delta();

        if drag_delta.y.abs() > 0.0 {
            ui.scroll_with_delta(drag_delta.mul(0.75));
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
    fn calculate_consecutive_months(&mut self) {
        self.sort_by_date();

        let mut conesecutive_months = 0;

        let current_date = Local::now();
        let current_year = current_date.year();
        let current_month = current_date.month();

        let (last_year, last_month, last_day) = self.dates.last().unwrap();

        let month_difference = current_month as i32 - *last_month as i32;
        let year_difference = current_year - last_year;

        if month_difference == 0 && year_difference == 0 {
            conesecutive_months += 1;
        } else if month_difference == 1 && year_difference == 0 {
            conesecutive_months += 1;
        } else if month_difference == -11 && year_difference == 1 {
            conesecutive_months += 1;
        } else {
            self.number_of_consecutive_months = conesecutive_months;
            return;
        }

        for ((year, month, day), (previous_year, previous_month, previous_day)) in
            self.dates.iter().rev().tuple_windows()
        {
            let month_difference = *month as i32 - *previous_month as i32;
            let year_difference = year - previous_year;

            if month_difference == 0 && year_difference == 0 {
                continue;
            }

            if month_difference == 1 && year_difference == 0 {
                conesecutive_months += 1;
            } else if month_difference == -11 && year_difference == 1 {
                conesecutive_months += 1;
            } else {
                break;
            }
        }

        self.number_of_consecutive_months = conesecutive_months;
    }

    fn calculate_number_of_events_last_year(&mut self) {
        self.sort_by_date();

        let mut events_last_year = 0;

        let current_date = Local::now();
        let current_year = current_date.year();
        let current_month = current_date.month();

        for (year, month, day) in self.dates.iter().rev() {
            let month_difference = current_month - month;
            let year_difference = current_year - year;

            if year_difference == 0 {
                events_last_year += 1;
            } else if year_difference == 1 && month_difference < 0 {
                events_last_year += 1;
            } else {
                break;
            }
        }

        self.number_of_events_last_year = events_last_year;
    }

    fn sort_by_date(&mut self) {
        let mut dates = self.dates.clone();
        dates.sort_by(|(a_year, a_month, a_day), (b_year, b_month, b_day)| {
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

        self.dates = dates;
    }
}

fn get_current_date() -> String {
    let now = Local::now();
    format!(
        "{}-{}-{}",
        now.year().to_string(),
        now.month().to_string(),
        now.day().to_string()
    )
}
