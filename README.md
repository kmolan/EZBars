# EZBars
[![On crates.io](https://img.shields.io/crates/v/ezbars.svg)](https://crates.io/crates/ezbars)
![Downloads](https://img.shields.io/crates/d/ezbars?style=flat-square)
![github](https://github.com/kmolan/EZBars/actions/workflows/build-tests.yml/badge.svg)
![github](https://github.com/kmolan/EZBars/actions/workflows/code-coverage.yml/badge.svg)

A lightweight, highly customizable, and thread-safe-ready CLI progress bar for Rust.


![EZBars demo](./assets/functionality_demo.gif)

## Features

* **EZ Syntax:** Wrap any exact-size iterator and get a visual progress bar instantly. Alternatively, create and manually tick the progress bar as needed.
* **Unicode-Safe Rendering:** Full support for multi-byte characters, emojis and specialized symbols.
* **Dynamic Text Updates:** Change the bar metrics and text on the fly. Let the users know exactly what's going on at any time.
* **Lifecycle States:** Let users know the final outcome using a success (green) or Failure (red) state. Configure whether to clear the bar from the terminal or persist upon completion. Keep it clean!
* **Safe Logging:** Only activates if running in a terminal to prevent piped output or log file clutter, encouraging hassle-free usage.
* **Multi-Bar Orchestration:** Spawn multiple bars at once seamlessly! EZBars auomatically adjust themselves on your terminal when in a nested configuration.
* **Smart ETA:**  Intelligent _Exponential Smoothing Algorithm_ for highly accurate statistics. 
* **Extensive library of built-in styles:** Choose from over 20 customizable styles: pick deterministic styles for percentage-based tracking or indeterminate spinners for active loading. To view all possible styles run `cargo run --example style_showcase` :

---

## Quick Start

Making progress bars are ez-pz!

```rust
use ezbars::ProgressIterator;

let pb = ProgressBar::new();

for item in pb.wrap(0..100) {
    // Perform work...
    std::thread::sleep(std::time::Duration::from_millis(30));
}

//Alternatively, manually increment while doing work
let pb2 = ProgressBar::new();
for iter in 0..500 {
    // ... perform complex logic ...

    // Manually increment by 1 (or any amount)
    pb.manually_increment(1);
}
```

## Static Configuration

You can fully customize the look and feel of the progress bar before the loop starts by chaining the builder methods.

```rust
use ezbars::{MultiProgress, ProgressBar, Style};

// Fully configure using the Builder Pattern
let pb = ProgressBar::new()
    .total(1000)                   // Define the target completion value (optional)
    .theme(Style::Fractional)      // Choose a visual style (optional)
    .width(60)                     // Customize the width in terminal columns (optional)
    .desc("High-Precision Upload") // Set the leading description string (optional)
    .vanish_on_finish(true);       // Vanish the bar once finished (optional)
```

## Dynamic Updates
Update the bar on the fly!

```rust
use ezbars::{MultiProgress, ProgressBar, Style};

let pb = ProgressBar::new();
for i in 1..=100 {
    if i == 25 { // Change the prefix text
        pb.set_description("Loading Assets...");
    }

    // Change metadata (like filenames or status) after the statistics
    if i == 50 {
        pb.set_postfix("Working on: metadata.json");
    }

    // Instead of incrementing, you can snap the bar to a specific value
    if i == 75 {
        pb.set_position(90); // Jump directly to 90%
    }
    else{
        // You can also manually increment to whatever value you want
        pb.manually_increment(1);
        // pb.manually_increment(5);
        // pb.manually_increment(10);
    }

    if failure_condition()
    {
        // Failure Termination
        // Stops the bar, colors it RED, and displays a final message
        pb.finish_with_failure("Connection Lost!");
        return;
    }

    thread::sleep(Duration::from_millis(50));
}

// Success Termination
// Finishes the bar, colors it Green, and displays a final message
pb.finish_with_message("Deployment Successful!");
```

## List of styles
By default, EZBars uses the `Fractional` style for a smooth classic feel. To see all a showcase of all style options, run `cargo run --example style_showcase`. This list will keep growing based on user feedback, so check back often! Many of these styles are also user-configurable for a personal feel.

![style demo](./assets/style_demo.gif)

```rust
/// [████---] | Deterministic | Fixed-character block bar. Parameters: (filled_char, empty_char)
Classic(char, char),

/// [███▌   ] | Deterministic | Default | High-resolution progress using Unicode 1/8th blocks.
#[default]
Fractional,

/// [ █▂   ] | Deterministic | Sequential vertical fill of individual cells from left to right.
VerticalFill,

/// [  ███  ] | Indeterminate | Ping-pong animation of a sliding block. Parameters: (block_width, filled_char, empty_char)
Bouncing(usize, char, char),

/// [ ██████ ] | Deterministic | Linear RGB interpolation between two 24-bit hex colors.
/// Parameters: (start_hex: String, end_hex: String)
Gradient(String, String),

/// [ ━━━━━━ ] | Deterministic | TrueColor slim-line bar for modern terminal emulators.
/// Parameters: (fill_hex: String, background_hex: String)
ModernSlim(String, String),

/// [ ━━━    ] | Indeterminate | color-shifting pattern for indeterminate states.
/// Parameters: (primary_hex: String, secondary_hex: String)
Marquee(String, String),

/// [ | ] | Indeterminate | Sequential ASCII rotation: `|`, `/`, `-`, `\`.
AsciiSpinner,

/// [ ⠋ ] | Indeterminate | Sequential rotation using 8-dot Braille patterns.
BrailleSpinner,

/// [ Hello! ] | Indeterminate | Horizontal marquee for custom strings.
TextTicker(String),

/// [  DVD   ] | Indeterminate | Bouncing DVD logo logic.
DVD,

/// [ ᗧ • • ] | Deterministic | Progression-based animation using pacman.
Pacman,

/// [ -/\--• ] | Deterministic | Dynamic EKG line with a leading pulse blip.
EKG,

/// [ ▂▃▅▆▇ ] | Deterministic | Global vertical fill level across the entire bar width.
WaterLevel,

/// [ ><(((°> ] | Deterministic | Fish swimming from left to right.
Fish,

/// [ ▁▅▇██▇▅ ] | Indeterminate | sine-wave oscillations using block height variations.
Waves,

/// [ >>>>   ] | Deterministic | Directional arrows using ASCII chevrons.
Arrows,

/// [ 🚀~~~~ ] | Deterministic | Rocket flying with ANSI-colored exhaust and starfield.
Rocket,

/// [ <°)))>< ] | Indeterminate | Ping-pong anuimation of a fish.
FishBounce,

/// [ ⠁⠈⠐⠠ ] | Indeterminate | High-speed rippling effect using dots.
DotWaves,

/// [ ~~~🐱  ] | Deterministic | 256-color rainbow trail with a cat.
NyanCat
```

## Safe Logging
_EZBars_ uses `std::io:IsTerminal` to automatically detect if its being used inside a terminal application. This allows it to prevent piped output or log file clutter, encouraging hassle-free usage.

---

## Ackowledgement
- Thanks to https://github.com/rsalmei/alive-progress for inspiration .
- I used Gemini Pro to write most of this code.

## Contact
Email me at anmolkathail@gmail.com , or create an issue in the github repository.

## Contributions
See [CONTRIBUTIONS.md](./CONTRIBUTIONS.md)