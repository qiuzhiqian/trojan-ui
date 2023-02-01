use serde::{Serialize, Deserialize};
use regex::Regex;

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

    // trojan://password@domain:port?security=tls&type=tcp&headerType=none#remark
    pub fn from_url(url:&str) -> Self {
        // ([-A-Za-z0-9+&@#/%?=~_|!:,.;]+[-A-Za-z0-9+&@#/%=~_|]):([0-9]*)/(.*)
        let re = Regex::new(r"^trojan://(?P<passwd>[^@]+)@(?P<domain>[-A-Za-z0-9+&@#/%?=~_|!:,.;]+[-A-Za-z0-9+&@#/%=~_|]*):(?P<port>[0-9]{1,5})[^#]+#(?P<remarks>[-A-Za-z0-9+&@#/%=~_|.]+)$").unwrap();


        let caps = re.captures(url).unwrap();

        let passwd = caps.name("passwd").unwrap().as_str().to_string();
        let domain = caps.name("domain").unwrap().as_str().to_string();
        let port_str = caps.name("port").unwrap().as_str().to_string();
        let remarks = caps.name("remarks").unwrap().as_str().to_string();

        let port = port_str.parse::<u16>().unwrap();//String to int

        println!("{} {} {} {}",passwd,domain,port,remarks);
        Self{
            remarks: remarks,
            server: domain,
            server_port: port,
            client: "127.0.0.1".to_string(),
            client_port: 1080u16,
            sni: "".to_string(),
            password: passwd,
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