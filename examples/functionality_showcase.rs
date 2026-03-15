use ezbars::ProgressBar;
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
fn main() {
    println!("The progress bar below will succeed");
    success_bar();
    print!("\n");

    println!("The progress bar below will FAIL");
    failed_bar();
    print!("\n\n");

    println!("The progress bar below will persist after its finished");
    let pb_persist = ProgressBar::new().desc("Permanent").clear_on_finish(false);
    for _ in pb_persist.wrap(0..20) {
        thread::sleep(Duration::from_millis(100));
    }
    print!("\n\n");

    println!("The progress bar below will VANISH after its finished");
    let pb_ghost = ProgressBar::new().desc("Ghost Bar").clear_on_finish(true);
    for _ in pb_ghost.wrap(0..20) {
        thread::sleep(Duration::from_millis(100));
    }

    println!("\n\nAll showcases complete!");
}
