mod beat_controller;
mod modul;
mod modul_utils;
mod nannou_view;
mod tape;
mod view;
use std::cmp::Ordering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args);
    // nannou_view::start();
    view::start(config);
}

pub struct Config {
    pub BPM: f32,
    pub bar_count: usize,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        match args.len().cmp(&2) {
            Ordering::Less => {
                println!("Modul will start with default config");
            }
            _ => {
                let arg: &str = &args[1][..];
            }
        }

        Self {
            BPM: 120.0,
            bar_count: 4,
        }
    }
}
