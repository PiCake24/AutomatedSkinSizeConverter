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
    #[default]
    Setup,      // showing the initial setup questions
    Running,    // normal app
}

#[derive(Default)]
pub struct AutomatedSkinSizeConverter {
    state: AppState,
    options: Options,
    sets: Vec<String>,
    selected1: String,
    show_create_set: bool,
    text_input: String,
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
            AppState::Setup => self.setup(ui),
            AppState::Running => self.main_ui(ui),
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
                        let (tx, rx) = mpsc::channel();
                        self.worker = Some(rx);
                        self.log.clear();

                        let ctx = ui.ctx().clone();  // needed to trigger repaints from the thread

                        thread::spawn(move || {
                            tx.send(WorkerMessage::Log("Starting download...".into())).ok();

                            // your actual work here
                            // cdtb::hashes::download_hashes();

                            tx.send(WorkerMessage::Log("Hashes downloaded successfully".into())).ok();
                            tx.send(WorkerMessage::Done).ok();

                            let x = 0;
                            while x < 100{
                                tx.send(WorkerMessage::Log("A".into())).ok();
                            }

                            ctx.request_repaint();  // wake the UI when done

                        });
                        //***********************
                    }

                });
            });
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            //make some buttons or checkboxes or smth
            ui.add(egui::Label::new("Hello World!"));
            ui.label("A shorter and more convenient way to add a label.");
            if ui.button("Click me").clicked() {
                // take some action here
            }
            ui.hyperlink("https://github.com/emilk/egui");

            ui.horizontal(|ui| {
                // checkbox for downloading files (this will also redownload hashes
                // checkbox for import cslol
                // checkbox for import ltk

                // button clear log

                if ui.button("Start Conversion").clicked() {
                    // convert according to rules in files
                }

                ui.label("Hi");
                ui.label("No");
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
            ui.text_edit_singleline(&mut self.text_input);

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("Confirm").clicked() {
                    self.sets.push(self.text_input.clone());
                    //todo create folder
                    // println!("Input: {}", self.text_input);
                    self.text_input.clear();
                    self.show_create_set = false;
                }
                if ui.button("Cancel").clicked() {
                    self.text_input.clear();
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

    fn setup(&mut self, ui: &Ui){
        //todo
        Self::check_options(self, ui);
        //check_sets()
        //todo versioncheck
        self.state = AppState::Running;
    }
    fn check_options(&mut self, ui: &Ui){
        let options_file = Path::new("Options.txt");
        if !options_file.exists(){
             egui::Modal::new(egui::Id::new("new_options")).show(ui, |ui| {
                ui.set_min_width(250.0);

                ui.heading("No Options.txt detected. Do you want to create a new one?");
                ui.separator();

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Confirm").clicked() {
                        self.sets.push(self.text_input.clone());
                        //todo create file, ask for data basically
                        File::create_new(options_file);
                    }
                    if ui.button("Cancel").clicked() {
                        // todo close application
                    }
                });
            });
            File::create_new(options_file).expect("Could not Create Options File");
        } else{
            //todo read file, fill options
            Options::new();
            Self::check_sets();
        }
    }
    fn check_sets() -> Vec<String>{ //todo
        return vec!("Default".to_string())
        //check if set folders exist, if yes, load them, if no create default one
        //when creating default one copy files into it
    }
}



