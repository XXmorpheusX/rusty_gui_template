mod tests;

use eframe::egui;
use rust_library_template::RImGui;

pub struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // App title
            RImGui::label(ui, "Title");
            RImGui::title(ui, "My first app");
            RImGui::spacing(ui);

            // Input label vertical and horizontal
            RImGui::label(ui, "Input label");
            //RImGui::input_label_ver(ui, "Your name: ", &mut self.name);
            RImGui::input_label_hor(ui, "Your name: ", &mut self.name);

            // Slider
            RImGui::label(ui, "Slider");
            RImGui::slider(ui, &mut self.age, 0..=120, "age");

            // Button
            RImGui::label(ui, "Button");
            RImGui::button(ui, "Click each year", || { self.age += 1 });
        });
    }
}