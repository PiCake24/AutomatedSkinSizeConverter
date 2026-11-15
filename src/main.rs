#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod cdtb;
mod converter;
mod data;

use crate::converter::main_gui::AutomatedSkinSizeConverter;
use std::default::Default;



// todo fn visibility reduction
// todo check paths in the app, so they are actually what the user sets and not my hardcoded ones
// todo add output to panics, because they do crash the app. Maybe try avoiding panics
// todo documentation
// todo check every file for warnings and errors and remove them
// todo replace strings with paths
// todo block ui when program is working
// todo improve error logs even more
const VERSION: &str = "3.0.0";
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Automated Skin Size Converter", options, Box::new(
        |_cc| Ok(Box::new(AutomatedSkinSizeConverter::new()))
    ))
}
