use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::{fmt::Write, path::Path};
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
    let path = PathBuf::from("./");
    let ytd = YoutubeDL::new(&path, args, url).unwrap();
    thread::spawn(move || {
        let _ = ytd.download();
    });
    let finalfile = format!("{} E{}.mp4", config.name, config.episode_count);
    let filepath = finalfile.clone() + ".part";

    if !check_file_exists(&filepath) {
        return Err("Failed to start download");
    }

    println!("Starting download...");
    let pb = ProgressBar::new(450000000); // average file size for 1080p anime episode since we can't get the file size from yt-dl
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    while !pb.is_finished() {
        if Path::new(&filepath).exists() {
            pb.set_position(fs::metadata(&filepath).unwrap().len());
        } else {
            break;
        }
        thread::sleep(Duration::from_millis(20))
    }

    if fs::metadata(&finalfile).unwrap().len() < 1000000 {
        return Err("File size too small, download most likely failed");
    }

    pb.set_length(fs::metadata(&finalfile).unwrap().len());
    pb.finish_with_message("Done");
    Ok(config)
}
