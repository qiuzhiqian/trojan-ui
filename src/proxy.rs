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
            let mut sta = thread_state.lock().unwrap();
            *sta = ThreadState::ABORT(e.to_string());
        } else {
            let mut sta = thread_state.lock().unwrap();
            *sta = ThreadState::EXIT;
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


pub enum TestState{
    WAITTING,
    RUNNING,
    SUCCESS(u32),
    FAILED(String),
}
pub fn test(server:&str) -> Option<Arc<Mutex<TestState>>> {
    let state:Arc<Mutex<TestState>> = std::sync::Arc::new(Mutex::new(TestState::WAITTING));
    let thread_state = state.clone();
    std::thread::spawn(move ||{
        {
            let mut sta = thread_state.lock().unwrap();
            *sta = TestState::RUNNING;
        }

        let runtime = Runtime::new().unwrap();
        if let Err(e) = runtime.block_on(run_proxy_test()) {
            let mut sta = thread_state.lock().unwrap();
            *sta = TestState::FAILED(e.to_string());
        } else {
            let mut sta = thread_state.lock().unwrap();
            *sta = TestState::SUCCESS(123);
        }
        
    });
    return Some(state);
}

async fn run_proxy_test() -> reqwest::Result<()> {
    // do socks5 client
    return Ok(());
}