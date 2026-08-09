use crate::converter::control::control;
use std::fs::File;
use std::path::Path;
use eframe::egui::{Context, Ui};
use eframe::{egui, Frame};
use std::sync::mpsc::{self, Receiver, Sender};
use std::{fs, thread};
use std::io::{BufRead, BufReader};
use std::process::Command;
use crate::cdtb;
use crate::data::options::Options;

#[derive(Default)]
enum AppState {

    VersionCheck,
    #[default]
    CheckFile,
    CheckSets,
    Running,
}

#[derive(Default)]
pub struct AutomatedSkinSizeConverter {
    state: AppState,
    options: Options,
    download_files: bool,
    export_ltk:bool,
    export_cslol: bool,

    sets: Vec<String>,
    selected1: String,
    show_create_set: bool,
    new_set_input: String,

    log: Vec<String>,
    worker: Option<Receiver<WorkerMessage>>,
}
pub enum WorkerMessage {
    Log(String),
    Done,
}
impl eframe::App for AutomatedSkinSizeConverter {
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        match self.state {
            AppState::VersionCheck => (),
            AppState::CheckFile => self.check_options(ui),
            AppState::Running => self.main_ui(ui),
            AppState::CheckSets => (
                self.check_sets()
            ),
        }
    }
}

impl AutomatedSkinSizeConverter{
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn main_ui(&mut self, ui: &mut Ui){
        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
        //***********************
        if let Some(rx) = &self.worker {
            let mut done = false;
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    WorkerMessage::Log(line) => self.log.push(line),
                    WorkerMessage::Done => done = true,
                }
            }
            ui.ctx().request_repaint();

            if done {
                self.worker = None;  // borrow of self.worker has ended, safe to assign
            }
        }

        let is_busy = self.worker.is_some();
        //***********************

        egui::Panel::top("my_panel").show_inside(ui, |ui| { //todo deprecated
            ui.add_enabled_ui(!is_busy, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Options").clicked() {
                        // todo open UI for options IG
                    }
                    ui.label("Set:");
                    egui::ComboBox::from_id_salt("SetOption")
                        .selected_text(format!("{:?}", self.selected1))
                        .show_ui(ui, |ui| {
                            for radio_item in self.sets.iter() {
                                ui.selectable_value(
                                    &mut self.selected1,
                                    radio_item.clone(),
                                    radio_item.to_string(),
                                );
                            }
                        });

                    // if ui.button("Create new Set").clicked() {
                    //     self.show_create_set = true;
                    // }
                    if ui.button("Open current set in explorer").clicked(){
                        println!("{}", format!(r"{}\0PutSizeOptionFilesHere\default", self.options.get_project_path()));
                        Command::new("explorer")
                            .arg(format!(r"{}\0PutSizeOptionFilesHere\default", self.options.get_project_path()))
                            .spawn()
                            .unwrap();
                    }

                    if ui.button("Download hashes").clicked() {
                        //***********************
                        let (sender, receiver) = mpsc::channel();
                        self.worker = Some(receiver);

                        let ctx = ui.ctx().clone();  // needed to trigger repaints from the thread
                        let options = self.options.clone();

                        thread::spawn(move || { //todo move this to its own fn
                            log(&sender, "Starting download...");


                            if cdtb::hashes::download_hashes(&options, &sender).is_ok(){
                                log(&sender, "Hashes downloaded and written successfully");
                            }

                            ctx.request_repaint();  // wake the UI when done

                        });
                        //***********************
                    }

                });
            });
        });
        egui::CentralPanel::default().show_inside(ui, |ui| { //todo fuck

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.download_files, "Download Files");
                ui.checkbox(&mut self.export_cslol, "Export to cslol");
                ui.checkbox(&mut self.export_ltk, "Export to ltk");

                // todo button clear log

                if ui.button("Start Conversion").clicked() {
                    let (sender, receiver) = mpsc::channel();
                    self.worker = Some(receiver);

                    let ctx = ui.ctx().clone();
                    let options = self.options.clone();
                    let download_files = self.download_files.clone();
                    let export_cslol = self.export_cslol.clone();
                    let export_ltk = self.export_ltk.clone();

                    thread::spawn(move || { //todo move this to its own fn

                        control(&sender, download_files, export_cslol, export_ltk, "default"); //todo change set

                        ctx.request_repaint();  // wake the UI when done

                    });

                }
            });

            ui.separator();
            //***********************
            let row_height = ui.text_style_height(&egui::TextStyle::Body);
            let num_rows = self.log.len();

            egui::ScrollArea::vertical()
                .stick_to_bottom(true).auto_shrink(false)
                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                    for line in &self.log[row_range] {
                        ui.label(line);
                    }
                });
            //***********************
        });

        if self.show_create_set {
            self.add_set(ui)
        }
    }
    fn add_set(&mut self, ui: &Ui){
        let modal = egui::Modal::new(egui::Id::new("new_set")).show(ui, |ui| {
            ui.set_min_width(250.0);

            ui.heading("Creating a new set");
            ui.separator();

            ui.label("Name of the new set:");
            ui.text_edit_singleline(&mut self.new_set_input);

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("Confirm").clicked() {
                    self.sets.push(self.new_set_input.clone());
                    //todo create folder
                    // println!("Input: {}", self.text_input);
                    self.new_set_input.clear();
                    self.show_create_set = false;
                }
                if ui.button("Cancel").clicked() {
                    self.new_set_input.clear();
                    self.show_create_set = false;
                }
            });
        });

        if modal.should_close() {
            self.show_create_set = false;
        }
    }
    fn options(){
        //todo modal for options
        //modify file
    }
    fn check_options(&mut self, ui: &Ui){
        println!("check options");
        let options_file = Path::new("Options.txt");
        if !options_file.exists(){
             egui::Modal::new(egui::Id::new("new_options")).show(ui, |ui| {
                ui.set_min_width(250.0);

                 // todo read own options first, if they exist dont change anything
                 // todo change this to reading options.txt, asking for rest
                ui.heading("No Options.txt in the current directory detected. Do you want to create a new one and also add the paths? (Not implemented yet, use your old options)");
                ui.separator();
                ui.add_space(8.0);
                //todo felder fuer options, // todo check if each needed value is set
                ui.horizontal(|ui| {
                    // if ui.button("Confirm").clicked() {
                    //     File::create_new(options_file).inspect_err(|e| { log(&Self::get_sender(), "Could not create file") }).expect("TODO: panic message"); //todo
                    //     //todo dann file befuellen
                    //     self.state = AppState::CheckSets
                    // }
                    if ui.button("Cancel (this closes the application)").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        } else{
            let f = File::open("Options.txt").unwrap();
            // todo If you only need to read the entire file contents, consider std::fs::read() or std::fs::read_to_string() instead. Uff
            let br = BufReader::new(&f);

            let mut project_path = String::new();
            let mut league_path = String::new();
            let mut cslol_path = String::new();
            let mut ltk_path = String::new();

            for line in br.lines() {
                let line = line.unwrap();
                if line.starts_with("Root Path:"){
                    project_path = line.split_once(":").unwrap().1.trim().parse().unwrap();
                } else if line.starts_with("League Path:") {
                    league_path = line.split_once(":").unwrap().1.trim().parse().unwrap();
                } else if line.starts_with("CsLol Path:") {
                    cslol_path = line.split_once(":").unwrap().1.trim().parse().unwrap();
                } else if line.starts_with("Ltk Path:") {
                    ltk_path = line.split_once(":").unwrap().1.trim().parse().unwrap();
                }
            }
            self.options = Options::new(&*project_path, &*league_path, &*cslol_path, &*ltk_path);
            self.state = AppState::CheckSets;
        }
    }
    fn check_sets(&mut self){
        self.sets.push("default".parse().unwrap());
        self.selected1 = "default".parse().unwrap();
        // in project path, does a folder in Put folder?
        // if yes, use any of them as default, ideally save it for later, so you can remember which set was opened (ideally put everything from options.txt into that file
        // if no, look if it contains folders, if yes translate them, if no only create folder + 0Options file

        let project_path = self.options.get_project_path();
        let dir_path = Path::new(project_path).join("0PutSizeOptionFilesHere/default");
        //************** // todo i srsly need to check it and fix it probs
        if !dir_path.exists() {
            let file_path = Path::new(project_path).join("0PutSizeOptionFilesHere");

            if file_path.exists() {
                println!("Exists");

                // create default dir
                fs::create_dir_all(&dir_path).unwrap();

                // copy files from "Size Options" dir to default dir
                for entry in fs::read_dir(&file_path).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    // only copy files, skip subdirectories (like "default" itself)
                    if path.is_file() {
                        if let Some(file_name) = path.file_name() {
                            let dest = dir_path.join(file_name);
                            fs::copy(&path, &dest).unwrap();
                        }
                    }
                }

                let options_txt_path = ("Options.txt");
                println!("{:?}", options_txt_path);


                let content = fs::read_to_string(&options_txt_path).unwrap();
                println!("{}", content);

                let filtered_lines: Vec<String> = content
                    .lines()
                    .filter(|line| !line.contains(':'))
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| {
                        if let Some((name, value)) = line.split_once(' ') {
                            let value = value.trim();
                            if !value.is_empty() && value.chars().all(|c| c.is_ascii_digit()) {
                                format!("{} 0-{}", name, value)
                            } else {
                                line.to_string()
                            }
                        } else {
                            line.to_string()
                        }
                    })
                    .collect();

                let output = filtered_lines.join("\n");

                let dest_options_path = dir_path.join("0Options.txt");
                fs::write(&dest_options_path, output).unwrap();


            } else {
                // create all needed dirs, + 0Options.txt in default dir
                fs::create_dir_all(&dir_path).unwrap();
                let options_file = dir_path.join("0Options.txt");
                fs::File::create(&options_file).unwrap();
            }
        } else{
            println!("Default exists")
        }
        //**************
        self.state = AppState::Running;
    //     vec!("Default".to_string());
    //     //check if set folders exist, if yes, load them, if no create default one
    //     //when creating default one copy files into it
    }
    fn get_sender() -> Sender<WorkerMessage>{
        let (sender, receiver) = mpsc::channel();
        sender
    }
}
pub fn log(sender: &Sender<WorkerMessage>, msg: impl Into<String>) {
    let _ = sender.send(WorkerMessage::Log(msg.into()));
}



