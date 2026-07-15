use std::fs;
use std::path::PathBuf;
use std::process::Command;
use rayon::prelude::*;
use crate::data::options::Options;
/// uses ritobin to convert multiple bin files at the same time into a json file
pub fn bin_to_json(options: &Options,champion: &str, champion_parent: &str){
    let ritobin_path = r"D:\Programs verknuepfng\Programs\ritobin\ritobin_cli.exe"; //todo take from options (it is somewhere in root)
    let bin_path = format!(r"D:\wad5\{}\data\characters\{}\skins\", champion_parent, champion);
    let files: Vec<_> = fs::read_dir(&bin_path).unwrap().collect();

    files.into_par_iter().for_each(|entry| {
        let entry = entry.unwrap();
        let filepath = entry.file_name().into_string().unwrap();

        if filepath.ends_with(".bin") && filepath != "root.bin" {
            bin_to_json_single(ritobin_path, &bin_path, &filepath).expect("ritobin failed");
        }
    });
}
/// uses ritobin to convert a bin file to a json file
fn bin_to_json_single(ritobin_path: &str, bin_path: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>>{
    let old_name = format!(r"{}{}", bin_path, filename);
    let new_name = format!(r"{}\{}{}", bin_path, &filename[..filename.len() - 3], "json");
    let output = Command::new("cmd") //todo test without hashing
        .args(["/C",
            ritobin_path,
            "-d",
            r"C:\Users\Yanni\RustroverProjects\Skinswapper\hashes\ritobin",
            &old_name,
            &new_name])
        .output()?;
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    Ok(())
}
/// uses ritobin to convert a json file to a bin file
pub fn json_to_bin(options: &Options,champion: &str, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{ //todo remove this return, make error handling in file
    //todo change path of ritobin, ideally have a somewhat global path, where both methods use it
    let ritobin_path = r"D:\Programs verknuepfng\Programs\ritobin\ritobin_cli.exe";
    let bin_path = format!(r"D:\wad5\{}\data\characters\{}\skins", champion_parent, champion);

    for entry in fs::read_dir(&bin_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let output_path: PathBuf = path.with_extension("bin");

            let output = Command::new("cmd")
                .args([
                    "/C",
                    ritobin_path,
                    <&str>::try_from(path.as_os_str())?,
                    <&str>::try_from(output_path.as_os_str())?,
                ])
                .output()?;
            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
    Ok(())
}