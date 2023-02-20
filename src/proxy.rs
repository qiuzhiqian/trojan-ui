use crate::config;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{channel, Sender};
use std::sync::{Arc,Mutex};

pub enum ThreadState{
    WAITTING,
    RUNNING,
    ABORT(String),
    EXIT,
}

pub fn start(config: &config::Config) -> (Option<Sender<bool>>,Option<Arc<Mutex<ThreadState>>>) {
    let proxy = trojan_rust::Proxy::new(&config.client, 
        config.client_port, &config.server, config.server_port, 
        &config.password, &config.sni);

    let (send, mut recv) = channel::<bool>(1);
    let state:Arc<Mutex<ThreadState>> = std::sync::Arc::new(Mutex::new(ThreadState::WAITTING));
    let thread_state = state.clone();
    std::thread::spawn(move ||{
        {
            let mut sta = thread_state.lock().unwrap();
            *sta = ThreadState::RUNNING;
        }
        let runtime = Runtime::new().unwrap();
        if let Err(e) = runtime.block_on(proxy.start(&mut recv)) {
            println!("trojan runtime err:{}",e);
            {
                let mut sta = thread_state.lock().unwrap();
                *sta = ThreadState::ABORT("proxy close".to_string());
            }
        }
    });
    
    return (Some(send),Some(state));
}

pub fn stop(send: &Sender<bool>) {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        send.send(true).await.expect("send failed");
    });
}

pub fn test(server:&str) {
    println!("TODO test by {}",server);
}