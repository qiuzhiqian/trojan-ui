#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod utils;
pub mod config;
pub mod proxy;
pub mod components;

use eframe::egui;
use egui_extras::RetainedImage;
use std::path::PathBuf;
use std::vec;

use crate::config::{ConfigList,Config};
//use crate::proxy;
//use crate::utils;
//use crate::proxy::config;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(420.0, 480.0)),
        resizable: false,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "Trojan Tools",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ) {
        println!("run app failed. Err:{}",e);
    }
}

struct MyApp {
    configs: ConfigList,
    has_selected: usize,
    started: bool,
    send: Option<tokio::sync::mpsc::Sender<bool>>,
    proxy_state: Option<std::sync::Arc<std::sync::Mutex<proxy::ThreadState>>>,
    input_url: String,
    page_num: u8,
    config_path: std::path::PathBuf,
    //dark_mode: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let (config_list,path) = load_config("config.json").expect("create config path failed");

        let app = Self {
            configs: config_list,
            has_selected: 0,
            started: false,
            send: None,
            proxy_state: None,
            input_url: "".to_string(),
            page_num: 0,
            config_path: path,
            //dark_mode: false,
        };

        return app;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(100.0);
            ui.vertical(|ui| {
                let style: egui::Style = (*ui.ctx().style()).clone();
                if self.configs.dark_mode != style.visuals.dark_mode {
                    if self.configs.dark_mode {
                        ui.ctx().set_visuals(egui::style::Visuals::dark());
                    } else {
                        ui.ctx().set_visuals(egui::style::Visuals::light());
                    }
                }
                if  let Some(s) = self.proxy_state.clone() {
                    let state = s.lock().unwrap();
                    //let thread_state = *state;
                    match &*state {
                        proxy::ThreadState::EXIT => println!("exit normal"),
                        proxy::ThreadState::ABORT(s) => println!("exit err {}",s),
                        _ => println!("is normal"),
                    }
                }
                match self.page_num {
                    1 => self.import_config_page(ui),
                    3 => self.share_config_page(ui),
                    4 => self.about_page(ui),
                    5 => self.settings_page(ui),
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
        ui.end_row();
        egui::ScrollArea::vertical()
            .max_height(400.0)
            //.max_width(150.0)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
            let item_count = self.configs.configs.len();
            for item in 0..item_count {
                if self.config_item_show(ui,item) {
                    return;
                }
            }
        }).inner;

        ui.separator();
        ui.horizontal(|ui|{
            if ui.button("⚙").on_hover_text("Settings").clicked() {
                self.page_num = 5;
            }

            ui.add_enabled_ui(!self.started,|ui|{
                if ui.button("➕").on_hover_text("Add").clicked() {
                    self.page_num = 1;
                }
            });
            

            if ui.button("❗").on_hover_text("About").clicked() {
                self.page_num = 4;
            }
        });
    }

    fn import_config_page(&mut self,ui: &mut egui::Ui) {
        ui.heading("Add Config");
        ui.separator();
        ui.add(egui::TextEdit::singleline(&mut self.input_url).hint_text("trojan://password@domain:port#remarks"));
        if ui.button("⮪").on_hover_text("Back").clicked() {
            if let Ok(config) = Config::from_url(&self.input_url){
                self.configs.configs.push(config);
                self.configs.save_to_file(self.config_path.to_str().expect("file is invalid")).expect("save config failed");
            }
            self.page_num = 0;
            self.input_url = "".to_string();
        }
    }

    fn share_config_page(&mut self,ui: &mut egui::Ui) {
        ui.heading("Share Config");
        ui.separator();
        let url = self.configs.configs[self.has_selected as usize].to_url();
        self.show_qrcode(ui,&url,185);

        ui.label(&url);
        if ui.button("⮪").on_hover_text("Back").clicked() {
            self.page_num = 0;
        }
    }

    fn show_qrcode(&mut self,ui: &mut egui::Ui,url:&str,size: u32) {
        let qr = fast_qr::QRBuilder::new(url).version(fast_qr::Version::V10)
            .build().unwrap();
        let width = qr.size;
        let height = qr.size;
        let image = image::ImageBuffer::from_fn(width as u32, height as u32, |x,y|{
            let index = y * (width as u32) + x;
            if qr.data[index as usize].value() {
                image::Rgb([0xffff as u16, 0xffff as u16, 0xffff as u16])
            } else {
                image::Rgb([0 as u16, 0 as u16, 0 as u16])
            }
        });
        let dy_image = image::DynamicImage::from(image);
        let dst_image = dy_image.resize_exact(size,size,image::imageops::FilterType::Nearest);
        let w = dst_image.width();
        let h = dst_image.height();
        let image_buffer = dst_image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let rgb = egui::ColorImage::from_rgba_unmultiplied([w as usize,h as usize],pixels.as_slice());

        let image = RetainedImage::from_color_image("qrcode",rgb);
        image.show(ui);
    }

    fn about_page(&mut self,ui: &mut egui::Ui) {
        //let version = env!("CARGO_PKG_VERSION");
        ui.heading("About");
        ui.separator();
        ui.label(format!("Version: {}",env!("CARGO_PKG_VERSION")));
        ui.label(format!("Auth: {}",env!("CARGO_PKG_AUTHORS")));
        if ui.button("⮪").on_hover_text("Back").clicked() {
            self.page_num = 0;
        }
    }

    fn config_item_show(&mut self,ui: &mut egui::Ui,mut index: usize) -> bool {
        let mut need_break = false;
        let layout = egui::Layout::right_to_left(egui::Align::Max).with_cross_align(egui::Align::Min);
        ui.with_layout(layout, |ui|{
            
            if index == self.has_selected {
                ui.add_enabled_ui(self.started, |ui|{
                    if ui.button("⚖").on_hover_text("Test").clicked() {
                        proxy::test("www.google.com");
                    }
                });

                ui.add_enabled_ui(!self.started,|ui|{
                    ui.spacing_mut().item_spacing.x = 2.0;
                    // Share
                    if ui.button("⌘").on_hover_text("Share").clicked() {
                        self.page_num = 3;
                    }
                    
                    // Edit
                    if ui.button("🖊").on_hover_text("Edit").clicked(){
                        self.page_num = 2;
                    }

                    // Delete
                    if ui.button("🗑").on_hover_text("Delete").clicked(){
                        self.configs.configs.remove(index);
                        index = 0;
                        self.has_selected = 0;
                        //记得需要保存配置
                        self.configs.save_to_file(self.config_path.to_str().expect("file is invalid")).expect("save config failed");
                        need_break = true;
                        return;
                    }
                });

                if need_break {
                    return;
                }
                //ui.separator();
                // alway enable
                ui.add_enabled_ui(true, |ui| {
                    let start_label=vec!["▶","⏹"];
                    let current_index = if !self.started {
                        0
                    } else {
                        1
                    };
                    
                    if ui.button(start_label[current_index]).clicked() {
                        if !self.started {
                            let config = &self.configs.configs[self.has_selected as usize];
                            (self.send,self.proxy_state) = proxy::start(config);
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

            if need_break {
                return;
            }

            if self.started {
                ui.set_enabled(false);
            } else {
                ui.set_enabled(true);
            }
            //让里面的元素占满整个布局，该项必须是整个父布局的最后一下，如果不是的话，考虑左右或者上下翻转的方式。
            let layout = egui::Layout::from_main_dir_and_cross_align(egui::Direction::TopDown,egui::Align::Min)
                .with_main_wrap(false)
                .with_cross_justify(true);

            ui.with_layout(layout,|ui|{
                ui.selectable_value(&mut self.has_selected, index, &self.configs.configs[index].remarks);
            });

            ui.end_row();
        });

        need_break
    }

    fn settings_page(&mut self,ui: &mut egui::Ui) {
        ui.horizontal(|ui|{
            ui.label("Dark Mode: ");
            ui.add(components::toggle_switch::toggle(&mut self.configs.dark_mode)).on_hover_text(
                "dark mode?",
            );
        });
        if ui.button("⮪").on_hover_text("Back").clicked() {
            self.configs.save_to_file(self.config_path.to_str().expect("file is invalid")).expect("save config failed");
            self.page_num = 0;
        }
    }
}


// $XDG_CONFIG_HOME/trojan_ui/config.json
// $HOME/.config/trojan_ui/config.json
fn find_config_file(name: &str) -> std::io::Result<PathBuf>{
    let mut path = utils::user_config_dir()?;
    if !path.exists() {
        std::fs::create_dir_all(path.clone())?;
    } else if !path.is_dir() {
        return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "the same name file exist"));
    }

    path.push("trojan_ui");
    path.push(name);

    Ok(path)
}

fn load_config(name:&str) -> std::io::Result<(ConfigList,PathBuf)> {
    let path = find_config_file(name)?;
    if !path.exists() {
        let config_list = ConfigList::default();
        config_list.save_to_file(path.to_str().expect("file path is invalid"))?;
        return Ok((config_list,path));
    } 
    return Ok((ConfigList::new_from_file(path.to_str().expect("is not vaild path")).expect("config is invalid"),path));
}