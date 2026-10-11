use std::collections::VecDeque;
use std::fs;
use std::fs::{read_to_string, File};
use std::io::{BufReader, Read};
use std::sync::mpsc::Sender;
use serde_json::{json, Value};
use crate::converter::file_converter::{bin_to_json, json_to_bin};
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::legendary_champion::LuxChampion;
use crate::data::legendary_champion::SonaChampion;
use crate::data::options::Options;


/// reads a
pub fn rescale_skins(sender:&Sender<WorkerMessage>,options: &Options, champion:&str, champion_parent:&str, skin: u16, scale: f32, current_set: &str) -> Result<(), Box<dyn std::error::Error>>{
    if champion == "lux" && skin == 7 {
        rescale_lux(sender, options, current_set)?;
        Ok(()) //todo
    } else if champion == "sona" && skin == 6 {
        let sona_champ = read_sona(sender, options, current_set)?;
        bin_to_json(sender, options, "sonadjgenre01")?;
        bin_to_json(sender, options, "sonadjgenre02")?;
        bin_to_json(sender, options, "sonadjgenre03")?;
        rescale_normal(sender, options, "sonadjgenre01", "", 6, sona_champ.get_genre1())?;
        rescale_normal(sender, options, "sonadjgenre02", "", 6, sona_champ.get_genre2())?;
        rescale_normal(sender, options, "sonadjgenre03", "", 6, sona_champ.get_genre3())?;
        json_to_bin(sender, options, "sonadjgenre01")?;
        json_to_bin(sender, options, "sonadjgenre02")?;
        json_to_bin(sender, options, "sonadjgenre03")?;
        Ok(())
    } else {
        rescale_normal(sender,options, champion, champion_parent, skin, scale)
    }
}
/// scales a normal champion
fn rescale_normal(sender:&Sender<WorkerMessage>,options: &Options, champion:&str, champion_parent:&str, skin: u16, scale: f32) -> Result<(), Box<dyn std::error::Error>>{
    let filepath = format!(r"{}\0WADS\data\characters\{}\skins\skin{}.json", options.get_project_path(), champion, skin);

    let mut data = String::new();
    let f = File::open(&filepath).inspect_err(|e| { log(sender, format!("Could not open file: {}, {}", &filepath, e)) })?;
    let mut br = BufReader::new(&f);
    br.read_to_string(&mut data).inspect_err(|e| { log(sender, format!("Could not read data: {}", e)) })?;
    let mut parsed: Value = serde_json::from_str(&data).inspect_err(|e| { log(sender, format!("Could convert data to json: {}", e)) })?;

    //change and write data
    let entries = parsed.get_mut("entries").ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing entries item"))?;
    traverse(sender, entries, vec![&format!("characters/{}/skins/skin{}",champion, skin), "skinmeshproperties","skinscale"].into(), scale, champion, skin)?;
    fs::write(&filepath, serde_json::to_string_pretty(&parsed).inspect_err(|e| { log(sender, format!("Could create string: {}", e)) })?).inspect_err(|e| { log(sender, format!("Could not write to file: {}", e)) })?;
    Ok(())
}

/// Traverses the json and changes a number according scale, if the last path part does not exist it creates it
///
/// # Arguments
/// * value: the json that should be traversed
/// * path: the path that should get traversed
/// + scale: the size increase of that Champion
fn traverse(sender:&Sender<WorkerMessage>, value: & mut Value, mut path:VecDeque<&str>, scale:f32,  champion:&str, skin: u16) -> Result<(), Box<dyn std::error::Error>>{
    let json_array = value["value"]["items"].as_array_mut().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing items array"))?;
    let key = path.pop_front();
    if key.is_none(){
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData, "No key"
        ).into());
    }
    for element in json_array.iter_mut(){
        if element["key"].as_str().is_some() {
            if element["key"].as_str().unwrap().to_lowercase() == key.unwrap().to_lowercase() {
                if path.is_empty() {
                    element["value"] = json!(scale);
                    return Ok(());
                }
                return traverse(sender, element, path, scale, champion, skin);
            }
        } else if element["key"].as_i64().is_some(){
            if element["key"].as_i64().unwrap().to_string() == key.unwrap() {
                if path.is_empty() {
                    element["value"] = json!(scale);
                    return Ok(());
                }
                return traverse(sender, element, path, scale, champion, skin);
            }
        }
    }
    if path.is_empty(){
        log(sender, "scale doesnt exist in json, adding");
        let new_entry = json!({
            "key": "skinScale",
            "type": "f32",
            "value": scale
        });
        json_array.push(new_entry);
        Ok(())
    } else{
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData, format!("JSON not complete: {}, {}", champion, skin)
        ).into())
    }
}
/// reads the file for elementalist lux and then rescales her
fn rescale_lux(sender:&Sender<WorkerMessage>, options: &Options, current_set: &str) -> Result<(), Box<dyn std::error::Error>>{
    let a = read_lux(sender, options, current_set)?;
    write_lux();
    Ok(())
}
///reads the file for elementalist lux
fn read_lux(sender:&Sender<WorkerMessage>, options: &Options, current_set: &str) -> Result<(LuxChampion), Box<dyn std::error::Error>>{
    let file = format!(r"{}\0PutSizeOptionFilesHere\{}\luxlegendary.txt", options.get_project_path(), current_set);
    let text = read_to_string(file).inspect_err(|e| { log(sender, format!("Could not read options file to file, trying lux options: {}", e)) });
    let mut default: Option<f32> = None;
    let mut champ = LuxChampion::new();
    if let Ok(text) = text {
        for line in text.lines() {
            if line.starts_with("lux:"){
                default = Some(line.split_once(":").unwrap().1.trim().parse().unwrap()); //todo unwrap etc
            } else if line.starts_with("magma:") {
                champ.set_magma(sender, line.split_once(":").unwrap().1.trim().parse().unwrap()); //todo unwrap etc
            } else if line.starts_with("dark:") {
                champ.set_dark(sender, line.split_once(":").unwrap().1.trim().parse().unwrap()); //todo unwrap etc
            } else if line.starts_with("mystic:") {
                champ.set_mystic(sender, line.split_once(":").unwrap().1.trim().parse().unwrap()); //todo unwrap etc
            } else if line.starts_with("ice:"){
                champ.set_ice(sender, line.split_once(":").unwrap().1.trim().parse().unwrap()); //todo unwrap etc
            } else if line.starts_with("storm:"){
                champ.set_storm(sender, line.split_once(":").unwrap().1.trim().parse().unwrap());//todo unwrap etc
            } else if line.starts_with("light:"){
                champ.set_light(sender, line.split_once(":").unwrap().1.trim().parse().unwrap());//todo unwrap etc
            } else if line.starts_with("water:"){
                champ.set_water(sender, line.split_once(":").unwrap().1.trim().parse().unwrap());//todo unwrap etc
            } else if line.starts_with("fire:"){
                champ.set_fire(sender, line.split_once(":").unwrap().1.trim().parse().unwrap());//todo unwrap etc
            } else if line.starts_with("air:"){
                champ.set_air(sender, line.split_once(":").unwrap().1.trim().parse().unwrap());//todo unwrap etc
            } else if line.starts_with("nature:"){
                champ.set_nature(sender, line.split_once(":").unwrap().1.trim().parse().unwrap());//todo unwrap etc
            }
        }
        if default.is_some(){
            champ.set_rest(default.unwrap());
        } else{
            // todo read default from lux, then from set
        }
    } else{
        let file = format!(r"{}\0PutSizeOptionFilesHere\{}\lux.txt", options.get_project_path(), current_set);
        let text = read_to_string(file).inspect_err(|e| { log(sender, format!("Could not read options file to file, trying set default options: {}", e)) });
        if let Ok(text) = text {
            //todo read default from lux file
        } else{
           //todo read default from set,
            // if that fails do idk? I guess abort?
        }

    }


    Ok(champ)
}

/// Reads the size options for DJ Sona from the selected set's `sonalegendary.txt`.
fn read_sona(sender: &Sender<WorkerMessage>, options: &Options, current_set: &str) -> Result<SonaChampion, Box<dyn std::error::Error>> {
    let file = format!(r"{}\0PutSizeOptionFilesHere\{}\sonalegendary.txt", options.get_project_path(), current_set);
    let mut champ = SonaChampion::new();
    let mut has_base = false;
    match read_to_string(&file) {
        Ok(text) => {
            for (line_number, line) in text.lines().enumerate() {
                let Some((key, value)) = line.split_once(':') else {
                    continue;
                };
                let value: f32 = value.trim().parse().map_err(|e| {
                    log(sender, format!("Invalid Sona size on line {}: {}", line_number + 1, e));
                    e
                })?;
                match key.trim().to_ascii_lowercase().as_str() {
                    "sona" | "base" => {
                        champ.set_base(value);
                        has_base = true;
                    }
                    "1" => champ.set_genre1(value),
                    "2" => champ.set_genre2(value),
                    "3" => champ.set_genre3(value),
                    _ => log(sender, format!("Ignoring unknown Sona legendary option on line {}: {}", line_number + 1, key.trim())),
                }
            }
        }
        Err(e) => {
            log(sender, format!("Could not read Sona legendary options {}: {}", file, e));
        }
    }

    // If the legendary file is missing or has no base value, use the regular
    // Sona file's skin 6 override, then its champion default.
    if !has_base {
        let sona_file = format!(r"{}\0PutSizeOptionFilesHere\{}\sona.txt", options.get_project_path(), current_set);
        match read_to_string(&sona_file) {
            Ok(text) => {
                let mut skin6 = None;
                let mut default = None;
                for line in text.lines() {
                    let Some((key, value)) = line.split_once(':') else { continue; };
                    let value: f32 = value.trim().parse().map_err(|e| {
                        log(sender, format!("Invalid Sona option in {}: {}", sona_file, e));
                        e
                    })?;
                    match key.trim().to_ascii_lowercase().as_str() {
                        "6" => skin6 = Some(value),
                        "sona" => default = Some(value),
                        _ => {}
                    }
                }
                if let Some(value) = skin6.or(default) {
                    champ.set_base(value);
                    has_base = true;
                }
            }
            Err(e) => log(sender, format!("Could not read Sona options {}: {}", sona_file, e)),
        }
    }

    // Finally try the shared default options file, whose fallback key is Scale:.
    if !has_base {
        let options_file = format!(r"{}\0PutSizeOptionFilesHere\{}\0Options.txt", options.get_project_path(), current_set);
        let text = read_to_string(&options_file).inspect_err(|e| {
            log(sender, format!("Could not read default options {}: {}", options_file, e));
        })?;
        let mut default = None;
        for (line_number, line) in text.lines().enumerate() {
            if let Some((key, value)) = line.split_once(':') {
                if key.trim().eq_ignore_ascii_case("Scale") {
                    default = Some(value.trim().parse::<f32>().map_err(|e| {
                        log(sender, format!("Invalid Scale value in {} on line {}: {}", options_file, line_number + 1, e));
                        e
                    })?);
                    break;
                }
            }
        }
        let value = default.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("No Scale: default found in {}", options_file)))?;
        champ.set_base(value);
    }
    champ.fill_missing_from_base();
    Ok(champ)
}
fn write_lux(){

}
