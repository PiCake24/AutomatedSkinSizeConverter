use std::fs::File;
use std::io::{BufRead, BufReader};
use rustc_hash::FxHashMap;


/// Store hashes, support load and caching
pub struct HashFile {
    filename: String,
    hashes: FxHashMap<u64, Box<str>>
}

impl HashFile{
    ///Constructor for a Hash file
    ///
    /// # Arguments
    /// * filename: the Name of the Hash file
    ///
    /// # Returns
    /// A HashFile object
    pub(crate) fn new(filename: String) -> HashFile{
        HashFile{
            filename,
            hashes: FxHashMap::with_capacity_and_hasher(2_300_000, Default::default())
        }
    }
    ///Reads and Loads a Hash file
    ///
    /// # Returns
    /// The HashMap of the file
    pub(crate) fn load(&mut self) -> &FxHashMap<u64, Box<str>> {
        if self.hashes.is_empty(){
            // open and read file line by line with a bufread
            // then put that into the hashmap
            let file = File::open(&self.filename).expect("Could not open file");
            let mut reader = BufReader::new(file);

            let mut line = String::with_capacity(256);
            while reader.read_line(&mut line).unwrap() > 0 {
                let trimmed = line.trim_end();
                if let Some((key_str, value)) = trimmed.split_once(' ') {
                    if let Ok(key) = u64::from_str_radix(key_str, 16) {
                        self.hashes.insert(key, value.into()); // Box<str>
                    }
                }
                line.clear();
            }

            // for line in reader.lines() {
            //     let line = line.unwrap();
            //     if let Some((key, value)) = line.split_once(' ') {
            //         let key = u64::from_str_radix(key, 16).unwrap();
            //         self.hashes.insert(key, value.into());
            //     }
            // }
        }
        &self.hashes
    }
}