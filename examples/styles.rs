use ezbars::{ProgressBar, Theme};
use std::thread;
use std::time::Duration;

fn main() {
    // Format: (Enum, Description, Width, Iterations)
    let showcase_items = vec![
        (Theme::Smooth, "Smooth", 30, 50),
        (Theme::FillUp, "FillUp", 30, 50),
        (Theme::bouncing(), "bouncing", 30, 50),
        (Theme::Spinner, "Spinner", 10, 30),
        (Theme::Claude, "Claude", 10, 30),
        (Theme::Pacman, "Pacman", 20, 40),
        (Theme::Heartbeat, "Heartbeat", 20, 40),
        (Theme::DVD, "DVD", 20, 40),
        (Theme::WaterLevel, "WaterLevel", 30, 50),
        (Theme::Fish, "Fish", 30, 60),
        (Theme::Snake, "Snake", 30, 60),
        (Theme::Waves, "Waves", 30, 60),
        (Theme::Arrows, "Arrows", 30, 50),
        (Theme::FishBounce, "FishBounce", 30, 50),
        (Theme::DotWaves, "DotWaves", 30, 50),
        (Theme::Banner(" Custom text here ".to_string()), "Banner", 30, 100),
    ];

    // 2. Loop through the list once
    for (_, (theme, desc, width, iters)) in showcase_items.into_iter().enumerate() {
        let pb = ProgressBar::new()
            .width(width)
            .theme(theme)
            .desc(desc);

        for _ in pb.wrap(0..iters) {
            thread::sleep(Duration::from_millis(100));
        }

        print!("\n")
    }

    println!("\nAll showcases complete!");
}