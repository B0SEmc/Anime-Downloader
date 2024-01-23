use config::{get_config, Config};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fs;
use std::io::stdin;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::{fmt::Write, path::Path};
use ytd_rs::{Arg, YoutubeDL};

mod config;

// get the link
fn main() {
    let mut config: Config = get_config();

    eprint!("Enter the m3u8 playlist link: ");
    let mut input = String::default();
    stdin().read_line(&mut input).expect("Failed to read link");
    let link = input.trim();
    // increase episode count
    config.episode_count += 1;
    config.save();
    match download(link, config) {
        Ok(_) => {
            println!("Done! Press enter to exit");
            stdin().read_line(&mut String::default()).unwrap();
            std::process::exit(0);
        }
        Err(e) => {
            println!("{}", e);
            println!("Press enter to exit");
            stdin().read_line(&mut String::default()).unwrap();
            std::process::exit(69);
        }
    }
}

fn download(url: &str, config: Config) -> Result<(), &str> {
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
        let download = ytd.download();
        return download;
    });
    let finalfile = String::from(format!("{} E{}.mp4", config.name, config.episode_count));
    let filepath = String::from(finalfile.clone() + ".part");
    thread::sleep(Duration::from_secs(5));

    println!("Starting download...");
    let pb = ProgressBar::new(450000000);
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
        thread::sleep(Duration::from_millis(30))
    }

    if fs::metadata(&finalfile).unwrap().len() < 1000000 {
        return Err("File size too small, download most likely failed");
    }

    pb.finish_with_message("Done");
    Ok(())
}
