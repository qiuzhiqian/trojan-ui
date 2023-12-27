#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod config;

use std::collections::HashMap;
use std::fs;
use std::env;
use std::rc::Rc;
use slint::Model;
use slint::VecModel;

slint::include_modules!();

fn load_config(path: &str) -> HashMap<String,config::Client> {
    let mut clients = HashMap::new();
    if let Ok(entrys) = fs::read_dir(path) {
        for entry in entrys {
            if let Ok(rel_entry) = entry {
                let path = rel_entry.path();
                if path.is_file() {
                    if let Ok(client) =  config::Client::from_file(path.to_str().unwrap()) {
                        clients.insert(path.to_str().unwrap().to_string(), client);
                    }
                }
            }
        }
    }
    clients
}

fn main() -> Result<(), slint::PlatformError> {
    let mut config_path = match env::var("XDG_CONFIG_HOME") {
        Ok(config_home) => config_home,
        _ => match env::var("HOME") {
            Ok(mut home) => {
                home.push_str("/.config");
                home
            },
            _ => return Err(slint::PlatformError::Other("can't find config path.".to_string())),
        },
    };
    config_path.push_str("/trojan_ui");

    let config_clients = load_config(&config_path);

    let ui = AppWindow::new()?;

    let mut clients: Vec<ClientData> = ui.get_client_datas().iter().collect();
    clients.extend(clients.clone());
    let clients_model = Rc::new(VecModel::from(clients));
    ui.set_client_datas(clients_model.clone().into());

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    for (path, client) in config_clients {
        clients_model.push(ClientData{title: client.remarks.clone().into(), running: false});
    }

    ui.run()
}