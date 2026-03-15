use ezbars::ProgressBar;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Customizing the progress bar:");

    let pb = ProgressBar::new()
        .width(60)
        .fill_char('#')
        .empty_char('.')
        .desc("Downloading");

    // The loop takes ownership of the wrapped iterator, not the handle!
    for _ in pb.wrap(0..100) {
        thread::sleep(Duration::from_millis(40));
    }

    println!("Download complete!");
}
