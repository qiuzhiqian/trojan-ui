use crate::config;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{channel, Sender};

pub fn start(config: &config::Config) -> Option<Sender<bool>> {
    let proxy = trojan_rust::Proxy::new(&config.client, 
        config.client_port, &config.server, config.server_port, 
        &config.password, &config.sni);

    let (send, mut recv) = channel::<bool>(1);
    std::thread::spawn(move ||{
        let runtime = Runtime::new().unwrap();
        if let Err(e) = runtime.block_on(proxy.start(&mut recv)) {
            println!("trojan runtime err:{}",e);
        }
    });
    
    return Some(send);
}

pub fn stop(send: &Sender<bool>) {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        send.send(true).await.expect("send failed");
    });
}