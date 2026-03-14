use ezbars::{tqdm, ProgressIterator};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Standard Python-style tqdm:");
    for _ in tqdm(0..50) {
        thread::sleep(Duration::from_millis(30));
    }

    println!("\nRusty extension trait style:");
    // Look how incredibly clean this is!
    for _ in (0..50).progress() {
        thread::sleep(Duration::from_millis(30));
    }

    println!("\nDone!");
}