use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::mpsc::Sender;
use serde_json::{json, Value};
use crate::converter::control::control;
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::options::Options;

/// reads a
pub fn rescale_skins(sender:&Sender<WorkerMessage>,options: &Options, champion:&str, champion_parent:&str, skin: u16, scale: f32) -> Result<(), Box<dyn std::error::Error>>{
    if champion == "lux" && skin == 7 {
        rescale_lux();
        Ok(()) //todo
    } else if champion == "sona" && skin == 6 {
        Ok(())
        // control(sender, options, download_files, "sonadjgenre01")
        // todo!() //ist djsona nicht einfach djsona?
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
    traverse(sender, entries, vec![&format!("characters/{}/skins/skin{}",champion, skin), "skinmeshproperties","skinscale"].into(), scale)?;
    fs::write(&filepath, serde_json::to_string_pretty(&parsed).inspect_err(|e| { log(sender, format!("Could create string: {}", e)) })?).inspect_err(|e| { log(sender, format!("Could not write to file: {}", e)) })?;
    Ok(())
}

/// Traverses the json and changes a number according scale, if the last path part does not exist it creates it
///
/// # Arguments
/// * value: the json that should be traversed
/// * path: the path that should get traversed
/// + scale: the size increase of that Champion
fn traverse(sender:&Sender<WorkerMessage>, value: & mut Value, mut path:VecDeque<&str>, scale:f32) -> Result<(), Box<dyn std::error::Error>>{
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
                return traverse(sender, element, path, scale);
            }
        } else if element["key"].as_i64().is_some(){
            if element["key"].as_i64().unwrap().to_string() == key.unwrap() {
                if path.is_empty() {
                    element["value"] = json!(scale);
                    return Ok(());
                }
                return traverse(sender, element, path, scale);
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
            std::io::ErrorKind::InvalidData, "JSON not complete"
        ).into())
    }
}

fn rescale_lux(){ //todo
    
}