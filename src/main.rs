#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, RwLock};
use std::thread;

use crate::cheat::CheatsManager;
use crate::cheat_helper::data::DataSaver;

mod app;
mod cheat;
mod cheat_helper;
mod gui;
mod mem;
mod cheats;

fn main() {
    let data_saver = DataSaver {
        infinite_ammo: false,
        tp_text: String::from("example"),
        is_running: true,
    };

    // Wrap your struct in an Arc with an RwLock
    let data_saver = Arc::new(RwLock::new(data_saver));

    // Clone the Arc when you need to share it across threads

    let cheat_manager = CheatsManager {
        running: false,
        data_saver: data_saver.clone(),
    };

    let cheat_thread = thread::spawn(move || unsafe {
        cheat_manager.run();
    });

    gui::run(data_saver.clone()).expect("TODO: panic message");


    cheat_thread.join().unwrap();
}
