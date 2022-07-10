pub mod libs;

use std::collections::HashMap;

use home;
use clap::App;
use config_loader::config_reader;

fn main() {
    let config_path = "bins/api-wrapper/config";
    let token_path = token_path();
    
    // let yaml = load_yaml!("../cli.yaml");
    // let matches = App::from_yaml(yaml).get_matches();
    let config = config_reader::get_config(config_path);
    let acct = get_endpoints(config, "AccountPositions");
    println!("{:?}", acct);
}

fn get_endpoints(config: HashMap<String,String>, endpoint: &str) -> String {
    match config.get(endpoint) {
        Some(value) => value.to_string(),
        None => panic!("key doesn't exist")
    }
}

fn token_path() -> String {
    match home::home_dir() {
        Some(path) => format!("{}{}", path.display().to_string(), "/.questrade.json"),
        None => panic!("Issue find the home directory for user"),
    }
}