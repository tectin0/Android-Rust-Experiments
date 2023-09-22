use egui::RichText;

use crate::helper::View;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Home {
    pub number_of_consecutive_months: usize,
    pub number_of_events_last_year: usize,
}

impl View for Home {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let consecutive_months_text = RichText::new(format!(
                "Consecutive Months: {}",
                self.number_of_consecutive_months
            ))
            .size(40.0);

            ui.label(consecutive_months_text);

            // ui.separator();

            let events_last_year_text = RichText::new(format!(
                "Events Last Year: {}",
                self.number_of_events_last_year
            ))
            .size(40.0);

            ui.label(events_last_year_text);
        });
    }
}
