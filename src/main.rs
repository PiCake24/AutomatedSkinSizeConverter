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

fn main() -> Result<(), eframe::Error> {
    check_options();
    let sets = check_sets();
    let options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", options, Box::new(
        |_cc| Ok(Box::new(AutomatedSkinSizeConverter::new(sets)))
    ))
}

fn check_options() -> Options{
    let options_file = Path::new("Options.txt");
    if !options_file.exists(){
        //todo ask user
        File::create_new(options_file).expect("TODO: panic message");
        //write into file
        // ask user for the options values
        println!("no exists");
        Options::new()
    } else{
        //read file
        Options::new()
    }
    //does options exist, if yes read it, else create it
}

fn check_sets() -> Vec<String>{
    return vec!("Default".to_string())
    //check if set folders exist, if yes, load them, if no create default one
    //when creating default one copy files into it
}