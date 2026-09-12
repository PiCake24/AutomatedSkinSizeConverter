use std::fs;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use chrono::{Utc, DateTime};
use serde_json::{json, Value};
use walkdir::WalkDir;
use zip::write::FileOptions;
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::options::Options;

/// todo
pub fn export_cslol(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    log(sender, "Creating WAD file");
    create_wad_file();
    log(sender, "Creating cslol mod folder");
    create_cslol_folder(sender, option, champion_parent)?; //todo logs
    log(sender, "Creating META file");
    create_cslol_meta(sender, option, champion_parent)?;
    log(sender, "Moving mod");
    move_wad_archive();
    log(sender, "Exported to cslol");
    Ok(())
}
///uses wadmake to create a new wad archive
fn create_wad_file(){
    todo!()
}
/// creates needed folders in cslol directory
fn create_cslol_folder(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    create_dir_all(format!(r"{}\installed\giant {}\WAD", option.get_cslol_path(), champion_parent))
        .inspect_err(|e| {log(sender, format!("Could not create WAD folder: {}", e))})?;
    create_dir_all(format!(r"{}\installed\giant {}\META", option.get_cslol_path(), champion_parent))
        .inspect_err(|e| {log(sender, format!("Could not create META folder: {}", e))})?;
    Ok(())
}
/// creates meta file for cslol mod
fn create_cslol_meta(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut file = File::create(format!(r"{}\installed\giant {}\META\info.json", option.get_cslol_path(), champion_parent))
        .inspect_err(|e| {log(sender, format!("Could not create info.json: {}", e))})?;
    let text = format!(r#"{{
    "Author": "AutomatedSkinSizeConverter",
    "Description": "",
    "Heart": "",
    "Home": "",
    "Name": "Giant {}",
    "Version": "1.0"
}}"#, champion_parent);
    file.write_all(text.as_ref()).inspect_err(|e| {log(sender, format!("Could not write into info.json: {}", e))})?;
    Ok(())
}
/// moves the wad archive to the mod folder in cslol
fn move_wad_archive(){
    todo!()
}
/// exports the mod to ltk
pub fn export_ltk(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    log(sender, "Creating Folder");
    create_proejct_folders(sender, option, champion_parent)?;
    log(sender, "Creating Meta");
    create_meta(sender, option, champion_parent)?;
    log(sender, "Copy Mod Files");
    copy_mod_files(sender, option, champion_parent)?;
    log(sender, "Zip Folder");
    zip_dir(sender, option, champion_parent)?;
    log(sender, "Moving zip");
    rename_and_move(sender, option, champion_parent)?;
    log(sender, "Create mod config");
    create_ltk_mod_config(sender, option, champion_parent)?;
    log(sender, "Modifying ltk library");
    modify_library(sender, option, champion_parent)?;
    log(sender, "Removing mod from overlay");
    remove_overlay(sender, option, champion_parent)?;
    log(sender, "Exported to ltk");
    Ok(())
}
/// creates needed folders
fn create_proejct_folders(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    create_dir_all(format!(r"{}\0WADS\{}.wad.client\META",option.get_project_path(), champion_parent))
        .inspect_err(|e| {log(sender, format!("Could not create META folder: {}", e))})?;
    create_dir_all(format!(r"{}\0WADS\{}.wad.client\WAD\{}.wad.client",option.get_project_path(), champion_parent, champion_parent))
        .inspect_err(|e| {log(sender, format!("Could not create WAD folder: {}", e))})?;
    Ok(())
}
/// creates META json
fn create_meta(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut file = File::create(format!(r"{}\0WADS\{}.wad.client\META\info.json", option.get_project_path(), champion_parent))
        .inspect_err(|e| {log(sender, format!("Could not create info.json: {}", e))})?;
    let text = format!(r#"{{
    "Author": "AutomatedSkinSizeConverter",
    "Description": "",
    "Heart": "",
    "Home": "",
    "Name": "Giant {}",
    "Version": "1.0"
}}"#, champion_parent);
    file.write_all(text.as_ref()).inspect_err(|e| {log(sender, format!("Could not write into info.json: {}", e))})?;
    Ok(())
}
///copies the mod files to the 0WADS folder
fn copy_mod_files(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    let source = Path::new(option.get_project_path()).join(format!("{}.wad.client", champion_parent ));
    let destination = Path::new(option.get_project_path()).join("0WADS")
        .join(format!("{}.wad.client",champion_parent)).join("WAD").join(format!("{}.wad.client",champion_parent));
    for file in WalkDir::new(&source) {
        let file = file.inspect_err(|e| {log(sender, format!("File does not exist: {}", e))})?;
        let source_path = file.path();

        let relative_path = source_path.strip_prefix(&source)
            .inspect_err(|e| {log(sender, format!("No prefix in filepath: {}", e))})?;
        let destination_path = destination.join(relative_path);

        if !file.file_type().is_dir() {
            if let Some(parent) = destination_path.parent() {
                create_dir_all(parent).inspect_err(|e| {log(sender, format!("Could not create target folder {:?}: {}", parent, e))})?;
            }
            fs::copy(source_path, &destination_path).inspect_err(|e| {log(sender, format!("Could not copy file {:?}: {}", source_path, e))})?;
        }
    }
    Ok(())
}
/// creates a zip that contains all files from the mod
fn zip_dir(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    let folder = format!(r"{}\0WADS\{}.wad.client", option.get_project_path(), champion_parent);
    let folder_path = Path::new(&folder);

    let zip_path = PathBuf::from(format!("{}.zip", folder));
    let zip_file = File::create(&zip_path).inspect_err(|e| {log(sender, format!("Could create zip file: {}", e))})?;

    let mut zip = zip::ZipWriter::new(zip_file);

    let options: FileOptions<'_, ()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(folder_path).inspect_err(|e| {log(sender, format!("No prefix in file {:?}: {}", path, e))})?;

        // Convert path to string and replace backslashes with forward slashes
        let name_str = name.to_string_lossy().replace(r"\", r"/");

        if path.is_file() {
            zip.start_file(&name_str, options).inspect_err(|e| {log(sender, format!("Could create file in zip archive: {}", e))})?;

            let mut f = File::open(path).inspect_err(|e| {log(sender, format!("Could not open file {:?}: {}", path, e))})?;
            buffer.clear();
            f.read_to_end(&mut buffer).inspect_err(|e| {log(sender, format!("Error while reading file {:?}: {}", f, e))})?;

            zip.write_all(&buffer).inspect_err(|e| {log(sender, format!("Could not write to file {:?}: {}", f, e))})?;
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(&name_str, options).inspect_err(|e| {log(sender, format!("Could not add directory {}to zip: {}", name_str, e))})?;
        }
    }

    zip.finish().inspect_err(|e| {log(sender, format!("Could not finish zip: {}" ,e))})?;
    fs::remove_dir_all(folder_path).inspect_err(|e| {log(sender, format!("Could clear directory {:?}: {}", folder_path, e))})?;
    Ok(())
}

/// Rename the zip to fantom and move it to the ltk mod folder
fn rename_and_move(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: &str) -> Result<(), Box<dyn std::error::Error>>{
    let source = format!(r"{}\0WADS\{}.wad.client.zip", option.get_project_path(), champion_parent);
    let destination = format!(r"{}\archives\Giant {}.fantome", option.get_ltk_path() ,champion_parent);
    fs::copy(&source, &destination).inspect_err(|e| {log(sender, format!("Could not copy zip {:?}: {}", source, e))})?;
    fs::remove_file(&source).inspect_err(|e| {log(sender, format!("Could not remove zip {:?}: {}", source, e))})?;
    Ok(())
}
//creates the config file ltk needs
fn create_ltk_mod_config(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: & str) -> Result<(), Box<dyn std::error::Error>>{
    create_dir_all(format!(r"{}\mods\Giant {}", option.get_ltk_path(), champion_parent))
        .inspect_err(|e| {log(sender, format!("Could not create dir for {:?}: {}", champion_parent, e))})?;
    let mut file = File::create(format!(r"{}\mods\Giant {}\mod.config.json", option.get_ltk_path(), champion_parent))
        .inspect_err(|e| {log(sender, format!("Could not create config file for {:?}: {}", champion_parent, e))})?;
    let text =  format!(r#"{{
  "name": "giant-{}",
  "display_name": "Giant {}",
  "version": "1.0",
  "description": "",
  "authors": [
    "AutomatedSkinSizeConverter"
  ],
  "layers": [
    {{
      "name": "base",
      "priority": 0,
      "description": "Base layer of the mod"
    }}
  ]
}}"#, champion_parent, champion_parent);
    file.write(text.as_ref()).inspect_err(|e| {log(sender, format!("Could not create config file for {:?}: {}", champion_parent, e))})?;
    Ok(())
}
///modifies the library, so the ltk knows there is a new mod
fn modify_library(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: & str) -> Result<(), Box<dyn std::error::Error>>{
    let filepath = format!(r"{}\library.json", option.get_ltk_path());

    let mut data = String::new();
    let f = File::open(&filepath)
        .inspect_err(|e| {log(sender, format!("Could not open file {:?}: {}", filepath, e))})?;
    let mut br = BufReader::new(&f);
    br.read_to_string(&mut data)
        .inspect_err(|e| {log(sender, format!("Could not read file {:?}: {}", filepath, e))})?;
    let mut parsed: Value = serde_json::from_str(&data)
        .inspect_err(|e| {log(sender, format!("Could not create json from file {:?}: {}", champion_parent, e))})?;

    let mods = parsed.get_mut("mods").unwrap(); //todo
    let array = mods.as_array_mut().unwrap(); //todo
    let key = format!("Giant {}", champion_parent);

    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();

    if let Some(element) = array.iter_mut().find(|e| {
        e["id"].as_str().map_or(false, |id| id == key)
    }) {
        element["installedAt"] = json!(timestamp);
    }
    else {
        let id = format!("Giant {}", champion_parent);
        let new_entry = json!({
                "id": id,
                "installedAt": timestamp,
                "format": "fantome"
            });
        array.push(new_entry);
    }

    let array = &mut parsed.get_mut("folders").unwrap().as_array_mut().unwrap(); //todo

    let new_mod_id = format!("Giant {}", champion_parent);

    if let Some(root) = array.iter_mut().find(|e| e["id"] == "root") {
        if let Some(mod_ids) = root["modIds"].as_array_mut() {
            if !mod_ids.iter().any(|id| id.as_str() == Some(&new_mod_id)) {
                mod_ids.push(json!(new_mod_id));
            }
        }
    }

    fs::write(&filepath, serde_json::to_string_pretty(&parsed)
        .inspect_err(|e| {log(sender, format!("String json of file {:?}: {}", filepath, e))})?)
    .inspect_err(|e| {log(sender, format!("Could not write to file {:?}: {}", filepath, e))})?;
    Ok(())
}
/// Cleanup Overlay, so mod gets reloaded
fn remove_overlay(sender:&Sender<WorkerMessage>, option: &Options, champion_parent: & str) -> Result<(), Box<dyn std::error::Error>>{
    let filepath = format!(r"{}\profiles\default\overlay.json", option.get_ltk_path());

    let mut data = String::new();
    let f = File::open(&filepath)
        .inspect_err(|e| {log(sender, format!("Could not open file {:?}: {}", filepath, e))})?;
    let mut br = BufReader::new(&f);
    br.read_to_string(&mut data)
        .inspect_err(|e| {log(sender, format!("Could not read file {:?}: {}", filepath, e))})?;
    let mut parsed: Value = serde_json::from_str(&data)
        .inspect_err(|e| {log(sender, format!("Could not parse file to json {:?}: {}", filepath, e))})?;

    let enabled = parsed.get_mut("enabledMods").unwrap(); //todo
    let array = enabled.as_array_mut().unwrap(); //todo
    let key = format!("Giant {}", champion_parent);

    array.retain(|v| v.as_str() != Some(key.as_str()));

    fs::write(&filepath, serde_json::to_string_pretty(&parsed).
        inspect_err(|e| {log(sender, format!("Could not parse json to string {:?}: {}", filepath, e))})?)
        .inspect_err(|e| {log(sender, format!("Could not write to file {:?}: {}", filepath, e))})?;
    Ok(())
}