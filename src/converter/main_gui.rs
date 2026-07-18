use crate::converter::control::control;
use std::fs::File;
use std::path::Path;
use eframe::egui::{Context, Ui};
use eframe::{egui, Frame};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
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
                // self.check_sets() todo
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

        egui::Panel::top("my_panel").show_inside(ui, |ui| {
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

                    if ui.button("Create new Set").clicked() {
                        self.show_create_set = true;
                    }

                    if ui.button("Download hashes").clicked() {
                        //***********************
                        let (sender, receiver) = mpsc::channel();
                        self.worker = Some(receiver);

                        let ctx = ui.ctx().clone();  // needed to trigger repaints from the thread
                        let options = self.options.clone();

                        thread::spawn(move || {
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
            //make some buttons or checkboxes or smth
            ui.add(egui::Label::new("Hello World!"));
            ui.label("A shorter and more convenient way to add a label.");
            if ui.button("Click me").clicked() {
                // take some action here
            }
            ui.hyperlink("https://github.com/emilk/egui");

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

                    thread::spawn(move || {

                        control(&sender, &options, download_files, export_cslol, export_ltk);

                        if true{
                            log(&sender, " and written successfully");
                        }

                        ctx.request_repaint();  // wake the UI when done

                    });

                }
            });

            ui.separator();

            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.label("Not much, as it turns out");
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
                ui.heading("No Options.txt in the current directory detected. Do you want to create a new one in the current directory?");
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Confirm").clicked() {
                        self.sets.push(self.new_set_input.clone());
                        File::create_new(options_file).inspect_err(|e|{log(&Self::get_sender(), "Could not create file")}); //todo
                        self.state = AppState::Running
                    }
                    if ui.button("Cancel (this closes the application)").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        } else{
            //todo read file, fill options
            self.options = Options::new();
            self.state = AppState::Running;
        }
    }
    // fn check_sets(&mut self){ //todo
    //     vec!("Default".to_string());
    //     //check if set folders exist, if yes, load them, if no create default one
    //     //when creating default one copy files into it
    // }
    fn get_sender() -> Sender<WorkerMessage>{
        let (sender, receiver) = mpsc::channel();
        return sender
    }
}
pub fn log(sender: &mpsc::Sender<WorkerMessage>, msg: impl Into<String>) {
    let _ = sender.send(WorkerMessage::Log(msg.into()));
}



