use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::Sender;
use rustc_hash::FxHashMap;
use crate::converter::main_gui::{log, WorkerMessage};

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
    pub(crate) fn load(&mut self, sender:&Sender<WorkerMessage>) -> Result<&FxHashMap<u64, Box<str>>, Box<dyn std::error::Error>> {
        if self.hashes.is_empty(){
            // open and read file line by line with a bufread
            // then put that into the hashmap
            let file = File::open(&self.filename).inspect_err(|e|{ log(sender, format!("Could not open file: {}", e))})?;
            let mut reader = BufReader::new(file);

            let mut line = String::with_capacity(256);
            while reader.read_line(&mut line).inspect_err(|e|{ log(sender, format!("Error while reading line: {}", e))})? > 0 {
                let trimmed = line.trim_end();
                if let Some((key_str, value)) = trimmed.split_once(' ') {
                    if let Ok(key) = u64::from_str_radix(key_str, 16) {
                        self.hashes.insert(key, value.into());
                    }
                }
                line.clear();
            }
        }
        Ok(&self.hashes)
    }
}