use std::cell::RefCell;
use std::io::{stdout, IsTerminal, Write};
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::iter::ProgressBarIter;
use crate::theme::Theme;

#[derive(Clone, PartialEq)]
pub enum Status {
    Running,
    Success,
    Failure,
}

pub(crate) struct SharedState {
    pub(crate) total: usize,
    pub(crate) current: usize,
    pub(crate) width: usize,
    pub(crate) theme: Theme,
    pub(crate) desc: String,
    pub(crate) postfix: String,
    pub(crate) start_time: Option<Instant>,
    pub(crate) clear_on_finish: bool,
    pub(crate) status: Status,
    pub(crate) final_message: Option<String>,
    pub(crate) offset: usize, // Vertical distance from the bottom line
    pub(crate) is_terminal: bool,
}

impl SharedState {
    // Helper to format seconds into 00:00
    fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    fn compute_eta(&self) -> String {
        if self.status != Status::Running {
            // Don't bother computing if progress bar is finished
            return String::new();
        }

        let mut time_info = String::new();

        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            let elapsed_str = Self::format_duration(elapsed);

            // Calculate speed (it/s)
            let speed = if elapsed.as_secs_f64() > 0.0 {
                self.current as f64 / elapsed.as_secs_f64()
            } else {
                0.0
            };

            // Calculate ETA
            let eta_str = if speed > 0.0 && self.current < self.total {
                let remaining = self.total - self.current;
                let eta_duration = Duration::from_secs_f64(remaining as f64 / speed);
                Self::format_duration(eta_duration)
            } else {
                "??:??".to_string()
            };

            time_info = format!(" [{} < {}, {:.2} it/s]", elapsed_str, eta_str, speed);
        }

        time_info
    }

    /// Determines the boundary characters based on the current theme
    fn get_boundary_characters(&self) -> (&str, &str) {
        match self.theme {
            Theme::Spinner
            | Theme::Claude
            | Theme::Pacman
            | Theme::DualColor(..)
            | Theme::Gradient(..)
            | Theme::Sliding(..) => ("", ""),
            _ => ("|", "|"),
        }
    }

    /// Formats the progress percentage and item counts
    fn format_stats(&self) -> String {
        let percent = if self.total == 0 {
            1.0
        } else {
            self.current as f64 / self.total as f64
        };

        format!(
            " {:>3}% [{}/{}]",
            (percent * 100.0) as usize,
            self.current,
            self.total
        )
    }

    /// Generates ANSI escape codes for multi-bar vertical movement
    fn get_vertical_movement(&self) -> (String, String) {
        if self.offset > 0 {
            (
                format!("\x1b[{}A", self.offset),
                format!("\x1b[{}B", self.offset),
            )
        } else {
            (String::new(), String::new())
        }
    }

    /// Handles description prefix and final message/error suffix
    fn compute_prefix_and_suffix(&self) -> (String, String) {
        let prefix = if self.desc.is_empty() {
            String::new()
        } else {
            format!("{}: ", self.desc)
        };

        let msg = self.final_message.as_deref().unwrap_or("");
        let suffix = match self.status {
            Status::Failure if !msg.is_empty() => format!(" - Error: {}", msg),
            _ if !msg.is_empty() => format!(" - {}", msg),
            _ => String::new(),
        };

        (prefix, suffix)
    }

    pub(crate) fn clear_line(&self) {
        // \r moves to start, \x1b[K clears from cursor to end of line
        print!("\r\x1b[K");
        let _ = std::io::stdout().flush();
    }

    pub(crate) fn print(&self) {
        if !self.is_terminal {
            // If we aren't in a terminal, don't print anything during increments.
            // This prevents the log file from being filled with unnecessary garbage.
            return;
        }

        let mut bar_string = self.theme.render(self.width, self.current, self.total);
        bar_string = match self.status {
            Status::Success => format!("\x1b[32m{}\x1b[0m", bar_string), // Green
            Status::Failure => format!("\x1b[31m{}\x1b[0m", bar_string), // Red
            Status::Running => bar_string,
        };

        let (left_boundary_character, right_boundary_character) = self.get_boundary_characters();
        let (move_up, move_down) = self.get_vertical_movement();
        let stats = self.format_stats();
        let (prefix, suffix) = self.compute_prefix_and_suffix();

        print!(
            "\r{}{}{}{}{}{}{}{}\x1b[K{}",
            move_up,
            prefix,
            left_boundary_character,
            bar_string,
            right_boundary_character,
            stats,
            self.compute_eta(),
            suffix,
            move_down
        );

        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
}

#[derive(Clone)]
pub struct ProgressBar {
    pub(crate) state: Rc<RefCell<SharedState>>,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(SharedState {
                total: 0,
                current: 0,
                width: 40,
                theme: Theme::default(), // Use default theme
                desc: String::new(),
                postfix: String::new(),
                start_time: None,
                clear_on_finish: false, // Default to persist
                status: Status::Running,
                final_message: None,
                offset: 0,
                is_terminal: stdout().is_terminal(),
            })),
        }
    }

    pub fn width(self, width: usize) -> Self {
        self.state.borrow_mut().width = width;
        self
    }

    // New builder method for themes
    pub fn theme(self, theme: Theme) -> Self {
        self.state.borrow_mut().theme = theme;
        self
    }

    pub fn desc<S: Into<String>>(self, desc: S) -> Self {
        self.state.borrow_mut().desc = desc.into();
        self
    }

    pub fn set_description<S: Into<String>>(&self, desc: S) {
        self.state.borrow_mut().desc = desc.into();
    }

    pub fn set_postfix<S: Into<String>>(&self, postfix: S) {
        self.state.borrow_mut().postfix = postfix.into();
    }

    /// Configuration: Should the bar disappear when finished?
    pub fn clear_on_finish(self, clear: bool) -> Self {
        self.state.borrow_mut().clear_on_finish = clear;
        self
    }

    /// Manual trigger to hide/clear the bar
    pub fn finish_and_clear(&self) {
        self.state.borrow_mut().clear_line();
    }

    pub fn finish_with_message(&self, msg: &str) {
        let mut state = self.state.borrow_mut();
        state.status = Status::Success;
        // Snap to 100% for a clean look
        state.current = state.total;
        state.final_message = Some(msg.to_string());

        if state.is_terminal {
            state.print();
        } else {
            // Log-friendly version: "Description: [Success] Message"
            println!("{}: [SUCCESS] {}", state.desc, msg);
        }
    }

    pub fn abandon(&self, msg: &str) {
        let mut state = self.state.borrow_mut();
        state.status = Status::Failure;
        state.final_message = Some(msg.to_string());

        if state.is_terminal {
            state.print();
        } else {
            // Log-friendly version: "Description: [Success] Message"
            println!("{}: [FAILURE] {}", state.desc, msg);
        }
    }

    /// Manually set the total (useful if not using .wrap())
    pub fn total(self, total: usize) -> Self {
        self.state.borrow_mut().total = total;
        self
    }

    /// Increment the progress by a specific amount
    pub fn inc(&self, amount: usize) {
        let mut state = self.state.borrow_mut();

        // Start timer on the first manual increment
        if state.start_time.is_none() {
            state.start_time = Some(std::time::Instant::now());
        }

        state.current = (state.current + amount).min(state.total);
        state.print();
    }

    /// Set progress to a specific absolute value
    pub fn set_position(&self, pos: usize) {
        let mut state = self.state.borrow_mut();
        state.current = pos.min(state.total);
        state.print();
    }

    pub fn wrap<I: IntoIterator>(&self, iterable: I) -> ProgressBarIter<I::IntoIter>
    where
        I::IntoIter: ExactSizeIterator,
    {
        let iter = iterable.into_iter();
        self.state.borrow_mut().total = iter.len();

        ProgressBarIter {
            iter,
            state: Rc::clone(&self.state),
        }
    }
}
