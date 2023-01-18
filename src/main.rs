#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use std::path::PathBuf;

use trojan_ui::config::ConfigList;

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
    proxy: Option<trojan_rust::Proxy>,
    send: Option<tokio::sync::mpsc::Sender<bool>>,
}

impl Default for MyApp {
    fn default() -> Self {
        let config_list = ConfigList::new_from_file();
        println!("{:#?}",config_list);

        //let proxy = trojan_rust::Proxy::new(client_addr, client_port, server_addr, server_port, passwd, sni)
        let app = Self {
            configs: config_list,
            has_selected: 0,
            started: false,
            proxy: None,
            send: None,
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
                        if !self.started {
                            let config = &self.configs.configs[self.has_selected as usize];
                            let proxy = trojan_rust::Proxy::new(&config.client, config.client_port, &config.server, config.server_port, &config.password, &config.sni);
                            
                            self.proxy = Some(proxy);
                            if let Some(proxy) = &self.proxy {
                                self.send = proxy.start();
                            }
                        } else {
                            if let Some(proxy) = &self.proxy {
                                if let Some(s) = &self.send {
                                    proxy.stop(s);
                                }
                                
                            }
                        }
                        
                        self.started = !self.started;
                    }
                });
            });
        });
    }
}
