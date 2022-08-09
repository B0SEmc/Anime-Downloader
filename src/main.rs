use std::io;
use std::process::Command;

// get the link
fn main() {
    eprint!("Enter the m3u8 playlist link: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read link");
    let link = input.trim();
    downloader(link);
}

// download the playlist to mp4
fn downloader(link: &str) {
    eprint!("Please enter the name of the anime: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read name");
    let name: &str = input2.trim();
    let name_cmd: String = "-o ".to_owned() + name + ".mp4";
    let mut cmd = Command::new("yt-dlp");
    cmd.arg("--all-subs");
    cmd.arg("-f mp4");
    cmd.arg(name_cmd);
    cmd.arg(link);
    println!("Downloading... This should take 5-10 minutes");
    cmd.output().expect("Failed to download");
    println!("Downloaded!");
}