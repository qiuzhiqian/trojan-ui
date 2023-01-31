use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub remarks: String,
    pub server: String,
    pub server_port: u16,
    pub client: String,
    pub client_port: u16,
    pub sni: String,
    pub password: String,
    verify: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigList {
    pub configs: Vec<Config>,
}

impl Config {
    pub fn default() -> Self {
        Self{
            remarks: "test".to_string(),
            server: "192.168.1.100".to_string(),
            server_port: 443u16,
            client: "127.0.0.1".to_string(),
            client_port: 1080u16,
            sni: "example.com".to_string(),
            password: "123456".to_string(),
            verify: true,
        }
    }
}

impl ConfigList{
    pub fn new_from_file(file: &str) -> std::io::Result<Self> {
        let f = std::fs::File::open(file).unwrap();
        let values:ConfigList = serde_json::from_reader(f)?;
        return Ok(values);
    }
}