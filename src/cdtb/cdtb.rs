use std::sync::mpsc::Sender;
use glob::Pattern;
use crate::cdtb::hash_file::HashFile;
use crate::cdtb::wad::Wad;
use crate::converter::main_gui::WorkerMessage;

/// Extracts the wad-archives of the champion
///
/// # Arguments
/// * champion_parent: The champion of whom the wad should be made
pub fn wad_extract(sender:&Sender<WorkerMessage>, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    let input_path = format!(r"D:\Riot Games\League of Legends\Game\DATA\FINAL\Champions\{}.wad.client", champion_parent);
    let output_path = format!(r"D:\wad5\{}", champion_parent);

    //todo different Hash-file location
    let mut hash_file = HashFile::new("hashes/hashes.game.txt".to_string());

    let mut wad = Wad::new(sender, input_path, hash_file.load(sender)?);
    let pattern = Pattern::new("*data*skins/*")?; //todo can I optimise this?
    wad.files.retain(|f| pattern.matches(&f.path));

    wad.guess_extensions(sender);
    wad.extract(sender, output_path)?;
    Ok(())
}
