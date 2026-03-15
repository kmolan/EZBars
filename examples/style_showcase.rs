use ezbars::{MultiProgress, ProgressBar, Theme};
use std::thread;
use std::time::Duration;

#[rustfmt::skip]
fn main() {
    let mut multi = MultiProgress::new();
    
    // We'll store the bars here after adding them to the manager
    let mut active_bars = Vec::new();

    let total_iterations = 200;

    // Format: (Enum, Description, Width, Iterations)
    let showcase_items = vec![
        (Theme::Fractional, "Fractional", 30, total_iterations),
        (Theme::VerticalFill, "VerticalFill", 30, total_iterations),
        (Theme::Bouncing(4, '█', ' '), "Bouncing", 30, total_iterations),
        (Theme::Gradient("#FF00FF".into(), "#00FFFF".into()), "Gradient", 30, total_iterations),
        (Theme::ModernSlim("#FF0000".into(), "#444444".into()), "ModernSlim", 30, total_iterations),
        (Theme::Marquee("#FF0000".into(), "#444444".into()), "Marquee", 30, total_iterations),
        (Theme::AsciiSpinner, "AsciiSpinner", 10, total_iterations),
        (Theme::BrailleSpinner, "BrailleSpinner", 10, total_iterations),
        (Theme::TextTicker("  Custom text here  ".to_string()), "TextTicker", 30, total_iterations),
        (Theme::DVD, "DVD", 20, total_iterations),
        (Theme::Pacman, "Pacman", 20, total_iterations),
        (Theme::EKG, "EKG", 20, total_iterations),
        (Theme::WaterLevel, "WaterLevel", 30, total_iterations),
        (Theme::Fish, "Fish", 30, total_iterations),
        (Theme::Waves, "Waves", 30, total_iterations),
        (Theme::Arrows, "Arrows", 30, total_iterations),
        (Theme::Rocket, "Rocket", 30, total_iterations),
        (Theme::FishBounce, "FishBounce", 30, total_iterations),
        (Theme::DotWaves, "DotWaves", 30, total_iterations),
        (Theme::NyanCat, "NyanCat", 30, total_iterations),
    ];

    // Initialize and stack all bars
    for (theme, desc, width, iters) in showcase_items {
        let pb = ProgressBar::new()
            .width(width)
            .theme(theme)
            .desc(desc)
            .total(iters);
        
        active_bars.push(multi.add(pb));
    }

    for _ in 0..total_iterations {
        for pb in &active_bars {
            pb.inc(1); 
        }

        thread::sleep(Duration::from_millis(100));
    }

    println!("\nAll showcases complete!");
}
