use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use serde_json::{json, Value};

pub fn rescale_skins(champion:&str, champion_parent:&str, skin: u16){
    //todo write into file
    // todo special cases lux, djsona
    let filepath = format!(r"D:\wad5\{}\data\characters\{}\skins\skin{}.json", champion_parent, champion, skin);
    let scale = get_scale(champion, skin);

    let mut data = String::new();
    let f = File::open(&filepath).expect("File not available");
    let mut br = BufReader::new(&f);
    br.read_to_string(&mut data).expect("Should be able to read to string");
    let mut parsed: Value = serde_json::from_str(&data).unwrap();

    //change and write data
    let entries = parsed.get_mut("entries").unwrap();
    traverse(entries, vec![&format!("characters/{}/skins/skin{}",champion, skin), "skinmeshproperties","skinscale"].into(), scale);
    // print!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    fs::write(&filepath, serde_json::to_string_pretty(&parsed).unwrap()).expect("Could not write into file");
}

/// Traverses the json and changes a number according scale, if the last path part does not exist it creates it
///
/// # Arguments
/// * value: the json that should be traversed
/// * path: the path that should get traversed
/// + scale: the size increase of that champion
fn traverse(value: & mut Value, mut path:VecDeque<&str>, scale:f64){
    let json_array = value["value"]["items"].as_array_mut().unwrap();
    let key = path.pop_front();
    // println!("Key: {:?}", key);
    for element in json_array.iter_mut(){
        // println!("{}", element["key"]);
        if element["key"].as_str().is_some() {
            if element["key"].as_str().unwrap().to_lowercase() == key.unwrap() {
                // println!("Element: {}", element);
                if path.is_empty() {
                    element["value"] = Value::Number((scale as u64).into());
                    return;
                }
                return traverse(element, path, scale);
            }
        } else if element["key"].as_i64().is_some(){
            if element["key"].as_i64().unwrap().to_string() == key.unwrap() {
                // println!("Element: {}", element);
                if path.is_empty() {
                    element["value"] = Value::Number((scale as u64).into());
                    return;
                }
                return traverse(element, path, scale);
            }
        }
        // println!("Element[key]: {:?}", element["key"])
    }
    if path.is_empty(){
        println!("scale doesnt exist in json, adding");
        let new_entry = json!({
            "key": "skinScale",
            "type": "f32",
            "value": scale
        });
        json_array.push(new_entry)
    } else{
        panic!("json not complete")
    }
}

/// todo
// todo maybe I have to rework some stuff, so it doesnt crash when file does not exist :/
pub fn get_scale(champion : &str, number : u16) -> f64 {
    let mut data = String::new();
    let path_to_file = format!("{}{}{}", r"D:\wad\0PutSizeOptionFilesHere\", champion, ".txt");
    let f = File::open(path_to_file).expect("There is no size options file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Should be able to read to string");
    let mut default:f64 = 2.0;

    let rows: Vec<&str> = data.split("\r\n").collect();

    for row in rows {
        let key_value: Vec<&str> = row.splitn(2, ":").collect();
        let key: Result<u16, _> = key_value.get(0).unwrap().trim().parse();
        let mut value: Result<f64, _> = key_value.get(1).unwrap().trim().parse();

        if value.is_ok() {
            if key.is_ok() {
                if key.unwrap() == number {
                    return value.unwrap()
                }
            } else {
                default = value.unwrap();
            }
        }
    }


    default
}