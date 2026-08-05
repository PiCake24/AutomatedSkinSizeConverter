use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::sync::mpsc::Sender;
use crate::converter::skin_rescaler::rescale_skins;
use crate::converter::file_converter::{bin_to_json, json_to_bin};
use crate::cdtb::cdtb::wad_extract;
use crate::cdtb::hashes::download_hashes;
use crate::converter::export::{export_cslol, export_ltk};
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::options::{get_ritobin_path, Options};
use crate::data::champion::{Champion, SkinScale};

/// todo
pub fn control(sender:&Sender<WorkerMessage>, options: &Options, download_files:bool, export_cslol_checkbox:bool, export_ltk_checkbox: bool, current_set: &str){
    //todo create project dir
    //todo read options actually
    unpack_ritobin(options);
    let champions = get_champions(sender, options, current_set );
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
            get_all_skins(sender, options, &mut champion);
        }
        let champion_parent = get_parent(champion.get_name().to_owned());
        // todo if no max skin set
        // let max_skin = map.get(&champion).unwrap(); //todo

        if download_files{
            //todo clean 0WADS/data
            wad_extract(options, sender, champion.get_name()).expect("TODO: panic message"); //todo
        }
        get_scale(sender,options, &mut champion, current_set);
        bin_to_json(sender, options, champion.get_name()).expect("TODO: panic message"); //todo
        for skin in champion.get_skins(){
            let skin_number = skin.get_skin();
            let scale = skin.get_scale();
            rescale_skins(sender, champion.get_name(), &champion_parent, skin_number, scale).expect("TODO: panic message");
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
fn get_champions(sender:&Sender<WorkerMessage>, options: &Options, current_set: &str) -> Vec<Champion>{
    let mut champions:Vec<Champion> = Vec::new();
    let f = File::open(format!(r"{}\0PutSizeOptionFilesHere\{}\0Options.txt", options.get_project_path(), current_set)).unwrap(); //todo

    let br = BufReader::new(&f);

    for line in br.lines(){
        let result = line.unwrap();


        let (champion_name, skins) = match result.split_once(' ') {
            Some((first, second)) => (first.to_string(), second.to_string()),
            None => (result.to_string(), String::new())
        };
        let mut champion = Champion::new(&champion_name);
        champion.set_skins(parse_skins(&skins));
        champions.push(champion);
    }
    println!("{:?}", champions);
    panic!();
    champions
}

fn parse_skins(skins: &String) -> Vec<SkinScale> {
    let skins = skins.trim();
    if skins.is_empty() {
        //todo log
        return Vec::new();
    }
    let mut set: HashSet<u16> = HashSet::new();

    for part in skins.split('|') {
        let part = part.trim();
        if part.is_empty() {
            //todo log
            continue;
        }
        match part.split_once('-') {
            Some((start, end)) => {
                let start: u16 = start.trim().parse().expect("invalid range start"); //todo
                let end: u16 = end.trim().parse().expect("invalid range end"); //todo
                for n in start..=end {
                    set.insert(n);
                }
            }
            None => {
                let n: u16 = part.parse().expect("invalid skin number"); //todo
                set.insert(n);
            }
        }
    }

    set.into_iter().map(SkinScale::new).collect()
}

/// gets all skins for a Champion that does not have their skin-number defined
fn get_all_skins(sender:&Sender<WorkerMessage>, options: &Options, champion: &mut Champion) {
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

    for skin in 0..number_of_skins{
        champion.add_skins(skin)
    }
}
/// extracts the Champion parent of a Champion
/// swaindemonform for example should return swain
/// topaz_swain should also return swain
fn get_parent(champion: String) -> String {
    //prefix
    if champion.contains("_"){
        let parent = champion.split("_");
        return parent.collect::<Vec<&str>>()[1].to_string()
    }
    let champion_parent = &champion;
    let file_path = &format!(r"D:\Riot Games\League of Legends\Game\DATA\FINAL\Champions\{}.wad.client",
                             champion_parent);
    let result = fs::exists(Path::new(file_path));
    if !result.unwrap(){
        return get_parent(champion.split_at(champion.len()-1).0.to_string())
    }
    champion_parent.to_string()
}
/// gets the scale for each skin of a champion
pub fn get_scale(sender:&Sender<WorkerMessage>, option: &Options, champion:&mut Champion, current_set: &str)  {
    let path = format!(
        r"{}\0PutSizeOptionFilesHere\{}\{}.txt",
        option.get_project_path(),
        current_set,
        champion.get_name()
    );

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            log(sender, format!("Could not open file: {}", e));
            return
        },
    };

    let mut data = String::new();

    if let Err(e) = BufReader::new(file).read_to_string(&mut data) {
        log(sender, format!("Could not read file: {}", e));
        return;
    }

    let mut default :f32 = 2.0;

    for row in data.lines() {
        let parts: Vec<&str> = row.splitn(2, ':').collect();

        if parts.len() != 2 {
            log(sender, format!("Entry malformed: {:?}", parts));
            continue;
        }

        let key = parts[0].trim();
        let value: f32 = match parts[1].trim().parse() {
            Ok(v) => v,
            Err(e) => { log(sender, format!("Entry malformed (not a number): {}", e));
                continue; },
        };

        match key.parse::<u16>() {
            Ok(skin_id) => {
                for skin in champion.get_skins_mut() {
                    if skin.get_skin() == skin_id {
                        skin.set_scale(value);
                        skin.set_changed();
                    }
                }
            }
            Err(_) => default = value,
        }
    }

    // Apply default champion scale to skins without explicit values
    for skin in champion.get_skins_mut() {
        if !skin.get_changed() {
            skin.set_scale(default);
        }
    }
}

