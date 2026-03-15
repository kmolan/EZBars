use ezbars::{ProgressBar, Theme};
use std::thread;
use std::time::Duration;

#[rustfmt::skip]
fn main() {
    // Format: (Enum, Description, Width, Iterations)
    let showcase_items = vec![
        (Theme::Smooth, "Smooth", 30, 50),
        (Theme::FillUp, "FillUp", 30, 50),
        (Theme::Bouncing(4, '█', ' '), "Bouncing", 30, 50),
        (Theme::Spinner, "Spinner", 10, 30),
        (Theme::Claude, "Claude", 10, 30),
        (Theme::Pacman, "Pacman", 20, 40),
        (Theme::Heartbeat, "Heartbeat", 20, 40),
        (Theme::DVD, "DVD", 20, 40),
        (Theme::WaterLevel, "WaterLevel", 30, 50),
        (Theme::Fish, "Fish", 30, 60),
        (Theme::Waves, "Waves", 30, 60),
        (Theme::Arrows, "Arrows", 30, 50),
        (Theme::Rocket, "Rocket", 30, 100),
        (Theme::FishBounce, "FishBounce", 30, 50),
        (Theme::DotWaves, "DotWaves", 30, 50),
        (Theme::Banner("  Custom text here  ".to_string()), "Banner", 30, 100),
        (Theme::NyanCat, "NyanCat", 30, 60),
        (Theme::Gradient("#FF00FF".into(), "#00FFFF".into()), "Gradient", 30, 50),
        (Theme::DualColor("#FF0000".into(), "#444444".into()), "DualColor", 30, 50),
    ];

    let pb_persist = ProgressBar::new().desc("Permanent").clear_on_finish(false);
    for _ in pb_persist.wrap(0..20) {
        thread::sleep(Duration::from_millis(100));
    }

    print!("\n");

    // This bar will vanish completely after finishing
    let pb_ghost = ProgressBar::new().desc("Ghost Bar").clear_on_finish(true);
    for _ in pb_ghost.wrap(0..20) {
        thread::sleep(Duration::from_millis(100));
    }

    print!("\n");

    for (_, (theme, desc, width, iters)) in showcase_items.into_iter().enumerate() {
        let pb = ProgressBar::new().width(width).theme(theme).desc(desc);

        for _ in pb.wrap(0..iters) {
            thread::sleep(Duration::from_millis(100));
        }

        print!("\n")
    }

    println!("\nAll showcases complete!");
}
