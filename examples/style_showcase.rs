use ezbars::{MultiProgress, ProgressBar, Style};
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
        (Style::Fractional, "Fractional", 30, total_iterations),
        (Style::VerticalFill, "VerticalFill", 30, total_iterations),
        (Style::Bouncing(4, '█', ' '), "Bouncing", 30, total_iterations),
        (Style::Gradient("#FF00FF".into(), "#00FFFF".into()), "Gradient", 30, total_iterations),
        (Style::ModernSlim("#FF0000".into(), "#444444".into()), "ModernSlim", 30, total_iterations),
        (Style::Marquee("#FF0000".into(), "#444444".into()), "Marquee", 30, total_iterations),
        (Style::AsciiSpinner, "AsciiSpinner", 10, total_iterations),
        (Style::BrailleSpinner, "BrailleSpinner", 10, total_iterations),
        (Style::TextTicker("  Custom text here  ".to_string()), "TextTicker", 30, total_iterations),
        (Style::DVD, "DVD", 20, total_iterations),
        (Style::Pacman, "Pacman", 20, total_iterations),
        (Style::EKG, "EKG", 20, total_iterations),
        (Style::WaterLevel, "WaterLevel", 30, total_iterations),
        (Style::Fish, "Fish", 30, total_iterations),
        (Style::Waves, "Waves", 30, total_iterations),
        (Style::Arrows, "Arrows", 30, total_iterations),
        (Style::Rocket, "Rocket", 30, total_iterations),
        (Style::FishBounce, "FishBounce", 30, total_iterations),
        (Style::DotWaves, "DotWaves", 30, total_iterations),
        (Style::NyanCat, "NyanCat", 30, total_iterations),
    ];

    // Initialize and stack all bars
    for (style, desc, width, iters) in showcase_items {
        let pb = ProgressBar::new()
            .width(width)
            .style(style)
            .desc(desc)
            .set_total_capacity(iters);
        
        active_bars.push(multi.add(pb));
    }

    for _ in 0..total_iterations {
        for pb in &active_bars {
            pb.manually_increment(1); 
        }

        thread::sleep(Duration::from_millis(100));
    }

    println!("\nAll showcases complete!");
}
