mod cdtb;
mod converter;
mod data;

use std::default::Default;
use std::fs::File;
use std::path::Path;
use eframe::{egui, Frame, Storage};
use eframe::egui::{Context, TextBuffer};
use crate::converter::main_gui::AutomatedSkinSizeConverter;
use crate::data::options::Options;

// todo version, version check
// todo fn visibility reduction
// todo check paths in the app, so they are actually what the user sets and not my hardcoded ones
// todo add output to panics, because they do crash the app. Maybe try avoiding panics
// (especially in cdtb lol)
// todo documentation
// todo check every file for warnings and errors and remove them
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", options, Box::new(
        |_cc| Ok(Box::new(AutomatedSkinSizeConverter::new()))
    ))
}



