use std::collections::HashMap;
use crate::converter::skin_rescaler::rescale_skins;
use crate::converter::file_converter::{bin_to_json, json_to_bin};
use crate::cdtb::cdtb::wad_extract;
use crate::cdtb::hashes::download_hashes;
use crate::converter::export::{export_cslol, export_ltk};
use crate::data::options::Options;

pub fn control(options: &Options, download_files:bool, export_cslol_checkbox:bool, export_ltk_checkbox: bool){
    unpack_ritobin();
    let map = get_champions();
    get_skins();
    if download_files {
        download_hashes();
    }
    for champion in map.keys(){
        let champion_parent = get_parent();
        if download_files{
            wad_extract(&champion);
        }
        let max_skin = map.get(champion).unwrap();
        bin_to_json(options, champion, &champion_parent);
        for skin_number in 0..*map.get(champion).unwrap(){ //todo skins
            rescale_skins(champion, &champion_parent, skin_number);
        }

        json_to_bin(champion, &champion_parent);

        if export_cslol_checkbox{
            export_cslol(&champion_parent);
        }
        if export_ltk_checkbox{
            export_ltk(&champion_parent);
        }
    }
}


fn get_champions() -> HashMap<String, u64>{
    //todo

}
fn get_skins() {
    // read options.txt (later in set)
    // todo
}
fn get_parent() -> String {
    todo!()
}

