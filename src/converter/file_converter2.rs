use ltk_meta::Bin;
use ltk_ritobin::{HashMapProvider, Print, print::PrintConfig};
use std::error::Error;
use std::fs;
use std::fs::{File, read_to_string};
use std::io::BufReader;

pub fn bin_to_py(bin_path: &str, filename: &str) -> Result<(), Box<dyn Error>> {

    let old_name = format!(r"{}{}", bin_path, filename);
    let new_name = format!(r"{}\{}{}", bin_path, &filename[..filename.len() - 3], "py");
    println!("{}", new_name);
    let mut reader = BufReader::new(File::open(old_name)?);
    let bin = Bin::from_reader(&mut reader)?;

    let mut hashes = HashMapProvider::new();
    hashes.load_from_directory("C:/Users/Yanni/RustroverProjects/Skinswapper/hashes/ritobin/"); // loads hashes.bin{entries,fields,hashes,types}.txt

    let text = bin.print_with_config(PrintConfig::default().with_hashes(hashes))?;
    fs::write(new_name, text)?;

    println!("{}", format!("Converted: {}", &filename));
    Ok(())
}
// Parse ritobin text
pub fn binback2(py_path: &str) -> Result<(), Box<dyn Error>>{

    let old_name = format!(r"{}", py_path);
    let new_name = format!(r"{}{}", &py_path[..py_path.len() - 2], "bin");

    println!("{}", old_name);
    println!("{}", new_name);
    // let mut reader = BufReader::new(File::open(old_name).unwrap());
    let text = read_to_string(old_name).unwrap();

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
    Ok(())
}