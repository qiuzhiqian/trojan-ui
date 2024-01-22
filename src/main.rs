#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod config;

use std::collections::HashMap;
use std::fs;
use std::env;
use std::rc::Rc;
use std::sync::{Arc,Mutex};
use chrono::{DateTime, Utc};

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
    if let Err(_) = env_logger::builder().filter_level(log::LevelFilter::Info).try_init(){
        log::info!("log has init.");
    }
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
    config_path.push_str("/trui");

    let config_clients = Arc::new(Mutex::new(load_config(&config_path)));

    let ui = AppWindow::new()?;

    let mut clients: Vec<ClientData> = ui.get_client_datas().iter().collect();
    clients.extend(clients.clone());
    let clients_model = Rc::new(VecModel::from(clients));
    ui.set_client_datas(clients_model.clone().into());

    let clients_obj = config_clients.clone();
    let ui_handle = ui.as_weak();
    let mut tx_arc :Option<tokio::sync::mpsc::Sender<bool>> = None;

    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let rt_clone = rt.clone();
    ui.on_connect(move |_on: bool, path: slint::SharedString| {
        log::info!("connect {}", &path.to_string());
        let locked_client = clients_obj.lock().unwrap();
        if let Some(p) = locked_client.get(&path.to_string()) {
            
            let (tx, mut rx) = tokio::sync::mpsc::channel(32);
            tx_arc = Some(tx);
            
            let proxy = trojan_rust::Proxy::new(&p.client,
                    p.client_port,
                    &p.server,
                    p.server_port,
                    &p.password,
                    &p.sni);
            rt_clone.spawn(async move{
                proxy.start(&mut rx).await;
            });
        }
    });

    let clients_obj = config_clients.clone();
    let ui_handle_obj = ui_handle.clone();
    ui.on_config_edit(move |path: slint::SharedString| {
        let ui = ui_handle_obj.unwrap();
        //ui.set_counter(ui.get_counter() + 1);
        if path.to_string() == "" {
            log::info!("add new config");
        } else {
            log::info!("config edit {}", &path.to_string());
            let locked_client = clients_obj.lock().unwrap();
            if let Some(p) = locked_client.get(&path.to_string()) {
                log::info!("remarks {}", &p.remarks);
                ui.set_remarks(p.remarks.clone().into());
                ui.set_server(p.server.clone().into());
                ui.set_server_port(p.server_port.to_string().into());
                ui.set_client(p.client.clone().into());
                ui.set_client_port(p.client_port.to_string().into());
                ui.set_sni(p.sni.clone().into());
                ui.set_password(p.password.clone().into());
                ui.set_verify(p.verify);
            }
        }
        ui.set_page_index(1);
    });

    let clients_obj = config_clients.clone();
    let ui_handle_obj = ui_handle.clone();
    let clients_model_obj = clients_model.clone();
    let config_path_clone = config_path.clone();
    ui.on_config_save(move |path: slint::SharedString| {
        let ui = ui_handle_obj.unwrap();
        if path.to_string() == "" {
            log::info!("save new config");
            let now: DateTime<Utc> = Utc::now();
            let new_path = format!("{}/config-{}.json", config_path_clone, now.format("%Y%m%d%H%M%S-%f"));
            let c = config::Client{
                remarks: ui.get_remarks().to_string().clone(),
                server: ui.get_server().to_string(),
                server_port : ui.get_server_port().to_string().parse().unwrap(),
                client : ui.get_client().to_string(),
                client_port : ui.get_client_port().to_string().parse().unwrap(),
                sni : ui.get_sni().to_string(),
                password : ui.get_password().to_string(),
                verify : ui.get_verify(),
            };

            let mut locked_client = clients_obj.lock().unwrap();
            
            clients_model_obj.push(ClientData{title: c.remarks.clone().into(), running: false, path: new_path.clone().into()});
            locked_client.insert(new_path, c);
        } else {
            log::info!("config save {}", &path.to_string());
            let mut locked_client = clients_obj.lock().unwrap();
            if let Some(p) = locked_client.get_mut(&path.to_string()) {
                log::info!("remarks {}", &p.remarks);
                p.remarks = ui.get_remarks().to_string().clone();
                p.server = ui.get_server().to_string();
                p.server_port = ui.get_server_port().to_string().parse().unwrap();
                p.client = ui.get_client().to_string();
                p.client_port = ui.get_client_port().to_string().parse().unwrap();
                p.sni = ui.get_sni().to_string();
                p.password = ui.get_password().to_string();
                p.verify = ui.get_verify();
                p.to_file(&path.to_string()).unwrap();

                let mut index = 0;
                for i in clients_model_obj.iter() {
                    if i.path.to_string() == path.to_string() {
                        log::info!("find path {}", &i.path.to_string());
                        break;
                    }
                    index = index + 1;
                }

                clients_model_obj.remove(index);
                clients_model_obj.insert(index, ClientData{title: p.remarks.clone().into(), running: false, path: path.into()})
            }
        }
        ui.set_page_index(0);
    });

    // for init config list
    {
        let clients_obj = config_clients.clone();
        let locked_client = clients_obj.lock().unwrap();
        for (path, client) in locked_client.iter() {
            clients_model.push(ClientData{title: client.remarks.clone().into(), running: false, path: path.into()});
        }
    }

    ui.run()
}