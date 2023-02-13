#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use std::path::PathBuf;

use trojan_ui::config::ConfigList;
use trojan_ui::proxy;
use trojan_ui::utils;

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
    configs: ConfigList,
    has_selected: usize,
    started: bool,
    send: Option<tokio::sync::mpsc::Sender<bool>>,
    input_url: String,
    page_num: u8,
    config_path: std::path::PathBuf,
}

impl Default for MyApp {
    fn default() -> Self {
        let path = find_config_file("config.json").expect("No configuration files could be found");
        let config_list = ConfigList::new_from_file(path.to_str().expect("is not vaild path")).expect("config is invalid");

        let app = Self {
            configs: config_list,
            has_selected: 0,
            started: false,
            send: None,
            input_url: "".to_string(),
            page_num: 0,
            config_path: path,
        };

        return app;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                match self.page_num {
                    1 => self.setting_page(ui),
                    _ => self.main_page(ui),
                };
                
            });
        });
    }
}

impl MyApp {
    fn main_page(&mut self,ui: &mut egui::Ui) {
        ui.heading("Trojan Tools");
        ui.separator();
        egui::ScrollArea::vertical()
            .max_height(200.0)
            //.max_width(150.0)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
            
            let item_count = self.configs.configs.len();
            for item in 0..item_count {
                let layout = egui::Layout::left_to_right(egui::Align::LEFT).with_main_justify(true);
                ui.with_layout(layout,|ui|{
                    if self.started {
                        ui.set_enabled(false);
                    } else {
                        ui.set_enabled(true);
                    }
                    
                    ui.selectable_value(&mut self.has_selected, item, &self.configs.configs[item].remarks);
                });
            }
        }).inner;

        ui.separator();
        ui.horizontal(|ui|{
            if self.configs.configs.is_empty() {
                ui.set_enabled(false);
            }else {
                ui.set_enabled(true);
            }
            if ui.button("Add").clicked() {
                println!("Add item...");
                self.page_num = 1;
            }

            if ui.button("Edit").clicked() {
                println!("TODO Edit item...");
            }

            if ui.button("Share").clicked() {
                println!("TODO Share item...");
            }

            let start_label=vec!["Start","Stop"];
            let current_index = if !self.started {
                0
            } else {
                1
            };
            if ui.button(start_label[current_index]).clicked() {
                if !self.started {
                    let config = &self.configs.configs[self.has_selected as usize];
                    self.send = proxy::start(config);
                } else {
                    if let Some(s) = &self.send {
                        proxy::stop(s);
                        self.send = None;
                    }
                }
                
                self.started = !self.started;
            }
        });
    }

    fn setting_page(&mut self,ui: &mut egui::Ui) {
        ui.heading("Add Config");
        ui.separator();
        ui.add(egui::TextEdit::singleline(&mut self.input_url).hint_text("trojan://password@domain:port#remarks"));
        if ui.button("Back").clicked() {
            if let Ok(config) = trojan_ui::config::Config::from_url(&self.input_url){
                self.configs.configs.push(config);
                self.configs.save_to_file(self.config_path.to_str().expect("file is invalid")).expect("save config failed");
            }
            self.page_num = 0;
        }
    }
}


// /current_dir/config.json
// $XDG_CONFIG_HOME/trojan_ui/config.json
// $HOME/.config/trojan_ui/config.json
// /etc/trojan_ui/config.json
fn find_config_file(name: &str) -> std::io::Result<PathBuf>{
    {
        let mut path = utils::get_current_dir()?;
        //path.push("trojan_ui");
        path.push(name);
        if path.is_file() {
            return Ok(path);
        }
    }
    

    if let Ok(val) = std::env::var("XDG_CONFIG_HOME") {
        let mut path = PathBuf::from(val);
        path.push("trojan_ui");
        path.push(name);
        if path.is_file() {
            return Ok(path);
        }
    }

    if let Ok(val) = std::env::var("HOME") {
        let mut path = PathBuf::from(val);
        path.push(".config");
        path.push("trojan_ui");
        path.push(name);
        if path.is_file() {
            return Ok(path);
        }
    }

    let mut path = PathBuf::from("/etc/trojan_ui");
    path.push(name);
    if path.is_file() {
        return Ok(path);
    }

    return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "file find failed"));
}