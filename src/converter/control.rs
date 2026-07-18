use std::collections::HashMap;
use std::hash::Hash;
use std::sync::mpsc::Sender;
use crate::converter::skin_rescaler::rescale_skins;
use crate::converter::file_converter::{bin_to_json, json_to_bin};
use crate::cdtb::cdtb::wad_extract;
use crate::cdtb::hashes::download_hashes;
use crate::converter::export::{export_cslol, export_ltk};
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::options::Options;
/// todo
pub fn control(sender:&Sender<WorkerMessage>, options: &Options, download_files:bool, export_cslol_checkbox:bool, export_ltk_checkbox: bool){
    unpack_ritobin();
    println!("Hi");
    let map = get_champions();
    get_skins();
    println!("{:?}", map);
    if download_files {
        if download_hashes(options, sender).is_ok(){ //todo
            log(sender, "Hashes downloaded and written successfully");
        } else{
            //todo check if hashes exist, if they do, continue anyways but inform user, else return
        }

    }
    for champion in map.keys(){
        let champion_parent = get_parent();
        if download_files{
            wad_extract(options, sender, &champion).expect("TODO: panic message"); //todo
        }
        let max_skin = map.get(champion).unwrap(); //todo
        bin_to_json(options, champion);
        for skin_number in 0..*map.get(champion).unwrap(){ // todo maybe implement a more sophisticated way of doing it
            rescale_skins(champion, &champion_parent, skin_number);
        }

        json_to_bin(options, champion, &champion_parent).expect("TODO: panic message");

        if export_cslol_checkbox{
            export_cslol(&champion_parent);
        }
        if export_ltk_checkbox{
            export_ltk(&champion_parent);
        }
    }
}
///todo
fn unpack_ritobin(){

}
/// todo
fn get_champions() -> HashMap<String, u16>{
    let mut map:HashMap<String, u16> = HashMap::new();
    map.insert("ahri".to_string(), 0);
    return map
}
///todo
fn get_skins() {
    // read options.txt (later in set)
    // todo
}
///todo
fn get_parent() -> String {
    // todo!()
    return "".to_string()
}

