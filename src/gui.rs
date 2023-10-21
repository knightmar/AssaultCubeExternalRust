#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// hide console window on Windows in release
use crate::app::TemplateApp;
use crate::cheat_helper::data::DataSaver;
use lazy_static::lazy_static;
use std::sync::RwLock;
use std::sync::{Arc, Mutex};

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn run(data_saver: Arc<RwLock<DataSaver>>) -> eframe::Result<()> {
    // Initialize the logger and other setup as needed.
    env_logger::init();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([1000.0, 400.0].into()),
        min_window_size: Some([400.0, 220.0].into()),
        ..Default::default()
    };

    let result = eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| {
            // Create the TemplateApp instance and pass data_saver to it.
            let app = TemplateApp::new(cc, data_saver);
            Box::new(app)
        }),
    );

    if result.is_ok() {
        println!("okay");
    }

    result
}
