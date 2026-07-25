use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::Sender;
use rayon::prelude::*;
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::options::{get_ritobin_path, Options};
/// uses ritobin to convert multiple bin files at the same time into a json file
pub fn bin_to_json(sender:&Sender<WorkerMessage>,options: &Options,champion: &str) -> Result<(), Box<dyn std::error::Error>>{
    let bin_path = format!(r"{}\0WADS\data\characters\{}\skins\",options.get_project_path() , champion);
    let files: Vec<_> = fs::read_dir(&bin_path).inspect_err(|e| {log(sender, format!("Error while reading work directory: {}", e))})?.collect();

    files.into_par_iter().for_each(|entry| {
        let entry_result = entry;
        if entry_result.is_ok() {
            let entry = entry_result.unwrap();
            let filepath_result = entry.file_name().into_string();
            if filepath_result.is_ok() {
                let filepath = filepath_result.unwrap();
                if filepath.ends_with(".bin") && filepath != "root.bin" {
                    bin_to_json_single(sender, options, &bin_path, &filepath).unwrap();
                }
            } else {
                log(sender, format!("Filepath error: {}", filepath_result.unwrap()))
            }
        } else {
            log(sender, format!("Entry error: {:?}", entry_result.unwrap()))
        }
});
    Ok(())
}
/// uses ritobin to convert a bin file to a json file
fn bin_to_json_single(sender:&Sender<WorkerMessage>, options: &Options, bin_path: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>>{
    let old_name = format!(r"{}{}", bin_path, filename);
    let new_name = format!(r"{}\{}{}", bin_path, &filename[..filename.len() - 3], "json");
    let output = Command::new("cmd") //todo test without hashing
        .args(["/C",
            &get_ritobin_path(options),
            "-d",
            format!(r"{}\0WADS\hashes\ritobin", options.get_project_path()).as_str(), // todo check existence?
            &old_name,
            &new_name])
        .output().inspect_err(|e| {log(sender, format!("Error while creating bin with ritobin: {}", e))})?;
    log(sender, format!("status: {}", output.status));
    // println!("status: {}", output.status);
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    Ok(())
}
/// uses ritobin to convert a json file to a bin file
pub fn json_to_bin(sender:&Sender<WorkerMessage>, options: &Options,champion: &str, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    let bin_path = format!(r"D:\wad5\{}\data\characters\{}\skins", champion_parent, champion);

    for entry in fs::read_dir(&bin_path).inspect_err(|e| {log(sender, format!("Could not read directory: {}", e))})? {
        let entry = entry.inspect_err(|e| {log(sender, format!("Entry failed: {}", e))})?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let output_path: PathBuf = path.with_extension("bin"); //todo write to wad.client instead (vorher anlegen)

            let output = Command::new("cmd")
                .args([
                    "/C",
                    &get_ritobin_path(options),
                    <&str>::try_from(path.as_os_str()).inspect_err(|e| {log(sender, format!("Could not convert to os path: {}", e))})?,
                    <&str>::try_from(output_path.as_os_str()).inspect_err(|e| {log(sender, format!("Could not convert to os path: {}", e))})?,
                ])
                .output().inspect_err(|e| {log(sender, format!("Error while creating bin with ritobin: {}", e))})?;
            log(sender, format!("status: {}", output.status));
            // println!("status: {}", output.status);
            // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
    Ok(())
}