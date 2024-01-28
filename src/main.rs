use config::{check_config_exists, get_config, no_config_found, Config};

#[cfg(windows)]
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{ShowWindow, SW_HIDE};

use download::download;
use eframe::egui;
use std::{thread, time::Duration};

mod config;
mod download;

#[derive(Default)]
struct MyApp {
    anime_link: String,
    anime_name: String,
}

fn do_config_stuff() -> Config {
    let mut config: Config = get_config();
    config.episode_count += 1;
    config.save();
    config
}

fn main() {
    if !check_config_exists() {
        no_config_found()
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 300.0]),
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
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
        static mut STARTUP: bool = true;
        if unsafe { STARTUP } {
            unsafe {
                STARTUP = false;
            }
            ctx.set_pixels_per_point(1.2);
            let mut style = egui::Style::default();
            style.spacing.item_spacing = egui::vec2(6.0, 10.0);

            ctx.set_style(style);
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                let window = unsafe { GetConsoleWindow() };
                unsafe {
                    ShowWindow(window, SW_HIDE);
                }
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Anime Downloader");
            ui.add(
                egui::TextEdit::multiline(&mut self.anime_link)
                    .hint_text("Anime link")
                    .desired_width(400.0),
            );
            ui.horizontal(|ui| {
                if ui.button("Download").clicked() {
                    let link = self.anime_link.clone();
                    println!("Downloading {}", link);
                    thread::spawn(move || {
                        download(link.trim(), do_config_stuff()).unwrap();
                    });
                }
                if ui.button("Open config").clicked() {
                    config::open_config();
                }
            });
            ui.horizontal(|ui| {
                ui.label(format!("Episode count: {}", get_config().episode_count + 1));
                if ui.button("➖").clicked() {
                    let mut config = get_config();
                    config.episode_count -= 1;
                    config.save();
                }
                if ui.button("➕").clicked() {
                    let mut config = get_config();
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
                        .hint_text(format!("Name: {}", get_config().name)),
                );
                if ui.button("Edit").clicked() {
                    config::set_anime_name(self.anime_name.clone());
                    self.anime_name = String::default();
                }
            });
        });
    }
}
