use serde::{Deserialize, Serialize};
use std::{fs, io::stdin, path::PathBuf};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub episode_count: u32,
    pub download_path: PathBuf,
    pub folders_per_anime: bool,
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
            download_path: PathBuf::from("."),
            folders_per_anime: false,
        }
    }
}

pub fn set_anime_name(name: String) {
    let mut config = get_config();
    config.name = name;
    config.save();
}

pub fn set_download_path(new_path: String) {
    let mut config = get_config();
    let path = PathBuf::from(new_path);
    match path.try_exists() {
        Ok(true) => {
            config.download_path = path;
            config.save();
        }
        Ok(false) => {
            println!("This path doesn't exist")
        }
        Err(_) => {
            println!("A problem occured while verifying the path.")
        }
    }
}

pub fn get_download_path() -> String {
    let config = get_config();
    if config.download_path.to_str().unwrap() == "." {
        ". (current directory)".to_owned()
    } else {
        config.download_path.to_str().unwrap().to_string()
    }
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
