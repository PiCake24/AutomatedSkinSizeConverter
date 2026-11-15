use ltk_meta::Bin;
use ltk_ritobin::{HashMapProvider, Print, print::PrintConfig};
use std::error::Error;
use std::fs;
use std::fs::{File, read_to_string};
use std::io::BufReader;
use std::sync::mpsc::Sender;
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::options::Options;

pub fn bin_to_py(sender:&Sender<WorkerMessage>, options: &Options, bin_path: &str, filename: &str) -> Result<(), Box<dyn Error>> {

    let old_name = format!(r"{}{}", bin_path, filename);
    let new_name = format!(r"{}\{}{}", bin_path, &filename[..filename.len() - 3], "py");
    log(sender, format!("Converting {} to {}", old_name, new_name));
    let mut reader = BufReader::new(File::open(&old_name)?);
    let bin = Bin::from_reader(&mut reader)?;

    let mut hashes = HashMapProvider::new();
    hashes.load_from_directory(format!(r"{}\0WADS\hashes\ritobin", options.get_project_path())); // loads hashes.bin{entries,fields,hashes,types}.txt

    let text = bin.print_with_config(PrintConfig::default().with_hashes(hashes))?;
    fs::write(new_name, text)?;

    log(sender, format!("Converted {}", old_name));
    Ok(())
}
// Parse ritobin text
pub fn py_to_bin(sender:&Sender<WorkerMessage>, py_path: &str, output_path: &str) -> Result<(), Box<dyn Error>>{

    let old_name = format!(r"{}", py_path);
    let new_name = format!(r"{}",output_path);

    log(sender, format!("Converting {} to {}", old_name, new_name));

    // let mut reader = BufReader::new(File::open(old_name).unwrap());
    let text = read_to_string(&old_name)?;

    let cst = ltk_ritobin::Cst::parse(&text);
    if !cst.errors.is_empty() {
        return Err(format!("Ritobin parse errors: {:#?}", cst.errors).into());
    }

    let bin = cst
        .build_bin(&text)
        .into_result()
        .map_err(|partial| format!("Ritobin diagnostics: {:#?}", partial.diagnostics))?;

    let mut file = File::create(new_name)?;
    bin.to_writer(&mut file)?;
    log(sender, format!("Converted {}", old_name));
    Ok(())
}