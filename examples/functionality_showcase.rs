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
    let conn_pb = ProgressBar::new()
        .total(100)
        .theme(Theme::Bouncing(4, '█', '·'))
        .desc("Connecting to Host");

    // Simulate waiting for a handshake
    for _ in 0..40 {
        conn_pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    conn_pb.finish_with_message("Connected!");
    
    // Add a small gap after the connection is established
    println!();

    // --- Multi-Bar Operations ---
    let mut multi = MultiProgress::new();

    // 1. A sleek ModernSlim bar for a "System Scan"
    let pb1 = multi.add(ProgressBar::new().total(100).theme(Theme::ModernSlim("#00FF00".into(), "#222222".into())).desc("Core Scan"));

    // 2. A Classic ASCII bar for "Network Sync"
    let pb2 = multi.add(ProgressBar::new().total(100).theme(Theme::Classic('█', '░')).desc("Net Sync"));

    // 3. A Rocket bar just for the flair
    let pb3 = multi.add(ProgressBar::new().total(100).theme(Theme::Rocket).desc("Delivery") );

    // Run a loop for 100 iterations
    for i in 1..=100 {
        // Core Scan is fast and reliable
        if i <= 100 { pb1.inc(1); }
        
        // Net Sync is a bit slower (stops at 60 due to abandonment)
        if i % 2 == 0 && i <= 60 { pb2.inc(1); }
        
        // Rocket is constant 2x speed
        pb3.inc(2);

        std::thread::sleep(std::time::Duration::from_millis(40));

        // Mid-way failure simulation for the Network Sync
        if i == 60 {
            pb2.abandon("Connection Timed Out!");
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

    println!("\n\nThe progress bar below will VANISH after its finished");
    let pb_ghost = ProgressBar::new().desc("Ghost Bar").clear_on_finish(true);
    for _ in pb_ghost.wrap(0..20) {
        thread::sleep(Duration::from_millis(100));
    }

    println!("\n\nSpawn multiple bars at once!");
    multiple_bars();

    println!("\n\nAll showcases complete!");
}
