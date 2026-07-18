use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use chrono::{Utc, DateTime};
use serde_json::{json, Value};
use walkdir::WalkDir;
use zip::write::FileOptions;

/// todo
pub fn export_cslol(champion_parent: &str){
    //todo
    // create wad archive
    // create folder in cslol installed
    // move wad archive
    // create meta
}

/// todo
// todo change println to log
pub fn export_ltk(champion_parent: &str){
    println!("creating folder");
    create_folders(champion_parent);
    println!("creating meta");
    create_meta(champion_parent);
    println!("copy mod files");
    copy_mod_files(champion_parent);
    println!("zip dir");
    zip_dir(champion_parent);
    println!("rename and remov");
    rename_and_move(champion_parent);
    println!("create mod conf");
    create_ltk_mod_config(champion_parent);
    println!("mod library");
    modify_library(champion_parent);
    println!("Removing from overlay");
    remove_overlay(champion_parent);
    println!("Exported to ltk");
}
/// creates needed folders
fn create_folders(champion_parent: &str){
    fs::create_dir_all(format!(r"D:\wad5\{}.wad.client\META", champion_parent)).expect("Could not create META folder");
    fs::create_dir_all(format!(r"D:\wad5\{}.wad.client\WAD\{}.wad.client", champion_parent, champion_parent)).expect("Could not create WAD folder");
}
/// creates META json
fn create_meta(champion_parent: &str){
    let mut file = fs::File::create(format!(r"D:\wad5\{}.wad.client\META\info.json", champion_parent)).expect("Could not create info.json");
    let text = format!(r#"{{
    "Author": "UNKNOWN",
    "Description": "",
    "Heart": "",
    "Home": "",
    "Name": "Supermod {}",
    "Version": "1.0"
}}"#, champion_parent);
    file.write(text.as_ref()).expect("Could not write info.json");
}

fn copy_mod_files(champion_parent: &str){
    let source = Path::new("D:\\wad5\\").join(champion_parent);
    let destination = Path::new("D:\\wad5\\").join(format!("{}.wad.client",champion_parent)).join("WAD").join(format!("{}.wad.client",champion_parent));
    for file in WalkDir::new(&source) {
        let file = file.expect("File does not exist");
        let source_path = file.path();

        let relative_path = source_path.strip_prefix(&source).unwrap();
        let destination_path = destination.join(relative_path);

        if !file.file_type().is_dir() {
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(source_path, &destination_path).unwrap();
        }
    }
}

fn zip_dir(champion_parent: &str) {
    let folder = format!(r"D:\wad5\{}.wad.client", champion_parent);
    let folder_path = Path::new(&folder);

    let zip_path = PathBuf::from(format!("{}.zip", folder));
    let zip_file = File::create(&zip_path).unwrap();

    let mut zip = zip::ZipWriter::new(zip_file); //todo update zip version

    let options: FileOptions<'_, ()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(folder_path).unwrap();

        // Convert path to string and replace backslashes with forward slashes
        let name_str = name.to_string_lossy().replace(r"\", r"/");

        if path.is_file() {
            zip.start_file(&name_str, options).unwrap();

            let mut f = File::open(path).unwrap();
            buffer.clear();
            f.read_to_end(&mut buffer).unwrap();

            zip.write_all(&buffer).unwrap();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(&name_str, options).unwrap();
        }
    }

    zip.finish().unwrap();
    fs::remove_dir_all(folder_path).unwrap();
}

/// Rename the zip to fantom and move it to the ltk mod folder
fn rename_and_move(champion_parent: &str){
    let source = format!(r"D:\wad5\{}.wad.client.zip", champion_parent);
    let destination = format!(r"C:\mods\archives\Supermod {}.fantome", champion_parent);
    fs::copy(&source, &destination).unwrap();
    fs::remove_file(&source).unwrap();
}
fn create_ltk_mod_config(champion_parent: & str){
    fs::create_dir_all(format!(r"C:\mods\mods\Supermod {}", champion_parent));
    let mut file = fs::File::create(format!(r"C:\mods\mods\Supermod {}\mod.config.json", champion_parent)).expect("Could not create info.json");
    let text =  format!(r#"{{
  "name": "supermod-{}",
  "display_name": "Supermod {}",
  "version": "1.0",
  "description": "",
  "authors": [
    "UNKNOWN"
  ],
  "layers": [
    {{
      "name": "base",
      "priority": 0,
      "description": "Base layer of the mod"
    }}
  ]
}}"#, champion_parent, champion_parent);
    file.write(text.as_ref());
}
fn modify_library(champion_parent: & str){
    let filepath = r"C:\mods\library.json";

    let mut data = String::new();
    let f = File::open(&filepath).expect("File not available");
    let mut br = BufReader::new(&f);
    br.read_to_string(&mut data).expect("Should be able to read to string");
    let mut parsed: Value = serde_json::from_str(&data).unwrap();

    // mods:
    let mods = parsed.get_mut("mods").unwrap();
    let array = mods.as_array_mut().unwrap();
    let key = format!("Supermod {}", champion_parent);

    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();

    if let Some(element) = array.iter_mut().find(|e| {
        e["id"].as_str().map_or(false, |id| id == key)
    }) {
        element["installedAt"] = json!(timestamp);
    }
    else {
        // Add new entry
        let id = format!("Supermod {}", champion_parent);


        let new_entry = json!({
                "id": id,
                "installedAt": timestamp,
                "format": "fantome"
            });
        array.push(new_entry);

    }

    //folders:
    let array = &mut parsed.get_mut("folders").unwrap().as_array_mut().unwrap();

    let new_mod_id = format!("Supermod {}", champion_parent);

    if let Some(root) = array.iter_mut().find(|e| e["id"] == "root") {
        if let Some(mod_ids) = root["modIds"].as_array_mut() {
            // Only push if not already present
            if !mod_ids.iter().any(|id| id.as_str() == Some(&new_mod_id)) {
                mod_ids.push(json!(new_mod_id));
            }
        }
    }

    fs::write(&filepath, serde_json::to_string_pretty(&parsed).unwrap()).expect("Could not write into file");


    // println!("{}", serde_json::to_string_pretty(&parsed["mods"]).unwrap());
}
// "C:\mods\profiles\default\overlay.json"
fn remove_overlay(champion_parent: & str){
    let filepath = r"C:\mods\profiles\default\overlay.json";

    let mut data = String::new();
    let f = File::open(&filepath).expect("File not available");
    let mut br = BufReader::new(&f);
    br.read_to_string(&mut data).expect("Should be able to read to string");
    let mut parsed: Value = serde_json::from_str(&data).unwrap();

    let enabled = parsed.get_mut("enabledMods").unwrap();
    let array = enabled.as_array_mut().unwrap();
    let key = format!("Supermod {}", champion_parent);

    array.retain(|v| v.as_str() != Some(key.as_str()));

    fs::write(&filepath, serde_json::to_string_pretty(&parsed).unwrap()).expect("Could not write into file");
}