#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 280.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Trojan UI Tools",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    configs: Vec<String>,
    has_selected: usize,
    started: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut app = Self {
            configs: Vec::<String>::new(),
            has_selected: 0,
            started: false,
        };
        app.configs.push("config.json".to_string());
        app.configs.push("examples/client.json".to_string());
        app.configs.push("examples/server.json".to_string());

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
                        self.started = !self.started;
                    }
                });
            });
        });
    }
}