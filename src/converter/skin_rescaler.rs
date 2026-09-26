use std::collections::VecDeque;
use std::fs;
use std::fs::{read_to_string, File};
use std::io::{BufReader, Read};
use std::sync::mpsc::Sender;
use serde_json::{json, Value};
use crate::converter::control::control;
use crate::converter::file_converter::{bin_to_json, json_to_bin};
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::lux_champion::LuxChampion;
use crate::data::options::Options;


/// reads a
pub fn rescale_skins(sender:&Sender<WorkerMessage>,options: &Options, champion:&str, champion_parent:&str, skin: u16, scale: f32, current_set: &str) -> Result<(), Box<dyn std::error::Error>>{
    if champion == "lux" && skin == 7 {
        // rescale_lux();
        Ok(()) //todo
    } else if champion == "sona" && skin == 6 {
        //todo get scale 
        bin_to_json(sender, options, "sonadjgenre01")?;
        bin_to_json(sender, options, "sonadjgenre02")?;
        bin_to_json(sender, options, "sonadjgenre03")?;
        rescale_normal(sender, options, "sonadjgenre01", "", 6, 2.)?; //todo
        rescale_normal(sender, options, "sonadjgenre02", "", 6, 2.)?; //todo
        rescale_normal(sender, options, "sonadjgenre03", "", 6, 2.)?; //todo
        json_to_bin(sender, options, "sonadjgenre01")?;
        json_to_bin(sender, options, "sonadjgenre02")?;
        json_to_bin(sender, options, "sonadjgenre03")?;
        Ok(())
    } else {
        rescale_normal(sender,options, champion, champion_parent, skin, scale)
    }
}

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

fn rescale_lux(sender:&Sender<WorkerMessage>, options: &Options, current_set: &str) -> Result<(), Box<dyn std::error::Error>>{ //todo
    let a = read_lux(sender, options, current_set)?;
    write_lux();
    Ok(())
}
fn read_lux(sender:&Sender<WorkerMessage>, options: &Options, current_set: &str) -> Result<(), Box<dyn std::error::Error>>{
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
            // if that fails do idk?
        }

    }


    Ok(())
}
fn write_lux(){

}