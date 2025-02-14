use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use colored::Colorize;

fn main() {
    let second: f64 = 10.0; //time
    let total_ticks: usize = (second * 10.0).round() as usize;

    for tick in 0..=total_ticks {
        let progress: f64 = (tick as f64 / total_ticks as f64) * 100.0;
        let bar_length: usize = 50;
        let filled_length: usize = (progress / 100.0 * bar_length as f64).round() as usize;
        let bar: String = "█".repeat(filled_length) + &" ".repeat(bar_length - filled_length);

        print!("\r[{}] {}% {:.1}/{}s", bar.blue(), progress.round(), (tick as f64) * 0.1, second);
        io::stdout().flush().expect("");

        thread::sleep(Duration::from_millis(100));
    }
}