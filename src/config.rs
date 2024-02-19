use serde::{Deserialize, Serialize};
use std::{fs, io::stdin, path::PathBuf, thread, time::Duration};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub episode_count: u32,
    pub download_path: PathBuf,
    pub folder_per_anime: bool,
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
            folder_per_anime: false,
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

pub fn set_folder_per_anime() {
    let mut config = get_config();
    config.folder_per_anime = !config.folder_per_anime;
    config.save();
}

pub fn open_config() {
    let _ = open::that("animed.toml");
}

pub fn check_config_exists() -> bool {
    fs::read_to_string("animed.toml").is_ok()
}

pub fn get_config() -> Config {
    thread::sleep(Duration::from_millis(2));
    let configfile = match fs::read_to_string("animed.toml") {
        Ok(configfile) => configfile,
        Err(_) => {
            // Create the config file if it doesn't exist
            let config = Config::default();
            config.save();
            println!("Created config file");
            return config;
        }
    };
    let config: Config = match toml::from_str(&configfile) {
        Result::Ok(config) => config,
        Result::Err(_) => {
            // wait 10ms and try again, some weird bug happens sometimes
            thread::sleep(Duration::from_millis(10));
            get_config()
        }
    };
    config
}
