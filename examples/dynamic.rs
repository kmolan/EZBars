use ezbars::ProgressBar;
use std::thread;
use std::time::Duration;

fn main() {
    let pb = ProgressBar::new().width(40).desc("Warming up");

    for i in pb.wrap(0..100) {
        // 1. Dynamic Prefix (Description)
        if i == 15 {
            pb.set_description("Training Model");
        } else if i == 85 {
            pb.set_description("Saving Data");
        }

        // 2. Dynamic Postfix (Metrics)
        // Calculate a fake loss value that goes down over time
        let fake_loss = 100.0 / (i as f64 + 1.0);
        pb.set_postfix(format!("loss: {:.4}", fake_loss));

        // 3. Safe Logging
        // Print warnings without breaking the progress bar visuals
        if i == 50 {
            pb.write("=> [INFO] Halfway point reached!");
        } else if i == 75 {
            pb.write("=> [WARNING] High memory usage detected.");
        }

        thread::sleep(Duration::from_millis(60));
    }

    println!("Task finished successfully.");
}
