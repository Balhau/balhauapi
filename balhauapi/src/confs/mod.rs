extern crate serde_yaml;
extern crate dotenv;

use dotenv::dotenv;
use std::fs::File;
use std::io::prelude::*;

pub mod envs;
pub mod msgs;


const CONFIG_FILE_ENV: &str = "CONFIG_FILE_ENV";

//Ssh Context needed for ssh command execution
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SshContext {
    pub name: String,
    pub host: String,
    pub user: String,
    pub pass: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebServer {
    pub binding_host: String,
    pub port: u16,
    pub log: bool,
    pub env: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    pub hostname: String,
    pub schema: String,
    pub port: i32,
    pub user: String,
    pub pass: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Folders {
    pub mediafolder: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BalhauApiConf {
    pub remotes : Vec<SshContext>,
    pub webserver: WebServer,
    pub database: Database,
    pub folders: Folders
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub configs: BalhauApiConf,
}

/**
* Method responsible for the loading of configurations
*/
pub fn load_app_configuration() -> AppConfig {
    dotenv().ok();
    let config_path = dotenv::var(CONFIG_FILE_ENV)
        .expect("Configuration environment variable not found");

    let mut f = File::open(config_path).expect("file not found");

    let mut yaml_str = String::new();
    f.read_to_string(&mut yaml_str)
        .expect("something went wrong reading the file");

    serde_yaml::from_str(&yaml_str).unwrap()
}