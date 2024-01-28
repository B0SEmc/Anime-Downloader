use std::path::Path;
use std::path::PathBuf;
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

pub fn download(url: &str, config: Config) -> Result<Config, &str> {
    let args = vec![
        Arg::new("--all-subs"),
        Arg::new_with_arg("-f", "mp4"),
        Arg::new_with_arg(
            "--output",
            &format!("{} E{}.mp4", config.name, config.episode_count),
        ),
    ];
    let path = PathBuf::from(".");
    let ytd = YoutubeDL::new(&path, args, url).unwrap();
    thread::spawn(move || {
        let _ = ytd.download();
    });
    let finalfile = format!("{} E{}.mp4", config.name, config.episode_count);
    let filepath = finalfile.clone() + ".part";

    if !check_file_exists(&filepath) {
        return Err("Failed to start download, make sure you have yt-dlp installed and that the URL is correct.");
    }
    Ok(config)
}
