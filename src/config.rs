use serde::{Deserialize, Serialize};
use std::{fs, io::stdin};
use toml;

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub episode_count: u32,
}

impl Config {
    pub fn save(&self) {
        let configfile = toml::to_string(&self).unwrap();
        fs::write("animed.toml", configfile).unwrap();
    }
}

pub fn get_config() -> Config {
    let configfile = match fs::read_to_string("animed.toml") {
        Ok(configfile) => configfile,
        Err(_) => {
            let config = Config {
                name: String::from("Anime name"),
                episode_count: 0,
            };
            config.save();
            println!("Created config file, please edit it and restart the program");
            stdin().read_line(&mut String::default()).unwrap();
            std::process::exit(0);
        }
    };
    let config: Config = toml::from_str(&configfile).unwrap();
    config
}
