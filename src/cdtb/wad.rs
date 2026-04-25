use std::fs;
use std::path::{Path, PathBuf};
use rustc_hash::FxHashMap;
use xxhash_rust::xxh3::xxh3_64;
use crate::cdtb::binary_parser::BinaryParser;
use zstd::stream::decode_all;

/// Single file entry in a WAD archive
pub struct WadFileHeader {
    path_hash: u64,
    offset: u32,
    compressed_size: u32,
    size: u32,
    subchunk_count: u8,
    wad_type: u8,
    duplicate: bool,
    first_subchunk_index: u16,
    sha256: u64,
    pub(crate) path: String,
    ext: String
}
impl WadFileHeader{
    // fn new_short(path_hash: u64, offset: u32, compressed_size: u32, size: u32, wad_type: u32) -> WadFileHeader{
    //     WadFileHeader{
    //         path_hash,
    //         offset,
    //         compressed_size,
    //         size,
    //         subchunk_count: (wad_type & 0xF0 >> 4) as u8, //this is buggy
    //         wad_type: (wad_type & 0xF) as u8, //see line above
    //         duplicate: false,
    //         first_subchunk_index: 0,
    //         sha256: 0,
    //         path: "".to_string(),
    //         ext: "".to_string(),
    //     }
    // }
    /// Creates a new WadFileHeader
    ///
    /// #Arguments
    /// * path_hash
    /// * offset
    /// * compressed_size
    /// * size
    /// * wad_type
    /// * duplicate
    /// * first_subchunk_index
    /// * sha256
    ///
    /// # Returns
    /// * A WadFileHeader
    fn new_long(path_hash: u64, offset: u32, compressed_size: u32, size: u32, wad_type: u8, duplicate: bool, first_subchunk_index: u16, sha256: u64) -> WadFileHeader {
        WadFileHeader {
            path_hash,
            offset,
            compressed_size,
            size,
            subchunk_count:( wad_type & 0xF0) >> 4,
            wad_type: wad_type & 0xF,
            duplicate,
            first_subchunk_index,
            sha256,
            path: "".to_string(),
            ext: "".to_string(),
        }
    }
    // _magic_numbers_ext = {
    //     b'OggS': 'ogg',
    //     bytes.fromhex('00010000'): 'ttf',
    //     bytes.fromhex('1a45dfa3'): 'webm',
    //     b'true': 'ttf',
    //     b'OTTO\0': 'otf',
    //     b'"use strict";': 'min.js',
    //     b'<template ': 'template.html',
    //     b'<!-- Elements -->': 'template.html',
    //     b'DDS ': 'dds',
    //     b'<svg': 'svg',
    //     b'PROP': 'bin',
    //     b'PTCH': 'bin',
    //     b'BKHD': 'bnk',
    //     b'r3d2Mesh': 'scb',
    //     b'r3d2anmd': 'anm',
    //     b'r3d2canm': 'anm',
    //     b'r3d2sklt': 'skl',
    //     b'r3d2': 'wpk',
    //     bytes.fromhex('33221100'): 'skn',
    //     b'PreLoadBuildingBlocks = {': 'preload',
    //     b'\x1bLuaQ\x00\x01\x04\x04': 'luabin',
    //     b'\x1bLuaQ\x00\x01\x04\x08': 'luabin64',
    //     bytes.fromhex('023d0028'): 'troybin',
    //     b'[ObjectBegin]': 'sco',
    //     b'OEGM': 'mapgeo',
    //     b'TEX\0': 'tex'
    // }

    /// Retrieve (uncompressed) data from WAD file object
    ///
    /// # Arguments
    /// * parser: The Binary Parser of the file
    /// * subchunk_toc
    ///
    /// # Returns
    /// * Success: Data as Vec<u8>
    /// * Fail: Error
    fn read_data(&self, parser: &mut BinaryParser, subchunk_toc: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        parser.seek(self.offset as u64);

        // println!("self.offset:          {}", self.offset);
        // println!("self.compressed_size: {}", self.compressed_size);
        // println!("self.size:            {}", self.size);

        //assume files are small enough to fit in memory
        let data = parser.read(self.compressed_size);
        if self.wad_type == 0 {
            return Ok(data)
        }
        else if self.wad_type == 1{
            // return gzip.decompress(data)
        }
        else if self.wad_type == 2{
            // n, = struct.unpack('<L', data[:4])
            // target = data[4:4+n].rstrip(b'\0').decode('utf-8')
            // logger.debug(f"file redirection: {target}")
            // return None
        }
        else if self.wad_type == 3{
            return Ok(decode_all(&*data)?)
        }
        else if self.wad_type == 4{
            // Data is split into individual subchunks that may be zstd compressed
            if !subchunk_toc.is_empty(){
                let mut chunks_data: Vec<u8> = vec![];
                let mut subchunk_parser = BinaryParser::new(subchunk_toc.clone());
                let mut offset = 0u32;

                // println!("subchunk_toc.len(): {}", &subchunk_toc.len());
                // println!("needed: {}", 16 * (self.first_subchunk_index as u64 + self.subchunk_count as u64));

                for index in self.first_subchunk_index..self.first_subchunk_index + self.subchunk_count as u16 {
                    // println!("seeking to: {}", 16 * index as u64);

                    subchunk_parser.seek(16 * index as u64); //this maybe works

                    let compressed_size = subchunk_parser.read_u32_le();   // I
                    let uncompressed_size = subchunk_parser.read_u32_le(); // I
                    let subchunk_hash = subchunk_parser.read_u64_le();     // Q
                    // compressed_size, uncompressed_size, subchunk_hash = struct.unpack('<IIQ', subchunk_toc[16*index:16*(index+1)])
                    // ensure wad data matches with the subchunktoc data

                    // println!("index:             {}", index);
                    // println!("data.len():        {}", data.len());
                    // println!("offset:            {}", offset);
                    // println!("compressed_size:   {}", compressed_size);
                    // println!("uncompressed_size: {}", uncompressed_size);

                    if data.len() < offset as usize + compressed_size as usize {
                        return Err("MALFORMED SUBCHUNK".into());
                    }
                    let mut subchunk_data: Vec<u8> = data[offset as usize..offset as usize + compressed_size as usize].to_vec();
                    if xxh3_64(&subchunk_data) != subchunk_hash {
                        return Err("Hashed Data is wrong".into());
                    }
                    if compressed_size == uncompressed_size{
                    // assume data is uncompressed
                     chunks_data.append(&mut subchunk_data)
                    }
                    else{
                     chunks_data.append(&mut decode_all(&*subchunk_data)?);
                    }
                    offset += compressed_size;
                }
                return Ok(chunks_data)
            }
            else{
                panic!("NO SUBCHUNK_TOC")
                // No subchunk TOC, try to decompress
            //     try:
            //         return zstd_decompress(data)
            //     except:
            //         raise MalformedSubchunkError(data)
            }
            // // No subchunk TOC, try to decompress
            //     try:
            //         return zstd_decompress(data)
            //     except:
            //         raise MalformedSubchunkError(data)
        }
        panic!("unsupported file type: {}", self.wad_type)
    }
    /// Read data, convert it if needed, and write it to a file
    /// On error, partially retrieved files are removed.
    /// File redirections are skipped.
    fn extract(&self, parser: &mut BinaryParser, output_path: PathBuf, subchunk_toc: Vec<u8>){
        let result = self.read_data(parser, subchunk_toc);
        if result.is_err(){
            return
        }
        let data = result.unwrap();
        if data.is_empty(){
            return
        }
        println!("Writing to {}", &output_path.to_str().unwrap());
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).expect("Could not create directory");
        }
        fs::write(&output_path, data).expect("Could not write into file");
    }
}
/// Version of the WAD archive
struct Version{
    major: u8,
    minor: u8
}
impl Version{
    /// Creates a new Version object
    ///
    /// # Returns
    /// * A Version object
    fn new() -> Version{
        Version{
            major: 0,
            minor: 0,
        }
    }
    /// sets the major and minor value of the version
    ///
    /// # Arguments
    /// * version_major: Major value of the version
    /// * version_minor: Minor value of the version
    fn add(&mut self, version_major: u8, version_minor: u8){
        self.major = version_major;
        self.minor = version_minor;
    }
}

/// A WAD archive is a file that contains other files.
/// It has a header that describes the format. There are multiple
/// formats that Riot uses depending on the version of the WAD file, which can be read from the header.
/// The files contained in a WAD file generally are all related to one "idea".
///
/// This class has one major purpose: to extract the individual files in a specific WAD archive for further analysis.
pub struct Wad{
    path: String,
    version: Version,
    pub(crate) files: Vec<WadFileHeader>,
    subchunk_toc: Vec<u8>
}
impl Wad{
    /// Constructor of the wad archive
    ///
    /// # Arguments
    /// * path: Path to the wad archive
    /// * hashes: hash file. generally hashes.game
    pub(crate) fn new(path: String, hashes: &FxHashMap<u64, Box<str>>) -> Wad{
        let mut wad = Wad {
            path,
            version: Version::new(),
            files: vec![],
            subchunk_toc: vec![]
        };
        wad.parse_headers();
        wad.resolve_paths(hashes);
        wad.load_subchunk_toc();
        wad
    }
    /// Parse version and file list
    fn parse_headers(&mut self){
        let data = fs::read(self.path.clone()).unwrap();

        let mut parser = BinaryParser::new(data);

        let magic         = parser.read_bytes::<2>(); // "2s"
        let version_major = parser.read_u8();            // "B"
        let version_minor = parser.read_u8();            // "B"


        if magic != *b"RW"{
            panic!("invalid magic code")
        }
        self.version.add(version_major, version_minor);

        if version_major == 1 {
            parser.seek(8);
        }
        else if version_major == 2 {
            parser.seek(100);
        }
        else if version_major == 3 {
            parser.seek(268);
        }
        else{
            panic!("unsupported WAD version: {version_major}.{version_minor}");
        }

        let entry_count = parser.read_u32_le();
        if version_major == 1{ //fix, if this ever panics
            panic!("VERSION 1: NEEDS Implementation");
            // self.files = (0..entry_count)
            //     .map(|_| WadFileHeader::new_short(
            //          parser.read_u64_le(), // Q
            //          parser.read_u32_le(), // I
            //          parser.read_u32_le(), // I
            //          parser.read_u32_le(), // I
            //          parser.read_u32_le(), // I
            //     ))
            //     .collect();
        } else{
            self.files = (0..entry_count)
                .map(|_| WadFileHeader::new_long(
                    parser.read_u64_le(), // Q
                    parser.read_u32_le(), // I
                    parser.read_u32_le(), // I
                    parser.read_u32_le(), // I
                    parser.read_u8_le(), // B
                    parser.read_bool(),   // ?
                    parser.read_u16_le(), // H
                    parser.read_u64_le(), // Q
                ))
                .collect();
        }
    }
    /// checks hashes against Hashfile
    ///
    /// # Arguments
    /// * hashes: Hashmap of the hashes
    fn resolve_paths(&mut self, hashes: &FxHashMap<u64, Box<str>>){
        for wad_file in &mut self.files{
            if let Some(path) = hashes.get(&wad_file.path_hash) {
                wad_file.path = path.to_string();
                wad_file.ext = Path::new(path.as_ref())
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("Could not resolve path")
                    .to_string();
            }
        }
    }
    ///finds and loads the subchunkchoc
    fn load_subchunk_toc(&mut self){
        for wad_file in &self.files{
            if wad_file.path == ""{
                continue
            }
            if !wad_file.path.ends_with(".subchunktoc"){
                continue
            }
            let data = fs::read(&self.path).unwrap();
            let mut parser = BinaryParser::new(data);
            self.subchunk_toc = wad_file.read_data(&mut parser, vec![]).unwrap();
        }
    }
    /// panics and stops the program if not all hashes are known
    pub fn guess_extensions(&mut self){
        // avoid opening the file if not needed
        let mut unknown_ext = false;
        for wad_file in &mut self.files{
            if wad_file.ext == "" {
                // wad_file.ext = _hash_to_guessed_extensions.get(wad_file.path_hash);
                //_hashe_to_guessed_extensions soll zwischen Dateien bestehen bleiben, solange das Porgramm nicht beendet wurde.
                if wad_file.ext == "" {
                    println!("The following extension is not known: {}", wad_file.path);
                    unknown_ext = true;
                }
            }
        }
        if !unknown_ext{
            println!("All extensions known");
            return
        }
        panic!("NOT ALL EXTENSIONS KNOWN")
        // with open(self.path, 'rb') as f{
        //     for wadfile in self.files{
        //         if not wadfile.ext{
        //             data = self.read_file_data(f, wadfile)
        //             if not data{
        //                 continue
        //             }
        //             wadfile.ext = WadFileHeader.guess_extension(data)
        //             _hash_to_guessed_extensions[wadfile.path_hash] = wadfile.ext
        //         }
        //     }
        // }
    }
    /// Extract WAD archive
    ///
    /// # Arguments
    /// * output_path: the output path of the
    pub fn extract(&mut self, output_path : String){
        self.sanitize_paths();
        self.set_unknown_paths("unknown");
        let data = fs::read(self.path.clone()).unwrap();
        let mut parser = BinaryParser::new(data);
        for wad_file in &self.files{
            let base = Path::new(&output_path);
            let relative = Path::new(&wad_file.path);

            let mut full_path = PathBuf::from(base);
            full_path.extend(relative.components());

            wad_file.extract(&mut parser, full_path, self.subchunk_toc.clone());
        }
    }

    /// Sanitize paths for extract purposes; for example truncating files whose basename has a length of at least 250
    fn sanitize_paths(&mut self){
        for wad_file in &mut self.files{
            if wad_file.path != ""{
                let ext = Path::new(&wad_file.path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| format!(".{}", e))
                    .unwrap_or(".cdtb".to_string());
                if ext == ".cdtb"{
                    println!("{}", ext);
                    panic!("CDTB PATH old name idk");
                    //some extensionless files conflict with folder names
                    //append a custom suffix to resolve this conflict
                    // ext = ".cdtb";
                    // if wad_file.ext != ""{
        //                 # extension was guessed, but the resolved path has no extension
        //                 # in this case, append the guessed extension
        //                 ext += f".{wadfile.ext}"
        //             }
        //             wadfile.path += ext
                }
                let path = PathBuf::from(&wad_file.path);

                let filename = path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy();

                if filename.len() >= 250 {
                    let ext = path.extension()
                        .map(|e| format!(".{}", e.to_string_lossy()))
                        .unwrap_or_default();

                    let parent = path.parent().unwrap_or(Path::new(""));

                    let truncated_stem = &filename[..250 - 17 - ext.len()];
                    let new_filename = format!("{}.{:016x}{}", truncated_stem, wad_file.path_hash, ext);

                    wad_file.path = parent.join(new_filename).to_string_lossy().into_owned();
                }
            }
        }

    }
    /// Set a path for files without one
    /// actually does nothing, but panics if not all paths are known, cuz I am lazy
    fn set_unknown_paths(&mut self, path: &str){
        for wad_file in &mut self.files{
            if wad_file.path == ""{
                if wad_file.ext == "" {
                    // wad_file.path = format!("{}/{}.{}", path, wad_file.path_hash:016x, wadfile.ext);
                }
                else {
                    // wad_file.path = format!("{}/{}", path, wadfile.path_hash:016x)
                }
                panic!("NOT ALL PATHS KNOWN")
            }
        }
    }
}