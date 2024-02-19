use config::{check_config_exists, get_config, get_download_path, set_folder_per_anime, Config};
use download::check_downloads_done;

#[cfg(windows)]
use winapi::um::wincon::GetConsoleWindow;

#[cfg(windows)]
use winapi::um::winuser::{ShowWindow, SW_HIDE};

use download::download;
use eframe::egui;
use std::thread;

mod config;
mod download;

#[derive(Default)]
struct MyApp {
    anime_link: String,
    anime_name: String,
    download_path: String,
}

fn do_config_stuff() -> Config {
    let mut config: Config = get_config();
    config.episode_count += 1;
    config.save();
    config
}

fn main() {
    if !check_config_exists() {
        get_config();
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 330.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Anime Downloader",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
    .unwrap();
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(windows)]
        {
            static mut STARTUP: bool = true;
            if unsafe { STARTUP } {
                unsafe {
                    STARTUP = false;
                }
                let mut style = egui::Style::default();
                style.spacing.item_spacing = egui::vec2(6.0, 10.0);

                ctx.set_style(style);
                thread::spawn(move || {
                    let window = unsafe { GetConsoleWindow() };
                    unsafe {
                        ShowWindow(window, SW_HIDE);
                    }
                });
            }
        }
        ctx.set_pixels_per_point(1.2);
        let mut config = get_config();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Anime Downloader");
            ui.add(
                egui::TextEdit::multiline(&mut self.anime_link)
                    .hint_text("Anime link")
                    .desired_width(f32::INFINITY),
            );
            ui.horizontal(|ui| {
                if ui.button("Download").clicked() {
                    let link = self.anime_link.clone();
                    thread::spawn(move || match download(link.trim(), do_config_stuff()) {
                        Ok(_) => println!("Download started successfully"),
                        Err(e) => println!("Error: {}", e),
                    });
                    self.anime_link = String::default();
                }
                if ui.button("Open config").clicked() {
                    config::open_config();
                }
                if ui
                    .checkbox(&mut config.folder_per_anime, "Folder per anime")
                    .changed()
                {
                    set_folder_per_anime();
                }
            });
            ui.horizontal(|ui| {
                ui.label(format!("Episode count: {}", config.episode_count + 1));
                if ui.button("➖").clicked() && config.episode_count != 0 {
                    config.episode_count -= 1;
                    config.save();
                }
                if ui.button("➕").clicked() {
                    config.episode_count += 1;
                    config.save();
                }
                if ui.button("Reset").clicked() {
                    config::set_episode_count(0);
                }
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.anime_name)
                        .hint_text(format!("Name: {}", config.name))
                        .desired_width(330.0),
                );
                if ui.button("Edit").clicked() {
                    config::set_anime_name(self.anime_name.clone());
                    self.anime_name = String::default();
                }
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.download_path)
                        .hint_text(format!("Path: {}", get_download_path()))
                        .desired_width(330.0),
                );
                if ui.button("Edit").clicked() {
                    config::set_download_path(self.download_path.clone());
                    self.download_path = String::default();
                }
            });
            // check if all downloads are done
            if check_downloads_done(config) {
                ui.label("All downloads are done");
            } else {
                ui.label("Downloads are still in progress");
            }
        });
    }
}
