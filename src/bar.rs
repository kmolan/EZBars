use std::cell::RefCell;
use std::io::{stdout, IsTerminal, Write};
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::iter::ProgressBarIter;
use crate::style::Style;

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
    pub(crate) style: Style,
    pub(crate) desc: String,
    pub(crate) postfix: String,
    pub(crate) start_time: Option<Instant>,
    pub(crate) clear_on_finish: bool,
    pub(crate) status: Status,
    pub(crate) final_message: Option<String>,
    pub(crate) offset: usize, // Vertical distance from the bottom line
    pub(crate) is_terminal: bool,
    pub(crate) smoothed_speed: f64, // Persisted EMA speed
}

impl SharedState {
    // Helper to format seconds into 00:00
    fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    fn compute_eta(&mut self) -> String {
        if self.status != Status::Running {
            return String::new();
        }

        let mut time_info = String::new();

        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            let elapsed_secs = elapsed.as_secs_f64();
            let elapsed_str = Self::format_duration(elapsed);

            // Calculate instantaneous speed
            let instant_speed = if elapsed_secs > 0.0 {
                self.current as f64 / elapsed_secs
            } else {
                0.0
            };

            // EMA Smoothing Logic: S_t = α * v_t + (1 - α) * S_{t-1}
            let alpha = 0.2; // Smoothing factor (0.0 to 1.0)
            if self.smoothed_speed == 0.0 {
                self.smoothed_speed = instant_speed;
            } else {
                self.smoothed_speed = (alpha * instant_speed) + (1.0 - alpha) * self.smoothed_speed;
            }

            // Calculate ETA using smoothed speed
            let eta_str = if self.smoothed_speed > 0.0 && self.current < self.total {
                let remaining = self.total - self.current;
                let eta_duration = Duration::from_secs_f64(remaining as f64 / self.smoothed_speed);
                Self::format_duration(eta_duration)
            } else {
                "??:??".to_string()
            };

            time_info = format!(
                " [{} < {}, {:.2} it/s]",
                elapsed_str, eta_str, self.smoothed_speed
            );
        }

        time_info
    }

    /// Determines the boundary characters based on the current style
    fn get_boundary_characters(&self) -> (&str, &str) {
        match self.style {
            Style::AsciiSpinner
            | Style::BrailleSpinner
            | Style::Pacman
            | Style::ModernSlim(..)
            | Style::Gradient(..)
            | Style::Marquee(..) => ("", ""),
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
        print!("\r\x1b[K");
        let _ = std::io::stdout().flush();
    }

    pub(crate) fn print(&mut self) {
        if !self.is_terminal {
            return;
        }

        let eta_info = self.compute_eta(); // Updates smoothed_speed internally
        let mut bar_string = self.style.render(self.width, self.current, self.total);
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
            eta_info,
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
    /// Initializes a new ProgressBar with default settings:
    /// 40-character width, Fractional style, and no initial total.
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(SharedState {
                total: 0,
                current: 0,
                width: 40,
                style: Style::default(),
                desc: String::new(),
                postfix: String::new(),
                start_time: None,
                clear_on_finish: false,
                status: Status::Running,
                final_message: None,
                offset: 0,
                is_terminal: stdout().is_terminal(),
                smoothed_speed: 0.0,
            })),
        }
    }

    /// Sets the horizontal width of the bar (in terminal columns).
    pub fn width(self, width: usize) -> Self {
        self.state.borrow_mut().width = width;
        self
    }

    pub fn style(self, style: Style) -> Self {
        self.state.borrow_mut().style = style;
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

    pub fn clear_on_finish(self, clear: bool) -> Self {
        self.state.borrow_mut().clear_on_finish = clear;
        self
    }

    pub fn finish_and_clear(&self) {
        self.state.borrow_mut().clear_line();
    }

    pub fn finish_with_message(&self, msg: &str) {
        let mut state = self.state.borrow_mut();
        state.status = Status::Success;
        state.current = state.total;
        state.final_message = Some(msg.to_string());

        if state.is_terminal {
            state.print();
        } else {
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
            println!("{}: [FAILURE] {}", state.desc, msg);
        }
    }

    pub fn total(self, total: usize) -> Self {
        self.state.borrow_mut().total = total;
        self
    }

    pub fn inc(&self, amount: usize) {
        let mut state = self.state.borrow_mut();

        if state.start_time.is_none() {
            state.start_time = Some(std::time::Instant::now());
        }

        state.current = (state.current + amount).min(state.total);
        state.print();
    }

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
