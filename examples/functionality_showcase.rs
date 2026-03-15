use ezbars::{MultiProgress, ProgressBar, Theme};
use std::thread;
use std::time::Duration;

fn success_bar() {
    let pb = ProgressBar::new().desc("Database Migration");

    for _ in pb.wrap(0..100) {
        thread::sleep(Duration::from_millis(20));
    }
    pb.finish_with_message("Migration Successful!");
}

fn failed_bar() {
    let pb = ProgressBar::new().desc("Database Migration");

    for i in pb.wrap(0..100) {
        if i == 90 {
            pb.abandon("Connection Lost!");
            return;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

#[rustfmt::skip]
fn multiple_bars() {
    let mut multi = MultiProgress::new();

    // 1. A sleek DualColor bar for a "System Scan"
    let pb1 = multi.add(ProgressBar::new().total(100).theme(Theme::DualColor("#00FF00".into(), "#222222".into())).desc("Core Scan"));

    // 2. A Standard ASCII bar for "Network Sync"
    let pb2 = multi.add(ProgressBar::new().total(100).theme(Theme::Standard('█', '░')).desc("Net Sync"));

    // 3. A Rocket bar just for the flair
    let pb3 = multi.add(ProgressBar::new().total(100).theme(Theme::Rocket).desc("Delivery") );

    // Run a loop for 100 iterations
    for i in 1..=100 {
        // Core Scan is fast and reliable
        if i <= 100 { pb1.inc(1); }
        
        // Net Sync is a bit slower
        if i % 2 == 0 && i <= 60 { pb2.inc(1); }
        
        // Rocket is constant
        pb3.inc(2);

        std::thread::sleep(std::time::Duration::from_millis(40));

        // Mid-way failure simulation for the Network Sync
        if i == 60 {
            pb2.abandon("Connection Timed Out!");
            // We'll keep the others running
        }
    }

    // Finalize the healthy ones
    pb1.finish_with_message("All systems green!");
    pb3.finish_with_message("Package landed!");
}

fn main() {
    println!("\n\nThe progress bar below will \x1b[32mSUCCEED\x1b[0m");
    success_bar();

    println!("\n\nThe progress bar below will \x1b[31mFAIL\x1b[0m");
    failed_bar();

    println!("\n\nThe progress bar below will persist after its finished");
    let pb_persist = ProgressBar::new().desc("Permanent").clear_on_finish(false);
    for _ in pb_persist.wrap(0..20) {
        thread::sleep(Duration::from_millis(100));
    }

    println!("\n\nThe progress bar below will VANISH after its finished");
    let pb_ghost = ProgressBar::new().desc("Ghost Bar").clear_on_finish(true);
    for _ in pb_ghost.wrap(0..20) {
        thread::sleep(Duration::from_millis(100));
    }

    println!("\n\nSpawn multiple bars to track specific processes! Some will \x1b[31mFAIL\x1b[0m others might \x1b[32mSUCCEED\x1b[0m");
    multiple_bars();

    println!("\n\nAll showcases complete!");
}
