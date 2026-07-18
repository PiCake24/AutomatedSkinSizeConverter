use crate::converter::main_gui::log;
use std::fs;
use std::io::Read;
use std::sync::mpsc::Sender;
use crate::converter::main_gui::WorkerMessage;
use crate::data::options::Options;

/// Downloads hashes from Communitydragon
pub fn download_hashes(option: &Options, sender:&Sender<WorkerMessage>) -> Result<(), Box<dyn std::error::Error>>{
    let target_dir = format!(r"{}\0WADS\hashes", option.get_project_path());
    log(sender, format!("Writing hashes to: {}", &target_dir));

    fs::create_dir_all(format!(r"{}\ritobin",&target_dir)).inspect_err(|e| {log(sender, format!("Error while creating directory: {}", e))})?;


    let hash_files = [
        "hashes.binentries.txt",
        "hashes.binfields.txt",
        "hashes.game.txt",
    ];
    for basename in hash_files{
        log(sender, format!("Downloading {}", basename));
        let url = format!("https://raw.communitydragon.org/data/hashes/lol/{}", basename);
        let mut res = reqwest::blocking::get(url).inspect_err(|e| { log(sender, format!("Could not download hashes: {}", e))})?;
        if res.status().is_success() {
            let mut body = String::new();
            res.read_to_string(&mut body).inspect_err(|e| {log(sender, format!("Error while reading file: {}", e))})?;
            log(sender, format!("Writing {}", basename));
            if basename == "hashes.game.txt"{
                fs::write(target_dir.to_owned() + "//" + basename, body).inspect_err(|e| {log(sender, format!("Error while writing file: {}", e))})?;
            } else{
                fs::write(target_dir.to_owned() + "//ritobin//" + basename, body).inspect_err(|e| {log(sender, format!("Error while writing file: {}", e))})?;
            }
        }
    }
    Ok(())
}