use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use imghdr::Type;
use rustc_hash::FxHashMap;
use xxhash_rust::xxh3::xxh3_64;
use crate::cdtb::binary_parser::BinaryParser;
use zstd::stream::decode_all;
use crate::converter::main_gui::{log, WorkerMessage};

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

                for index in self.first_subchunk_index..self.first_subchunk_index + self.subchunk_count as u16 {

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
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData, "No Subchunk_toc"
                ).into())

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
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData, format!("Unsupported file type: {}", self.wad_type)
        ).into())
    }
    /// Read data, convert it if needed, and write it to a file
    /// On error, partially retrieved files are removed.
    /// File redirections are skipped.
    fn extract(&self, sender:&Sender<WorkerMessage>, parser: &mut BinaryParser, output_path: PathBuf, subchunk_toc: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.read_data(parser, subchunk_toc)?;

        if data.is_empty(){
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData, "No Data"
            ).into())
        }
        log(sender, format!("Writing to {}", &output_path.to_str().unwrap_or("Could not read file")));
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).inspect_err(|e|{ log(sender, format!("Could not create directory: {}", e))})?;
        }
        fs::write(&output_path, data).inspect_err(|e|{ log(sender, format!("Could not write into file: {}", e))})?;
        Ok(())
    }
    /// Guesses an extension
    /// static
    fn guess_extension(sender:&Sender<WorkerMessage>, data: Vec<u8>)-> String{
        // def guess_extension(data):
        // # image type
        let typ = imghdr::from_bytes(data);
        // typ = imghdr.what(None, h=data)
        if typ.unwrap() == Type::Jpeg{
            return "jpg".to_string()
        } else if typ.unwrap() == Type::Xbm { //todo
            // pass
        } else if typ.is_some() {
            let mut s = format!("{:?}", typ);
            s.make_ascii_lowercase();
            return s
        }
        // # json
        // try:
        //  json.loads(data)
        //  return 'json'
        // except (json.JSONDecodeError, UnicodeDecodeError):
        //  pass
        //
        // # others
        // for prefix, ext in WadFileHeader._magic_numbers_ext.items():
        //  if data.startswith(prefix):
        //      return ext
        "".to_string()

    }
    // def guess_extension(data):
    // # image type
    // typ = imghdr.what(None, h=data)
    // if typ == 'jpeg':
    // return 'jpg'
    // elif typ == 'xbm':
    // pass  # some HLSL files are recognized as xbm
    // elif typ is not None:
    // return typ
    //
    // # json
    // try:
    // json.loads(data)
    // return 'json'
    // except (json.JSONDecodeError, UnicodeDecodeError):
    // pass
    //
    // # others
    // for prefix, ext in WadFileHeader._magic_numbers_ext.items():
    // if data.startswith(prefix):
    // return ext
    // return None
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
    subchunk_toc: Vec<u8>,
    hash_to_guessed_extensions: HashMap<u64, String>
}
impl Wad{
    /// Constructor of the wad archive
    ///
    /// # Arguments
    /// * path: Path to the wad archive
    /// * hashes: hash file. generally hashes.game
    pub(crate) fn new(sender:&Sender<WorkerMessage>, path: String, hashes: &FxHashMap<u64, Box<str>>) -> Wad{
        let mut wad = Wad {
            path,
            version: Version::new(),
            files: vec![],
            subchunk_toc: vec![],
            hash_to_guessed_extensions: HashMap::new()
        };
        wad.parse_headers().expect("TODO: panic message"); //todo
        wad.resolve_paths(hashes);
        wad.load_subchunk_toc(sender).expect("TODO: panic message"); //todo
        wad
    }
    /// Parse version and file list
    fn parse_headers(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let data = fs::read(self.path.clone())?;

        let mut parser = BinaryParser::new(data);

        let magic         = parser.read_bytes::<2>(); // "2s"
        let version_major = parser.read_u8();            // "B"
        let version_minor = parser.read_u8();            // "B"


        if magic != *b"RW"{
            panic!("invalid magic code") //todo
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
            panic!("unsupported WAD version: {version_major}.{version_minor}"); //todo
        }

        let entry_count = parser.read_u32_le();
        if version_major == 1{ //fix, if this ever panics
            panic!("VERSION 1: NEEDS Implementation"); //todo
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
        Ok(())
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
                    .unwrap_or("")
                    .to_string();
            }
        }
    }
    /// finds and loads the subchunkchtoc
    fn load_subchunk_toc(&mut self, sender:&Sender<WorkerMessage>) -> Result<(), Box<dyn std::error::Error>>{
        for wad_file in &self.files{
            if wad_file.path == ""{
                continue
            }
            if !wad_file.path.ends_with(".subchunktoc"){
                continue
            }
            let data = fs::read(&self.path).inspect_err(|e| { log(sender, format!("Could not read subchunk_toc: {}", e))})?;
            let mut parser = BinaryParser::new(data);
            self.subchunk_toc = wad_file.read_data(&mut parser, vec![])?;
        }
        Ok(())
    }
    /// panics and stops the program if not all hashes are known
    pub fn guess_extensions(&mut self, sender:&Sender<WorkerMessage>){
        // avoid opening the file if not needed
        let mut unknown_ext = false;
        for wad_file in &mut self.files{
            if wad_file.ext == "" {
                wad_file.ext = self.hash_to_guessed_extensions.get(&wad_file.path_hash).unwrap().clone(); //todo
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
        let all_data = fs::read(self.path.clone()).unwrap(); //todo
        let parser = &mut BinaryParser::new(all_data);
        for wad_file in &mut self.files{
            if wad_file.ext == "" {
                let data = wad_file.read_data(parser, self.subchunk_toc.clone()).unwrap_or(vec![]);
                if data.is_empty(){
                    continue
                }
                wad_file.ext = WadFileHeader::guess_extension(sender, data); //probably todo
                self.hash_to_guessed_extensions.insert(wad_file.path_hash, wad_file.ext.clone());
            }
        }
    }
    /// Extract WAD archive
    ///
    /// # Arguments
    /// * output_path: the output path of the
    pub fn extract(&mut self, sender:&Sender<WorkerMessage>, output_path : String) -> Result<(), Box<dyn std::error::Error>>{
        self.sanitize_paths();
        self.set_unknown_paths("unknown");
        let data = fs::read(self.path.clone()).inspect_err(|e|{ log(sender, format!("Could not read file: {}", e))})?;
        let mut parser = BinaryParser::new(data);
        for wad_file in &self.files{
            let base = Path::new(&output_path);
            let relative = Path::new(&wad_file.path);

            let mut full_path = PathBuf::from(base);
            full_path.extend(relative.components());

            wad_file.extract(sender, &mut parser, full_path, self.subchunk_toc.clone())?;
        }
        Ok(())
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
                    panic!("CDTB PATH old name idk"); //todo
                    //some extensionless files conflict with folder names
                    //append a custom suffix to resolve this conflict
                    // ext = ".cdtb"; todo
                    // if wad_file.ext != ""{
                    //                 # extension was guessed, but the resolved path has no extension
                    //                 # in this case, append the guessed extension
                    //                 ext += f".{wadfile.ext}"
                    //             }
                    //             wadfile.path += ext
                }
                let path = PathBuf::from(&wad_file.path);

                let filename = path.file_name()
                    .unwrap_or_default() //todo?
                    .to_string_lossy();

                if filename.len() >= 250 {
                    let ext = path.extension()
                        .map(|e| format!(".{}", e.to_string_lossy()))
                        .unwrap_or_default(); //todo?

                    let parent = path.parent().unwrap_or(Path::new("")); //todo?

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
                    wad_file.path = format!("{}/{:016x}.{}", path, wad_file.path_hash, wad_file.ext); //removed the :016x from path hash
                }
                else {
                    wad_file.path = format!("{}/{:016x}", path, wad_file.path_hash)
                }
            }
        }
    }
}