mod cdtb;
mod converter_gui;
mod data;

use std::default::Default;
use std::fs::File;
use std::path::Path;
use eframe::{egui, Frame, Storage};
use eframe::egui::{Context, TextBuffer};
use crate::converter_gui::main_gui::AutomatedSkinSizeConverter;
use crate::data::options::Options;

//todo version

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", options, Box::new(
        |_cc| Ok(Box::new(AutomatedSkinSizeConverter::new()))
    ))
}



