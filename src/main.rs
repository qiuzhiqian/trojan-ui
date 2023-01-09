#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use std::process::Command;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 280.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Trojan Tools",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    configs: Vec<String>,
    has_selected: usize,
    started: bool,
    child_path: String,
    child_process: Option<std::process::Child>,
}

impl Default for MyApp {
    fn default() -> Self {
        let app = Self {
            configs: get_configs(get_current_dir().expect("this is normal path"),"json"),
            has_selected: 0,
            started: false,
            child_path: find_process("trojan"),
            child_process: None,
        };

        return app;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Trojan Tools");
                ui.separator();
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    //.max_width(150.0)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                   
                    let item_count = self.configs.len();
                    for item in 0..item_count {
                        let layout = egui::Layout::left_to_right(egui::Align::LEFT).with_main_justify(true);
                        ui.with_layout(layout,|ui|{
                            if self.started {
                                ui.set_enabled(false);
                            } else {
                                ui.set_enabled(true);
                            }
                            
                            ui.selectable_value(&mut self.has_selected, item, &self.configs[item]);
                        });
                    }
                }).inner;

                ui.separator();
                ui.horizontal(|ui|{
                    if self.configs.is_empty() {
                        ui.set_enabled(false);
                    }else {
                        ui.set_enabled(true);
                    }
                    if ui.button("Edit").clicked() {
                        println!("Editor item...");
                    }

                    let start_label=vec!["Start","Stop"];
                    let current_index = if !self.started {
                        0
                    } else {
                        1
                    };
                    if ui.button(start_label[current_index]).clicked() {
                        println!("Start to connect {}",self.configs[self.has_selected as usize]);
                        
                        if !self.started {

                            if let Ok(mut path) = get_current_dir() {
                                path.push(&self.configs[self.has_selected as usize]);

                                if path.is_file() {
                                    self.child_process = Some(Command::new(&self.child_path)
                                        .args(["-c",path.to_str().expect("this is normal path")])
                                        .spawn()
                                        .expect("Failed to start echo process"));
                                }
                            }
                        } else {
                            if let Some(child) = self.child_process.as_mut() {
                                child.kill().expect("trojan is not running...");
                            }
                        }
                        
                        self.started = !self.started;
                    }
                });
            });
        });
    }

    fn on_close_event(&mut self) -> bool {
        if let Some(x) = self.child_process.as_mut(){
            x.kill().expect("trojan is not running...");
        }
                            
        return true;
    }
}

fn get_configs(path: PathBuf,suffix: &str) -> Vec<String> {
    let mut result = Vec::new();
    for entry in path.read_dir().expect("this is not dir") {
        let entry = entry.expect("this is entry...");
        if let Some(x) = entry.path().extension() {
            if x == suffix {
                result.push(entry.file_name().to_str().expect("is normal string").to_string());
            }
        }
    }

    return result;
}

fn get_current_dir() -> Result<PathBuf,String> {
    if let Ok(mut path) = std::env::current_exe() {
        path.pop();
        
        if path.is_dir() {
            return Ok(path);
        }
    }
    return Err("is not directory path".to_string());
}

fn find_process(name: &str) -> String {
    if let Ok(mut path) = get_current_dir() {
        path.push(name);
        
        if path.is_file() {
            return path.to_str().expect("is not normal path").to_string();
        }
    }

    return find_process_from_path(name);
}

fn find_process_from_path(name: &str) -> String {
    let key = "PATH";
    if let Ok(val) = std::env::var(key) {
        let paths :Vec<&str>= val.split(':').collect();

        for path in paths {
            let mut filepath = Path::new(path).to_path_buf();
            filepath.push(name);

            if filepath.is_file() {
                return filepath.to_str().expect("is not normal path").to_string();
            }
        }
    }

    return "".to_string();
}