use eframe::egui::{Context, Ui};
use eframe::{egui, Frame};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use crate::cdtb;

#[derive(Default)]
pub struct AutomatedSkinSizeConverter {
    sets: Vec<String>,
    selected1: String,
    show_popup: bool,
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
                        self.show_popup = true;
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

        if self.show_popup {
            self.add_set(ui)
        }
    }
}

impl AutomatedSkinSizeConverter{
    pub fn new(existing_sets: Vec<String>) -> Self {
        Self {
            sets: existing_sets,
            ..Default::default()
        }
    }
    fn add_set(&mut self, ui: &Ui,){
        let modal = egui::Modal::new(egui::Id::new("my_modal")).show(ui, |ui| {
            ui.set_min_width(250.0);

            ui.heading("My Popup");
            ui.separator();

            ui.label("Enter some text:");
            ui.text_edit_singleline(&mut self.text_input);

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("Confirm").clicked() {
                    self.sets.push(self.text_input.clone());
                    // println!("Input: {}", self.text_input);
                    self.text_input.clear();
                    self.show_popup = false;
                }
                if ui.button("Cancel").clicked() {
                    self.text_input.clear();
                    self.show_popup = false;
                }
            });
        });

        // Close if user clicks the dark backdrop
        if modal.should_close() {
            self.show_popup = false;
        }
    }
    fn options(){
    //todo modal for options
    }
}
