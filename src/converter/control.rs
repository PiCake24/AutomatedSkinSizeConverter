use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::sync::mpsc::Sender;
use crate::converter::skin_rescaler::rescale_skins;
use crate::converter::file_converter::{bin_to_json, json_to_bin};
use crate::cdtb::cdtb::wad_extract;
use crate::cdtb::hashes::download_hashes;
use crate::converter::export::{export_cslol, export_ltk};
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::options::{get_ritobin_path, Options};
use crate::data::champion::Champion;

/// todo
pub fn control(sender:&Sender<WorkerMessage>, options: &Options, download_files:bool, export_cslol_checkbox:bool, export_ltk_checkbox: bool, current_set: &str){
    //todo create project dir
    //todo read options actually
    unpack_ritobin(options);
    let mut champions = get_champions();
    if download_files {
        if download_hashes(options, sender).is_ok(){
            log(sender, "Hashes downloaded and written successfully");
        } else{
            if Path::new(&get_ritobin_path(options)).exists(){
                log(sender, "Using old hashes")
            } else{
                return //todo error + log
            }
        }
    }


    for mut champion in champions{
        if champion.get_skins().is_empty(){
            champion = get_all_skins(sender, options, champion);
        }
        let champion_parent = get_parent(champion.get_name().to_owned());
        // todo if no max skin set
        // let max_skin = map.get(&champion).unwrap(); //todo

        if download_files{
            //todo clean 0WADS/data
            wad_extract(options, sender, champion.get_name()).expect("TODO: panic message"); //todo
        }
        champion = get_scale(champion);
        bin_to_json(sender, options, champion.get_name()).expect("TODO: panic message"); //todo
        for skin in champion.get_skins(){
            let skin_number = skin.get_skin();
            let scale = skin.get_scale();
            rescale_skins(sender, champion.get_name(), &champion_parent, skin_number, scale);
        }
        //todo clean .wad.client folders
        json_to_bin(sender, options, champion.get_name(), &champion_parent).expect("TODO: panic message");

        if export_cslol_checkbox{
            export_cslol(&champion_parent);
        }
        if export_ltk_checkbox{
            export_ltk(&champion_parent);
        }
    }
}
static EMBEDDED_EXE: &[u8] = include_bytes!("../../resources/ritobin_cli.exe");
/// unpacks ritobin into the 0WADS directory in the project directory
fn unpack_ritobin(options: &Options){
    //todo only create when version is not old (if there is no version, rebuild anyways and not down current version
    let path_string = &get_ritobin_path(options);
    let path = Path::new(path_string);
    if !path.exists(){
        let mut file = File::create(path).unwrap();
    file.write_all(EMBEDDED_EXE).unwrap();
    }
}
/// reads from options.txt which champions and which skins to convert
fn get_champions() -> Vec<Champion>{
    let mut champions = Vec::new();
    champions.push(Champion::new("ahri"));
    //todo also write skins array

    // let mut map:HashMap<String, u16> = HashMap::new();
    // map.insert("ahri".to_string(), 0);
    champions
}
/// gets all skins for a Champion that does not have their skin-number defined
fn get_all_skins(sender:&Sender<WorkerMessage>, options: &Options, mut champion: Champion) -> Champion{
    log(sender, "Getting number of skins");
    let mut number_of_consecutive_tries = 0;
    let mut number_of_skins = 0;
    while number_of_consecutive_tries < 50 {
        let path_string = format!(r"{}\0WADS\data\characters\{}\skins\skin{}.bin", options.get_project_path(), champion.get_name(), number_of_consecutive_tries);
        let path = Path::new(&path_string);
        if !path.exists() {
            number_of_consecutive_tries += 1;
            number_of_skins += 1;
        } else {
            number_of_skins += 1;
            number_of_consecutive_tries = 0;
        }
    }
    // subtract the 51 skins again that do not exist
    number_of_skins -= 51;
    log(sender, format!("Number of skins for {}: {}",champion.get_name(), number_of_skins));

    let all_skins: Vec<_> = (0..number_of_skins).collect();
    for skin in 0..number_of_skins{
        champion.add_skins(skin)
    }
    champion
}
/// extracts the Champion parent of a Champion
/// swaindemonform for example should return swain
/// topaz_swain should also return swain
fn get_parent(champion: String) -> String {
    // todo!()
    //der Champion befindet sich normalerweise immer ganz am anfang oder ganz am ende, also müsste man zweimal iterativ drübergehen?
    champion
}
/// gets the scale for each skin of a champion
pub fn get_scale(mut champion: Champion) -> Champion {
    const DEFAULT_SCALE: f32 = 2.0;

    // Start with defaults
    for skin in champion.get_skins_mut() {
        skin.set_scale(DEFAULT_SCALE);
    }

    let path = format!(
        r"D:\wad\0PutSizeOptionFilesHere\{}.txt",
        champion.get_name()
    );

    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return champion,
    };

    let mut data = String::new();

    if BufReader::new(file).read_to_string(&mut data).is_err() {
        return champion;
    }

    let mut default = DEFAULT_SCALE;

    for row in data.lines() {
        let parts: Vec<&str> = row.splitn(2, ':').collect();

        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim();
        let value: f32 = match parts[1].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Champion default value
        if let Err(_) = key.parse::<u16>() {
            default = value;
            continue;
        }

        let skin_id: u16 = key.parse().unwrap();

        for skin in champion.get_skins_mut() {
            if skin.get_skin() == skin_id {
                skin.set_scale(value);
            }
        }
    }

    // Apply default champion scale to skins without explicit values
    for skin in champion.get_skins_mut() {
        if skin.get_scale() == 2.0 {
            skin.set_scale(default);
        }
    }

    champion
}

