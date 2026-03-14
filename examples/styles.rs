use ezbars::{ProgressBar, Theme};
use std::thread;
use std::time::Duration;

fn main() {
    println!("1. The High-Resolution Smooth Bar:");
    let pb_smooth = ProgressBar::new()
        .width(30)
        .theme(Theme::Smooth)
        .desc("Downloading");
    
    for _ in pb_smooth.wrap(0..100) {
        thread::sleep(Duration::from_millis(30));
    }

    println!("\n2. The Bouncing 'Cylon' Bar:");
    let pb_bounce = ProgressBar::new()
        .width(30)
        .theme(Theme::Bouncing { block_width: 5, fill: '█', empty: ' ' })
        .desc("Waiting for server");

    for _ in pb_bounce.wrap(0..100) {
        thread::sleep(Duration::from_millis(30));
    }

    println!("\n3. The Classic ASCII Spinner:");
    let pb_classic_spinner = ProgressBar::new()
        .width(10)
        .theme(Theme::Spinner)
        .desc("Connecting");

    for _ in pb_classic_spinner.wrap(0..50) {
        thread::sleep(Duration::from_millis(50));
    }

    println!("\n4. The 'Claude' Spinner:");
    let pb_claude = ProgressBar::new()
        .width(10)
        .theme(Theme::Claude)
        .desc("Thinking");

    for _ in pb_claude.wrap(0..50) {
        thread::sleep(Duration::from_millis(50));
    }

    println!("\nAll tasks complete!");
}