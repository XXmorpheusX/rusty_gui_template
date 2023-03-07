mod tests;

use eframe::egui::{self, TextBuffer};

pub struct RImGui {}

impl RImGui {
    pub fn title(ui: &mut egui::Ui, title: &str) -> () {
        ui.heading(title);
    }

    pub fn input_label_hor(ui: &mut egui::Ui, text: &str, name: &mut str) -> () {
        ui.horizontal(|ui| {
            let name_label = ui.label(text);
            ui.text_edit_singleline(&mut name)
                .labelled_by(name_label.id);
        });
    }
}
