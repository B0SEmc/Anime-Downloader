use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fs::File;
use std::io::stdin;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::{error::Error, fmt::Write};
use ytd_rs::{Arg, YoutubeDL};

// get the link
fn main() -> Result<(), Box<dyn Error>> {
    eprint!("Enter the m3u8 playlist link: ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read link");
    let link = input.trim();
    eprint!("Enter the name: ");
    let mut input2 = String::new();
    stdin().read_line(&mut input2).expect("Failed to read name");
    let name: &str = input2.trim();
    let name_cmd: String = name.to_string() + ".mp4";
    download(link, name_cmd)
}

// download the playlist to mp4
fn download(url: &str, name_cmd: String) -> Result<(), Box<dyn Error>> {
    let args = vec![
        Arg::new("--all-subs"),
        Arg::new_with_arg("-f", "mp4"),
        Arg::new_with_arg("--output", &name_cmd),
    ];
    // set the path to the current directory
    let path = PathBuf::from(".");
    let ytd = YoutubeDL::new(&path, args, &*url)?;
    thread::spawn(move || {
        let download = ytd.download();
        return download;
    });
    let filepath = "".to_string() + &name_cmd + ".part";
    thread::sleep(Duration::from_secs(6));

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
        .read_line(&mut String::new())
        .expect("TODO: panic message");
    Ok(())
}
