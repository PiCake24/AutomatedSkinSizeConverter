use std::fs;
use std::io::Read;

/// Downloads hashes from Communitydragon
pub fn download_hashes(){
    let target_dir = "hashes";

    fs::create_dir_all(&target_dir).expect("Could not create target directory");

    let hash_files = [
        "hashes.binentries.txt",
        "hashes.binfields.txt",
        // "hashes.binhashes.txt",
        "hashes.bintypes.txt",
        "hashes.game.txt",
        // "hashes.lcu.txt",
        // "hashes.rst.txt",
    ];
    for basename in hash_files{
        println!("Downloading {}", basename);
        let url = format!("https://raw.communitydragon.org/data/hashes/lol/{}", basename);
        let mut res = reqwest::blocking::get(url).expect("");
        if res.status().is_success() {
            let mut body = String::new();
            res.read_to_string(&mut body).expect("");

            // println!("Status: {}", res.status());
            // println!("Headers:\n{:#?}", res.headers());
            // println!("Body:\n{}", body);
            fs::write(target_dir.to_owned() + "//" + basename, body).expect("Could not write to file");
        }
    }
}