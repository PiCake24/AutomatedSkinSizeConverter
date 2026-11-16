use crate::converter::file_converter2::{bin_to_py, py_to_bin};
use crate::converter::main_gui::{WorkerMessage, log};
use crate::data::options::{Options, get_ritobin_path};
use rayon::prelude::*;
use std::fs;
use std::ops::Add;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::Sender;
use crate::data::champion::Champion;

/// uses ritobin to convert multiple bin files at the same time into a json file
pub fn bin_to_json(sender:&Sender<WorkerMessage>,options: &Options,champion: &str) -> Result<(), Box<dyn std::error::Error>>{
    let bin_path = format!(r"{}\0WADS\data\characters\{}\skins\",options.get_project_path() , champion);
    let files: Vec<_> = fs::read_dir(&bin_path).inspect_err(|e| {log(sender, format!("Error while reading work directory: {}", e))})?.collect();
    //todo ideally only convert the files we need
    files.into_par_iter().for_each(|entry| { //todo ganz so einfach ist dass dann doch nicht
        let entry_result = entry;
        if entry_result.is_ok() {
            let entry = entry_result.unwrap();
            let filepath_result = entry.file_name().into_string();
            if filepath_result.is_ok() {
                let filepath = filepath_result.unwrap();
                if filepath.ends_with(".bin") && filepath != "root.bin" {
                    bin_to_py(sender, options, &bin_path, &filepath);

                }
            } else {
                log(sender, format!("Filepath error: {:?}", filepath_result))
            }
        } else {
            log(sender, format!("Entry error: {:?}", entry_result))
        }
});
    Ok(())
}
/// uses ritobin to convert a bin file to a json file
fn bin_to_json_single(sender:&Sender<WorkerMessage>, options: &Options, bin_path: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>>{
    let old_name = format!(r"{}{}", bin_path, filename);
    let new_name = format!(r"{}\{}{}", bin_path, &filename[..filename.len() - 3], "json");
    let output = Command::new(&get_ritobin_path(options))
        .args([
            "-d",
            format!(r"{}\0WADS\hashes\ritobin", options.get_project_path()).as_str(),
            &old_name,
            &new_name])
        .output().inspect_err(|e| {log(sender, format!("Error while creating bin with ritobin: {}", e))})?;
    if output.status.success() {
        log(sender, format!("Successful converted {} to json", filename));
    } else {
        log(sender, format!("Ritobin Error: {}: {}", output.status, String::from_utf8_lossy(&output.stderr)));
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other, format!("WAD_MAKE Error: {}: {}", output.status, String::from_utf8_lossy(&output.stderr))
        ).into())
    }
    Ok(())
}
/// uses ritobin to convert a json file to a bin file
pub fn json_to_bin(sender:&Sender<WorkerMessage>, options: &Options,champion: &Champion) -> Result<(), Box<dyn std::error::Error>>{ //todo also only convert files we need
    let input_path = format!(r"{}\0WADS\data\characters\{}\skins", options.get_project_path(), champion.get_name());

    let output_path = format!(r"{}\{}.wad.client\data\characters\{}\skins", options.get_project_path(), champion.get_name(), champion.get_name());
    fs::create_dir_all(&output_path).inspect_err(|e| {log(sender, format!("Could not create output directory: {}", e))})?;

    // for entry in fs::read_dir(&input_path).inspect_err(|e| {log(sender, format!("Could not read directory: {}", e))})? {
    for skin in champion.get_skins(){
        let input = format!(r"{}\skin{}.py", input_path, skin.get_skin());
        let output = format!(r"{}\skin{}.bin", output_path, skin.get_skin());
        py_to_bin(sender, &input, &output)?;
    // }
        // let entry = entry.inspect_err(|e| {log(sender, format!("Entry failed: {}", e))})?;
        // let path = entry.path();
        //
        //
        //
        // if path.extension().and_then(|e| e.to_str()) == Some("py") {
        //     let filename = path.as_path().file_name().unwrap().to_str().unwrap().to_string();
        //     let filename:String = filename[..filename.len() - 2].to_string();
        //     py_to_bin(sender, path.to_str().unwrap(), &format!("{}/{}{}", &output_path, filename, "bin"))?;

            // let file_stem = path.file_stem().ok_or("No file stem").inspect_err(|e| {log(sender, format!("Could not get file stem for {:?}: {}",path, e))})?;
            // let output_file = Path::new(&output_path)
            //     .join(file_stem)
            //     .with_extension("bin");
            //
            // let output = Command::new("cmd")
            //     .args([
            //         "/C",
            //         &get_ritobin_path(options),
            //         <&str>::try_from(path.as_os_str()).inspect_err(|e| {log(sender, format!("Could not convert to os path: {}", e))})?,
            //         <&str>::try_from(output_file.as_os_str()).inspect_err(|e| {log(sender, format!("Could not convert to os path: {}", e))})?,
            //     ])
            //     .output().inspect_err(|e| {log(sender, format!("Error while creating bin with ritobin: {}", e))})?;
            // if output.status.success() {
            //     log(sender, format!("Successful converted {} to bin", json_path));
            // } else{
            //     log(sender, format!("Ritobin Error: {}: {}", output.status, String::from_utf8_lossy(&output.stderr)));
            //     return Err(std::io::Error::new(
            //         std::io::ErrorKind::Other, format!("WAD_MAKE Error: {}: {}", output.status, String::from_utf8_lossy(&output.stderr))
            //     ).into())
            // }
        // }
    }
    Ok(())
}