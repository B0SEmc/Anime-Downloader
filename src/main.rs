use config::{get_config, Config};
use download::download;
use std::io::stdin;

mod config;
mod download;

fn main() {
    let config: Config = get_config();

    eprint!("Enter the m3u8 playlist link: ");
    let mut input = String::default();
    stdin().read_line(&mut input).expect("Failed to read link");
    let link = input.trim();
    match download(link, config) {
        Ok(mut config) => {
            config.episode_count += 1;
            config.save();
            println!("Done! Press enter to exit");
            if config.auto_close {
                stdin().read_line(&mut String::default()).unwrap();
            }
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
