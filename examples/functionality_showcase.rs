use ezbars::{MultiProgress, ProgressBar, Style};
use std::thread;
use std::time::Duration;

fn success_bar() {
    let pb = ProgressBar::new().desc("Database Migration");

    for _ in pb.wrap(0..100) {
        thread::sleep(Duration::from_millis(20));
    }
    pb.finish_with_success("Migration Successful!");
}

fn failed_bar() {
    let pb = ProgressBar::new().desc("Database Migration");

    for i in pb.wrap(0..100) {
        if i == 90 {
            pb.finish_with_failure("Connection Lost!");
            return;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

#[rustfmt::skip]
fn multiple_bars() {    
    // --- Multi-Bar Operations ---
    let mut multi = MultiProgress::new();

    // 1. A sleek ModernSlim bar for a "System Scan"
    let pb1 = multi.add(ProgressBar::new().set_total_capacity(100).style(Style::ModernSlim("#00FF00".into(), "#222222".into())).desc("Core Scan"));

    // 2. A Classic ASCII bar for "Network Sync"
    let pb2 = multi.add(ProgressBar::new().set_total_capacity(100).style(Style::Classic('█', '░')).desc("Net Sync"));

    // 3. A Rocket bar just for the flair
    let pb3 = multi.add(ProgressBar::new().set_total_capacity(100).style(Style::VerticalFill).desc("Delivery") );

    // Run a loop for 100 iterations
    for i in 1..=100 {
        // Core Scan is fast and reliable
        if i <= 100 { pb1.manually_increment(1); }
        
        // Net Sync is a bit slower (stops at 60 due to finish_with_failurement)
        if i % 2 == 0 && i <= 60 { pb2.manually_increment(1); }
        
        // Rocket is constant 2x speed
        pb3.manually_increment(2);

        std::thread::sleep(std::time::Duration::from_millis(40));

        // Mid-way failure simulation for the Network Sync
        if i == 60 {
            pb2.finish_with_failure("Connection Timed Out!");
        }
    }

    // Finalize the healthy ones
    pb1.finish_with_success("All systems green!");
    pb3.finish_with_success("Package landed!");
}

fn otf_updates() {
    let pb = ProgressBar::new()
        .set_total_capacity(100)
        .style(Style::Gradient("#FF00FF".into(), "#00FFFF".into()))
        .desc("Initializing...");

    for i in 0..=100 {
        // 2. Update the text description on-the-fly based on progress
        if i == 20 {
            pb.set_description("Loading Assets");
        }
        if i == 50 {
            pb.set_description("Connecting to API");
        }
        if i == 80 {
            pb.set_description("Finalizing Data");
        }

        // 3. Increment the bar
        pb.manually_increment(1);

        thread::sleep(Duration::from_millis(50));
    }

    pb.finish_with_success("Process Complete!");
}

fn main() {
    println!("\n\nThe classic EZ way:");
    let pb = ProgressBar::new();
    for _ in pb.wrap(0..20) {
        thread::sleep(Duration::from_millis(50));
    }

    println!("\n\nProgress bars can \x1b[32mSUCCEED\x1b[0m");
    success_bar();

    println!("\n\nProgress bars can \x1b[31mFAIL\x1b[0m");
    failed_bar();

    println!("\n\nProgress bars can VANISH after they're done!");
    let pb_ghost = ProgressBar::new()
        .style(Style::Marquee("#FF0000".into(), "#444444".into()))
        .desc("Loading...")
        .vanish_on_finish(true);
    for _ in pb_ghost.wrap(0..20) {
        thread::sleep(Duration::from_millis(100));
    }

    println!("\n\nDynamic text and colors!");
    otf_updates();

    println!("\n\nNested bars!");
    multiple_bars();

    println!("\n");
}
