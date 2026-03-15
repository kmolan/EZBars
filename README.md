# Rust TQDM-Style Progress Bar

A lightweight, highly customizable, and thread-safe-ready CLI progress bar for Rust. Inspired by Python's beloved `tqdm` library, this tool completely separates your iteration logic from your terminal display logic.

## Features

* **Clean Syntax:** Wrap any exact-size iterator and get a visual progress bar instantly.
* **Builder Pattern Configuration:** Easily customize the width, characters, and initial text.
* **Dynamic Text Updates:** Change the prefix (description) and suffix (postfix metrics) on the fly from *inside* your `for` loop without fighting the borrow checker.
* **Safe Logging:** Print standard log messages (`println!` style) without breaking or overwriting the visual progress bar.

---

## Quick Start

The absolute simplest way to use the progress bar is to create a handle, configure it, and wrap your iterator.

```rust
use my_crate::ProgressBar;
use std::thread;
use std::time::Duration;

fn main() {
    let pb = ProgressBar::new();

    // Wrap your iterator (0..100) and loop as normal!
    for _ in pb.wrap(0..100) {
        thread::sleep(Duration::from_millis(50));
    }
}
```

## Static Configuration

You can fully customize the look and feel of the progress bar before the loop starts by chaining builder methods.

```rust
let pb = ProgressBar::new()
    .width(50)             // Set the physical width of the bar
    .fill_char('#')        // Character for completed progress
    .empty_char('.')       // Character for remaining progress
    .desc("Downloading");  // Initial prefix text

for _ in pb.wrap(0..100) {
    // ...
}
```

## Dynamic Text Updates
Because the ProgressBar uses a shared-state architecture under the hood, you can safely update the text of the bar from inside the loop body!

- set_description: Updates the text before the progress bar. Great for indicating current phases or steps.

- set_postfix: Updates the text after the progress bar. Perfect for live metrics (loss, accuracy, speed, file names).

```rust
let pb = ProgressBar::new().desc("Initializing");

for i in pb.wrap(0..100) {
    // 1. Update the description when reaching specific milestones
    if i == 20 {
        pb.set_description("Extracting Files");
    } else if i == 80 {
        pb.set_description("Cleaning Up");
    }

    // 2. Update the postfix on every single iteration with live data
    let current_memory = calculate_memory(i);
    pb.set_postfix(format!("Mem: {} MB", current_memory));
}
```

## Safe Logging (write)

If you use a standard println!() inside your loop, it will print right over the top of the progress bar, leaving a messy trail in your terminal.

Instead, use pb.write(). This method temporarily clears the bar, prints your log message, and seamlessly redraws the bar underneath it.

```rust
let pb = ProgressBar::new().desc("Processing");

for i in pb.wrap(0..100) {
    // Safely print warnings or logs mid-loop without breaking the UI
    if i == 50 {
        pb.write("=> [WARNING] Halfway point reached, network latency spiking.");
    }
}
```

## Putting It All Together

```rust
fn main() {
    let pb = ProgressBar::new()
        .width(40)
        .fill_char('█')
        .empty_char('-')
        .desc("Warming up");

    for i in pb.wrap(0..100) {
        // Dynamic Prefix
        if i == 10 {
            pb.set_description("Training Model");
        }

        // Dynamic Postfix
        let fake_loss = 100.0 / (i as f64 + 1.0);
        pb.set_postfix(format!("loss: {:.4}", fake_loss));

        // Safe Logging
        if i == 75 {
            pb.write("=> Epoch 75 reached. Saving checkpoint...");
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    
    println!("Task Complete!");
}
```

## Checklist
- Iterator Wrapper for tracking progress of any iterable
- Manual control for manual updates and incrementing tasks
- Multi-line support with an offset system for simultaneous updates
- Dynamic descriptions for custom prefix text
- Custom postfix capability for appending extra data
- Auto-timing that starts when the first iteration occurs
- Speed tracking for real-time iterations per second 
- Elapsed time tracking in minutes and seconds
- Dynamic ETA based on current performance
- Auto-scaling statistics for percentage and item counts
- Smooth and SmoothFill themes using Unicode fractional blocks
- Standard, Arrows, Spinner, and Claude ASCII themes
- Animated Pacman, Snake, and Rocket themes with propulsion trails
- Progressive Fish, FishBounce, and Water rising level themes
- DVD bouncing logo and Heartbeat pulse themes
- Waves and DotWaves right-to-left flowing ripple themes
- Banner theme for customizable scrolling tickers 
- 24-bit TrueColor gradients with customizable Hex codes
- DualColor theme for high-contrast filled and empty sections
- Slim profile floating bars using mid-height characters
- ANSI Nyan Cat with animated 256-color rainbow trail
- Intelligent bracket logic that adapts to the chosen theme
- Thread-safe shared state management
- Unicode-safe character handling for multi-byte emojis and symbols
- Hidden State: Allow the bar to "finish and disappear" or "finish and persist" based on a configuration flag. Users should also be able to call this function manually.
- Success/Failure States: Add a .finish_with_message("Done!") or .abandon() method that changes the bar color to green (success) or red (error) once the loop ends.
- Nested Bars: Provide a MultiProgress manager that handles the drawing offsets automatically, so users can just call multi.add(pb) without manually calculating line offsets.
- No-terminal Mode: If the program is piped into a file (e.g., myapp > log.txt), the library should detect this and stop printing ANSI escape codes/animations to avoid cluttering the log file with "garbage" characters.

## TODO

- Template Strings: Instead of hardcoding where the description, bar, and stats go, allow users to define a template like:
  "{desc} {bar} {percentage} | {eta}".

- Smoothing (EMA): Currently, if one loop iteration takes 5 seconds and the next takes 0.1 seconds, the ETA will jump wildly. Implement an Exponential Moving Average for the speed calculation so the ETA remains steady.

- Pause/Resume: Methods to temporarily stop the timer and stop the animation if the program needs to wait for user input or an external event.