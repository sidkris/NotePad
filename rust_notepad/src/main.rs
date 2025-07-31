use eframe::egui;
use rfd::FileDialog;
use std::fs;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("NotePad", options, Box::new(|_cc| Box::new(NotePad::default())))
}


struct NotePad{
    text : String,
    path : Option<String>,
}

impl Default for NotePad {
    fn default() -> Self {
        Self { 
             text: String::new(),
             path: None, 
            }
    }
}

impl eframe::App for NotePad {
    fn update(&mut self, ctx : &egui::Context, _ : &mut eframe::Frame) {

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(
                |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(file) = FileDialog::new().add_filter("Text", &["txt"]).pick_file() {
                            if let Ok(contents) = fs::read_to_string(&file) {
                                self.text = contents;
                                self.path = Some(file.display().to_string());
                            }
                        }
                    }

                    if ui.button("Save").clicked() {
                        if let Some(ref path) = self.path {
                            let _ = fs::write(path, &self.text);
                        }
                        else if let Some(file) = FileDialog::new().save_file(){
                            let _ = fs::write(&file, &self.text);
                            self.path = Some(file.display().to_string());

                        }
                    }

                    if ui.button("New").clicked() {
                        self.text.clear();
                        self.path = None;
                    }

                });
                
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.text).desired_rows(25));
            });
        }
    }

