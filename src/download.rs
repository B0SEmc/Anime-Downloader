use notify_rust::Notification;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use ytd_rs::{Arg, YoutubeDL};

use crate::config::Config;

fn check_file_exists(path: &str) -> bool {
    for _ in 0..9 {
        if Path::new(path).exists() {
            return true;
        }
        thread::sleep(Duration::from_secs(1));
    }
    false
}

fn final_check_file_exists(path: &str) -> bool {
    if Path::new(path).exists() {
        return true;
    }
    false
}

pub fn download(url: &str, config: Config) -> Result<Config, &str> {
    let args = vec![
        Arg::new("--all-subs"),
        Arg::new_with_arg("-f", "best"),
        Arg::new_with_arg(
            "--output",
            &format!("{} E{}.mp4", config.name, config.episode_count),
        ),
    ];
    let path = &config.download_path;
    let path = if config.folder_per_anime {
        path.join(&config.name)
    } else {
        path.to_path_buf()
    };
    let pathstring = match path.to_str() {
        Some(".") => "./",
        Some(p) => p,
        None => "./",
    };
    let ytd = YoutubeDL::new(&path, args, url).unwrap();
    thread::spawn(move || {
        let _ = ytd.download();
    });
    let finalfile = format!(
        "{}/{} E{}.mp4",
        pathstring, config.name, config.episode_count
    );
    let filepath = format!("{}.part", finalfile);

    if !check_file_exists(&filepath) {
        if final_check_file_exists(&finalfile) {
            return Ok(config);
        }
        Notification::new()
            .summary("Download failed")
            .body("Failed to start download, make sure you have yt-dlp installed and that the URL is correct.")
            .appname("Anime Downloader")
            .timeout(0)
            .show()
            .unwrap();
        return Err("Failed to start download, make sure you have yt-dlp installed and that the URL is correct.");
    }
    Ok(config)
}

pub fn check_downloads_done(config: Config) -> bool {
    let path = if config.folder_per_anime {
        config.download_path.join(&config.name)
    } else {
        config.download_path
    };

    if !path.exists() {
        return true;
    }

    let entries = fs::read_dir(path).unwrap();
    !entries
        .filter_map(Result::ok)
        .any(|entry| entry.path().to_str().unwrap().contains(".part"))
}
