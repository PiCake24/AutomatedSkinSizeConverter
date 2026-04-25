use glob::Pattern;
use crate::cdtb::hash_file::HashFile;
use crate::cdtb::wad::Wad;

/// Extracts the wad-archives of the champion
///
/// # Arguments
/// * champion_parent: The champion of whom the wad should be made
pub fn wad_extract(champion_parent: &str) {
    let input_path = format!(r"D:\Riot Games\League of Legends\Game\DATA\FINAL\Champions\{}.wad.client", champion_parent);
    let output_path = format!(r"D:\wad5\{}", champion_parent);

    let mut hash_file = HashFile::new("hashes/hashes.game.txt".to_string());

    let mut wad = Wad::new(input_path, hash_file.load());
    let pattern = Pattern::new("*data*skins/*").unwrap();
    wad.files.retain(|f| pattern.matches(&f.path));

    wad.guess_extensions();
    wad.extract(output_path);
}
