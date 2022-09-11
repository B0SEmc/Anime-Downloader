use std::error::Error;
use std::io::stdin;
use std::path::PathBuf;
use ytd_rs::{YoutubeDL, Arg};

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
    let args = vec![Arg::new("--all-subs"), Arg::new_with_arg("-f","mp4"), Arg::new_with_arg("--output", &*name_cmd)];
    let path = PathBuf::from("C:/Divers/Anime Downloader");
    let ytd = YoutubeDL::new(&path, args, &*url)?;

    println!("Starting download...");
    let download = ytd.download()?;

    eprint!("Downloading at: {}", download.output());
    stdin().read_line(&mut "".to_string()).expect("TODO: panic message");
    Ok(())
}