use crate::converter::main_gui::log;
use std::fs;
use std::io::Read;
use std::sync::mpsc::Sender;
use crate::converter::main_gui::WorkerMessage;

/// Downloads hashes from Communitydragon
pub fn download_hashes(sender:&Sender<WorkerMessage>) -> Result<(), Box<dyn std::error::Error>>{
    let target_dir = "hashes";

    fs::create_dir_all(&target_dir)?;

    let hash_files = [
        "hashes.binentries.txt",
        "hashes.binfields.txt",
        "hashes.bintypes.txt",
        "hashes.game.txt",
    ];
    for basename in hash_files{
        log(sender, format!("Downloading {}", basename));
        let url = format!("https://raw.communitydragon.org/data/hashes/lol/{}", basename);
        let mut res = reqwest::blocking::get(url).inspect_err(|e| { log(sender, format!("Could not download hashes: {}", e))})?;
        if res.status().is_success() {
            let mut body = String::new();
            res.read_to_string(&mut body).inspect_err(|e| {log(sender, format!("Error while reading file: {}", e))})?;
            let path = target_dir.to_owned() + "//" + basename;
            fs::write(&path, body).inspect_err(|e| { log(sender,format!("Could not write to file {}: {}", &path, e))})?;
        }
    }
    Ok(())
}