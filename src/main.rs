use config::{get_config, Config};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use std::fs::File;
use std::io::stdin;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
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
    download(link, config);
}

// download the playlist to mp4
fn download(url: &str, config: Config) {
    let args = vec![
        Arg::new("--all-subs"),
        Arg::new_with_arg("-f", "mp4"),
        Arg::new_with_arg(
            "--output",
            &format!("{} E{}.mp4", config.name, config.episode_count),
        ),
    ];
    let path = PathBuf::from(".");
    let ytd = YoutubeDL::new(&path, args, &*url).unwrap();
    thread::spawn(move || {
        let download = ytd.download();
        return download;
    });
    let filepath = String::from(format!("{} E{}.mp4", config.name, config.episode_count) + ".part");
    thread::sleep(Duration::from_secs(5));

    println!("Starting download...");
    let pb = ProgressBar::new(450000000);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    while !pb.is_finished() {
        let file = File::open(&filepath).unwrap();
        pb.set_position(file.metadata().unwrap().len());
        thread::sleep(Duration::from_millis(12))
    }
    pb.finish_with_message("Done");

    println!("Done ! Press enter to close");
    stdin()
        .read_line(&mut "".to_string())
        .expect("TODO: panic message");
}
