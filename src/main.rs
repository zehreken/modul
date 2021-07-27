mod beat_controller;
mod modul;
mod modul_utils;
mod nannou_view;
mod tape;
mod view;
use std::cmp::Ordering;
use std::env;
mod audio_model;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args);
    // nannou_view::start();
    view::start(config);
}

pub struct Config {
    pub bpm: f32,
    pub bar_count: usize,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let mut bpm: f32 = 120.0;
        let mut bar_count: usize = 4;
        match args.len().cmp(&3) {
            Ordering::Less => {
                println!("Not enough arguments. Modul will start with default config.");
            }
            Ordering::Equal => {
                let arg_one: &str = &args[1][..];
                let arg_two: &str = &args[2][..];

                bpm = arg_one.parse().unwrap();
                bar_count = arg_two.parse().unwrap();

                println!("bpm: {}, bar count: {}", bpm, bar_count);
            }
            Ordering::Greater => {
                println!("Too many arguments. Modul will start with default config.")
            }
        }

        Self { bpm, bar_count }
    }
}
