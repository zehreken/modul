use std::cmp::Ordering;
use std::env;
mod core;
mod features;
mod view;
use colored::Colorize;
mod winit_view;
use winit_view::app;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args);

    pollster::block_on(app::start(config));
}

pub struct Config {
    pub bpm: u16,
    pub bar_count: usize,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let default_bpm = 120;
        let default_bar_count = 4;
        let mut bpm: u16 = default_bpm;
        let mut bar_count: usize = default_bar_count;

        println!();
        println!("{}", "                 ".on_yellow());
        println!("{}", "      MODUL      ".bold().blue().on_yellow());
        println!("{}", "                 ".on_yellow());
        println!();

        match args.len().cmp(&3) {
            Ordering::Less => {
                let message = format!("Not enough arguments...\nModul will start with default config {} BPM and {} bars...\n", default_bpm, default_bar_count);
                println!("{}", message.yellow());
            }
            Ordering::Equal => {
                let arg_one: &str = &args[1][..];
                let arg_two: &str = &args[2][..];

                bpm = arg_one.parse().unwrap();
                bar_count = arg_two.parse().unwrap();

                println!(
                    "Modul will start with config {} BPM and {} bars...\n",
                    bpm, bar_count
                );
            }
            Ordering::Greater => {
                println!("Too many arguments...\nModul will start with default config {} BPM and {} bars...\n", default_bpm, default_bar_count)
            }
        }

        Self { bpm, bar_count }
    }
}
