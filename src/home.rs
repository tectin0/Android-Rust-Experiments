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
        ui.heading("Infos");
        ui.vertical_centered(|ui| {
            ui.label(format!(
                "Consecutive Months: {}",
                self.number_of_consecutive_months
            ));

            ui.separator();

            ui.label(format!(
                "Events Last Year: {}",
                self.number_of_events_last_year
            ));
        });
    }
}
