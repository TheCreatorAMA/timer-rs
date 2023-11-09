use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

/// A simple timer program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of time for the timer to run for
    #[arg(short, long)]
    time: u64,

    /// Unit of time (s: seconds, m: minutes, h: hours)
    #[arg(short, long, default_value_t = 's')]
    unit: char,

    /// Link to open when timer is up
    #[arg(short, long)]
    link_to_play: Option<String>,
}

fn main() {
    let args = Args::parse();

    run_progress_bar(args.time, args.unit);

    if args.link_to_play.is_some() {
        let _ = open::that(args.link_to_play.unwrap());
    }
}

fn run_progress_bar(length: u64, unit: char) {
    let conversion: u64 = match unit {
        's' => 1,
        'm' => 60,
        'h' => 60 * 60,
        _ => 1,
    };

    let total: u64 = conversion * length;

    let bar = ProgressBar::new(total);
    bar.set_style(
        ProgressStyle::with_template("⏱️  [{eta_precise}] ❰{bar:50.cyan/blue}❱ {msg}")
            .unwrap()
            .progress_chars("██▒"),
    );

    for _ in 0..total {
        thread::sleep(Duration::from_secs(1));
        bar.inc(1);
    }

    bar.finish_with_message("Done!✅");
}
