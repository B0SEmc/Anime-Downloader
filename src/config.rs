use serde::{Deserialize, Serialize};
use std::{fs, io::stdin};

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

impl Default for Config {
    fn default() -> Self {
        Config {
            name: String::from("Anime name"),
            episode_count: 0,
        }
    }
}

pub fn set_anime_name(name: String) {
    let mut config = get_config();
    config.name = name;
    config.save();
}

pub fn set_episode_count(count: u32) {
    let mut config = get_config();
    config.episode_count = count;
    config.save();
}

pub fn open_config() {
    let _ = open::that("animed.toml");
}

pub fn check_config_exists() -> bool {
    fs::read_to_string("animed.toml").is_ok()
}

pub fn no_config_found() {
    let config = Config::default();
    config.save();
    println!("Created config file, please edit it and restart the program");
    stdin().read_line(&mut String::default()).unwrap();
    std::process::exit(0);
}

pub fn get_config() -> Config {
    let configfile = match fs::read_to_string("animed.toml") {
        Ok(configfile) => configfile,
        Err(_) => {
            let config = Config::default();
            config.save();
            println!("Created config file, please edit it and restart the program");
            stdin().read_line(&mut String::default()).unwrap();
            std::process::exit(0);
        }
    };
    let config: Config = toml::from_str(&configfile).unwrap();
    config
}
