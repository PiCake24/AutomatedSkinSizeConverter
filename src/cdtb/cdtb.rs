use std::sync::mpsc::Sender;
use glob::Pattern;
use crate::cdtb::hash_file::HashFile;
use crate::cdtb::wad::Wad;
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::options::Options;

/// Extracts the wad-archives of the champion
///
/// # Arguments
/// * champion_parent: The champion of whom the wad should be made
pub fn wad_extract(option: &Options, sender:&Sender<WorkerMessage>, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    let input_path = format!(r"{}\DATA\FINAL\Champions\{}.wad.client", option.get_league_path(), champion_parent);
    let output_path = format!(r"{}\0WADS", option.get_project_path());
    log(sender, format!(r"Using hash: {}\0WADS\hashes\hashes.game.txt", option.get_project_path()));
    let mut hash_file = HashFile::new(format!(r"{}\0WADS\hashes\hashes.game.txt", option.get_project_path()));


    let mut wad = Wad::new(sender, input_path, hash_file.load(sender)?)?; //todo?
    let pattern = Pattern::new("*data*skins/*")?; //todo can I optimise this?
    wad.files.retain(|f| pattern.matches(&f.path));

    wad.guess_extensions(sender)?;
    wad.extract(sender, output_path)?;
    Ok(())
}
