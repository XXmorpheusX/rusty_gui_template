#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod app;

use app::MyApp;
use rust_library_template::RImGui;

fn main() -> Result<(), eframe::Error> {

    let app = RImGui::new(320.0, 240.0);

    app.run_native("app_name", Box::new(MyApp::default()))
}