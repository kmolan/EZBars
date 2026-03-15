# EZBars
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
* **Extensive library of built-in styles:** Choose from over 20 customizable styles: pick deterministic styles for percentage-based tracking or indeterminate spinners for active loading. To view all possible styles run `cargo run --example style_showcase` :

---

![style demo](./assets/style_demo.gif)

## Quick Start

Making progress bars are ez-pz!

```rust
let pb = ProgressBar::new();

for item in pb.wrap(0..100) {
    // Perform work...
    std::thread::sleep(std::time::Duration::from_millis(30));
}

//Alternatiely, manually increment while doing work

let pb2 = ProgressBar::new();
for iter in 0..500 {
    // ... perform complex logic ...

    // Manually increment by 1 (or any amount)
    pb.inc(1);
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
- Smooth and SmoothFill styles using Unicode fractional blocks
- Standard, Arrows, Spinner, and BrailleSpinner ASCII styles
- Animated Pacman, Snake, and Rocket styles with propulsion trails
- Progressive Fish, FishBounce, and Water rising level styles
- DVD bouncing logo and EKG pulse styles
- Waves and DotWaves right-to-left flowing ripple styles
- TextTicker style for customizable scrolling tickers 
- 24-bit TrueColor gradients with customizable Hex codes
- ModernSlim style for high-contrast filled and empty sections
- Slim profile floating bars using mid-height characters
- ANSI Nyan Cat with animated 256-color rainbow trail
- Intelligent bracket logic that adapts to the chosen style
- Thread-safe shared state management
- Unicode-safe character handling for multi-byte emojis and symbols
- Hidden State: Allow the bar to "finish and disappear" or "finish and persist" based on a configuration flag. Users should also be able to call this function manually.
- Success/Failure States: Add a .finish_with_message("Done!") or .abandon() method that changes the bar color to green (success) or red (error) once the loop ends.
- Nested Bars: Provide a MultiProgress manager that handles the drawing offsets automatically, so users can just call multi.add(pb) without manually calculating line offsets.
- No-terminal Mode: If the program is piped into a file (e.g., myapp > log.txt), the library should detect this and stop printing ANSI escape codes/animations to avoid cluttering the log file with "garbage" characters.
- Smoothing (EMA): Currently, if one loop iteration takes 5 seconds and the next takes 0.1 seconds, the ETA will jump wildly. Implement an Exponential Moving Average for the speed calculation so the ETA remains steady.