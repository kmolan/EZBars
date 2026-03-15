use crate::bar::ProgressBar;

pub struct MultiProgress {
    bars: Vec<ProgressBar>,
}

impl Default for MultiProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiProgress {
    pub fn new() -> Self {
        Self { bars: Vec::new() }
    }

    /// Adds a bar to the manager.
    /// Existing bars are pushed "up" to make room for the new one at the bottom.
    pub fn add(&mut self, pb: ProgressBar) -> ProgressBar {
        // Every time we add a bar, we print a newline to
        // "reserve" space and move the terminal scrollback down.
        println!();

        // Increment offset for all bars already in the list
        for bar in &self.bars {
            let mut state = bar.state.borrow_mut();
            state.offset += 1;
        }

        // Newest bar starts at offset 0 (the current bottom line)
        self.bars.push(pb.clone());
        pb
    }
}
