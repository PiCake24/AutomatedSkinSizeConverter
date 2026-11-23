use std::fs;
use std::fs::{read_to_string, File};
use std::io::{BufReader, Read};
use std::sync::mpsc::Sender;
use crate::cdtb::cdtb::wad_extract;
use crate::converter::file_converter::{bin_to_json, json_to_bin};
use crate::converter::main_gui::{log, WorkerMessage};
use crate::converter::skin_rescaler2::{brace_delta, rescale_normal2};
use crate::data::champion::Champion;
use crate::data::legendary_champion::LuxChampion;
use crate::data::legendary_champion::SonaChampion;
use crate::data::options::Options;


/// reads a
pub fn rescale_skins(sender:&Sender<WorkerMessage>,options: &Options, champion:&str, champion_parent:&str, skin: u16, scale: f32, current_set: &str) -> Result<(), Box<dyn std::error::Error>>{
    if champion == "lux" && skin == 7 {
        rescale_lux(sender, options, current_set)?;
        Ok(())
    } else if champion == "sona" && skin == 6 {
        let sona_champ = read_sona(sender, options, current_set)?;
        let mut genre1 = Champion::new(options, "sonadjgenre01");
        let mut genre2 = Champion::new(options, "sonadjgenre02");
        let mut genre3 = Champion::new(options, "sonadjgenre03");
        genre1.add_skins(6);
        genre2.add_skins(6);
        genre3.add_skins(6);
        wad_extract(options, sender, "sona", &mut genre1)?;
        wad_extract(options, sender, "sona", &mut genre2)?;
        wad_extract(options, sender, "sona", &mut genre3)?;
        bin_to_json(sender, options, "sonadjgenre01")?;
        bin_to_json(sender, options, "sonadjgenre02")?;
        bin_to_json(sender, options, "sonadjgenre03")?;
        rescale_normal2(sender, options, "sonadjgenre01", "", 6, sona_champ.get_genre1())?;
        rescale_normal2(sender, options, "sonadjgenre02", "", 6, sona_champ.get_genre2())?;
        rescale_normal2(sender, options, "sonadjgenre03", "", 6, sona_champ.get_genre3())?;
        json_to_bin(sender, options, &genre1, "sona")?;
        json_to_bin(sender, options, &genre2, "sona")?;
        json_to_bin(sender, options, &genre3, "sona")?;
        Ok(())
    } else {
        rescale_normal2(sender,options, champion, champion_parent, skin, scale)
    }
}

/// reads the file for elementalist lux and then rescales her
fn rescale_lux(sender:&Sender<WorkerMessage>, options: &Options, current_set: &str) -> Result<(), Box<dyn std::error::Error>>{
    let lux = read_lux(sender, options, current_set)?;
    write_lux(sender, options, lux)?;
    Ok(())
}
///reads the file for elementalist lux
fn read_lux(sender:&Sender<WorkerMessage>, options: &Options, current_set: &str) -> Result<LuxChampion, Box<dyn std::error::Error>>{
    let file = format!(r"{}\0PutSizeOptionFilesHere\{}\luxlegendary.txt", options.get_project_path(), current_set);
    let mut lux = LuxChampion::new();
    let mut has_base = false;

    match read_to_string(&file){
        Ok(text) => {
            for line in text.lines() {
                let Some((key, value)) = line.split_once(':') else {
                    continue;
                };
                let value: f32 = value.trim().parse().map_err(|e| {
                    log(sender, format!("Invalid Lux size: {}", e));
                    e
                })?;
                match key.trim().to_ascii_lowercase().as_str() {
                    "lux" => {
                        has_base = true;
                        lux.set_base(sender, value);
                    },
                    "magma" => lux.set_magma(sender, value),
                    "dark" => lux.set_dark(sender, value),
                    "mystic" => lux.set_mystic(sender, value),
                    "ice" => lux.set_ice(sender, value),
                    "storm" => lux.set_storm(sender, value),
                    "light" => lux.set_light(sender, value),
                    "water" => lux.set_water(sender, value),
                    "fire" => lux.set_fire(sender, value),
                    "air" => lux.set_air(sender, value),
                    "nature" => lux.set_nature(sender, value),
                    _ => log(sender, "Ignoring invalid option")
                }
            }
        }
        Err(e) => {
            log(sender, format!("Could not read LuxLegendary.txt: {}", e));
        }
    }

    // if file did not contain a base, read lux.txt instead in order skin 7 -> base
    if !has_base{
        let file = format!(r"{}\0PutSizeOptionFilesHere\{}\lux.txt", options.get_project_path(), current_set);
        match read_to_string(&file) {
            Ok(text) => {
                let mut skin7 = None;
                let mut default = None;
                for line in text.lines() {
                    let Some((key, value)) = line.split_once(':') else { continue; };
                    let value: f32 = value.trim().parse().map_err(|e| { //todo actually bug, we want to continue instead of abort, either maybe the other value is still valid or the default scale is valid
                        log(sender, format!("Invalid Lux option in {}: {}", file, e));
                        e
                    })?;
                    match key.trim().to_ascii_lowercase().as_str() {
                        "7" => skin7 = Some(value),
                        "sona" => default = Some(value),
                        _ => {}
                    }
                }
                if let Some(value) = skin7.or(default) {
                    lux.set_base(sender, value);
                    has_base = true;
                }
            }
            Err(e) => {
                log(sender, format!("Could not read lux.txt: {}", e));
            }
        }
    }

    // if still no base set read default option in 0Options.txt
    if !has_base{
        let file = format!(r"{}\0PutSizeOptionFilesHere\{}\0Options.txt", options.get_project_path(), current_set);
        let text = read_to_string(&file).inspect_err(|e| {
            log(sender, format!("Could not read default options {}: {}", file, e));
        })?;
        let mut default = None;
        for (line_number, line) in text.lines().enumerate() {
            if let Some((key, value)) = line.split_once(':') {
                if key.trim().eq_ignore_ascii_case("Scale") {
                    default = Some(value.trim().parse::<f32>().map_err(|e| {
                        log(sender, format!("Invalid Scale value in {} on line {}: {}", file, line_number + 1, e));
                        e
                    })?);
                    break;
                }
            }
        }
        let value = default.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("No Scale: default found in {}", file)))?;
        lux.set_base(sender, value);
    }
    Ok(lux)
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
/// writes elementalist Lux
fn write_lux(sender: &Sender<WorkerMessage>, options: &Options, lux: LuxChampion) -> Result<(), Box<dyn std::error::Error>>{
    let filepath = format!(r"{}\0WADS\data\characters\{}\skins\skin{}.py", options.get_project_path(), "lux", 7);

    // log(sender, format!("Writing into {} with size {}", &filepath, scale));


    let mut data = String::new();
    let f = File::open(&filepath).inspect_err(|e| {log(sender, format!("Could not open file {}: {}", &filepath, e))})?;
    let mut br = BufReader::new(&f);
    br.read_to_string(&mut data).inspect_err(|e| {log(sender, format!("Could not read file {}: {}", &filepath, e))})?;

    let patched = write_elements(&data, lux);

    fs::write(&filepath, &patched).inspect_err(|e| {log(sender, format!("Could not write to file {}: {}", &filepath, e))})?;

    log(sender, format!("Completed writing into {}", filepath));
    Ok(())
}
/// Writes each element of Elementalist Lux
fn write_elements(content: &str, lux: LuxChampion) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut inside_element = false;
    let mut inside_properties = false;
    let mut scale = 2.0;
    let mut bracket_count = 0;
    let mut skin_scale_set = false;
    let nl = if content.contains("\r\n") { "\r\n" } else { "\n" };

    for line in content.lines() {
        if  line.contains("0x27dd6361"){ //line.contains("Characters/Lux/Skins/Skin7") &&
            let element = line.split_once("/Gear/Lux").unwrap_or(("", "Light\"")).1.split_once("\"").unwrap().0;
            scale = lux.get_scale_from_element(element);
            inside_element = true;
            bracket_count = 1;

        }
        if inside_properties {
            bracket_count += brace_delta(line);
            if line.contains("skinScale: f32 ="){
                skin_scale_set = true;
                out.push(format!("skinScale: f32 = {}", scale));
            } else{
                out.push(line.to_string());
            }
            continue;
        }
        if inside_element {
            bracket_count += brace_delta(line);
            if line.contains("skinMeshProperties: embed ="){
                inside_properties = true
            }
        }
        if inside_properties && bracket_count == 0{
            if !skin_scale_set {
                out.push(format!("skinScale: f32 = {} \n }}", scale));
            } else {
                out.push(line.to_string());
            }
            inside_properties = false;
            inside_element = false;
            skin_scale_set = false;
            continue;
        }
        out.push(line.to_string());
    }

    let mut result = out.join(nl);
    if content.ends_with('\n') {
        result.push_str(nl);
    }
    result
}