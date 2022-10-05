use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::error::Error;
use std::fmt::Write;
use std::io::stdin;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
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
    let path = PathBuf::from("C:/Divers/Anime Downloader");
    let ytd = YoutubeDL::new(&path, args, &*url)?;
    let download = ytd.download()?;

    println!("Starting download... Approximating 5 minutes as the program is currently unable to get yt-dl's output");
    thread::spawn(|| {
        let pb = ProgressBar::new(300);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:1}s", state.eta().as_secs()).unwrap())
            .progress_chars("#>-"));
        for _ in 0..300 {
            pb.inc(1);
            thread::sleep(Duration::from_millis(1000));
        }
        pb.finish_with_message("Done")
    });

    println!("{} Done ! Press enter to close", download.output());
    stdin()
        .read_line(&mut "".to_string())
        .expect("TODO: panic message");
    Ok(())
}
