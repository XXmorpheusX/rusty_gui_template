mod tests;

use std::ops::RangeInclusive;

use eframe::{egui, App};
use eframe::{egui::{widgets}, emath};

pub struct RImGui {
    native_options: eframe::NativeOptions,
}

impl RImGui {
    /// Create a new RImGui instance
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            native_options: eframe::NativeOptions {
                initial_window_size: Some(egui::Vec2::new(window_width, window_height)),
                ..Default::default()
            },
        }
    }

    /// Run the app
    pub fn run_native(&self, app_name: &str, app: Box<dyn App> ) -> Result<(), eframe::Error> {
        eframe::run_native(
            app_name,
            self.native_options.clone(),
            Box::new(|_cc| app),
        )
    }

    /// Create a title
    pub fn title(ui: &mut egui::Ui, title: &str) -> () {
        ui.heading(title);
    }

    /// Create a label
    pub fn label(ui: &mut egui::Ui, text: &str) -> () {
        ui.label(text);
    }

    // Create a input label (displayed horizontally)
    pub fn input_label_hor<S: widgets::text_edit::TextBuffer>(
        ui: &mut egui::Ui, 
        text: &str, 
        name: &mut S
    ) -> () {
        ui.horizontal(|ui| {
            let name_label = ui.label(text);
            ui.text_edit_singleline(name)
                .labelled_by(name_label.id);
        });
    }

    // Create a input label (displayed vertically)
    pub fn input_label_ver<S: widgets::text_edit::TextBuffer>(
        ui: &mut egui::Ui, 
        text: &str, 
        name: &mut S
    ) -> () {
        ui.vertical(|ui| {
            let name_label = ui.label(text);
            ui.text_edit_singleline(name)
                .labelled_by(name_label.id);
        });
    }

    /// Create a slider
    pub fn slider<Num: emath::Numeric>(
        ui: &mut egui::Ui,
        value: &mut Num,
        range: RangeInclusive<Num>,
        text: &str,
    ) -> () {
        ui.add(egui::Slider::new(value, range).text(text));
    }

    /// Create a button
    pub fn button(ui: &mut egui::Ui, text: &str, callback: impl FnOnce()) -> () {
        if ui.button(text).clicked() {
            callback();
        }
    }

    /// Spacing
    pub fn spacing(ui: &mut egui::Ui) -> () {
        ui.spacing();
    }

}