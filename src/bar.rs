use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::iter::ProgressBarIter;
use crate::theme::Theme;

pub(crate) struct SharedState {
    pub(crate) total: usize,
    pub(crate) current: usize,
    pub(crate) width: usize,
    pub(crate) theme: Theme,
    pub(crate) desc: String,
    pub(crate) postfix: String,
    pub(crate) start_time: Option<Instant>,
    pub(crate) clear_on_finish: bool,
}

impl SharedState {
    // Helper to format seconds into 00:00
    fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    pub(crate) fn clear_line(&self) {
        // \r moves to start, \x1b[K clears from cursor to end of line
        print!("\r\x1b[K");
        let _ = std::io::stdout().flush();
    }

    pub(crate) fn print(&self) {
        let percent = if self.total == 0 { 1.0 } else { self.current as f64 / self.total as f64 };
        let bar_string = self.theme.render(self.width, self.current, self.total);

        // --- Timing Logic ---
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

        let prefix = if self.desc.is_empty() { String::new() } else { format!("{}: ", self.desc) };
        let suffix = if self.postfix.is_empty() { String::new() } else { format!(", {}", self.postfix) };

        let (left, right) = match self.theme {
            Theme::Spinner | Theme::Claude | Theme::Pacman | Theme::DualColor(..) | Theme::Gradient(..) => ("", ""),
            _ => ("|", "|"),
        };

        // Standard stats (Percent and Count)
        let stats = match self.theme {
            Theme::Spinner | Theme::Claude => String::new(),
            _ => format!(" {:>3}% [{}/{}]", (percent * 100.0) as usize, self.current, self.total),
        };

        print!(
            "\r{}{}{}{}{}{}{}{}",
            prefix, left, bar_string, right, stats, time_info, suffix, "\x1b[K"
        );
        io::stdout().flush().unwrap();
    }

    pub(crate) fn write(&self, msg: &str) {
        print!("\r\x1b[K"); 
        println!("{}", msg);  
        self.print();         
    }
}

#[derive(Clone)]
pub struct ProgressBar {
    pub(crate) state: Rc<RefCell<SharedState>>,
}

impl Default for ProgressBar {
    fn default() -> Self { Self::new() }
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

    pub fn write<S: AsRef<str>>(&self, msg: S) {
        self.state.borrow().write(msg.as_ref());
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

    pub fn wrap<I: IntoIterator>(&self, iterable: I) -> ProgressBarIter<I::IntoIter> 
    where 
        I::IntoIter: ExactSizeIterator 
    {
        let iter = iterable.into_iter();
        self.state.borrow_mut().total = iter.len();

        ProgressBarIter {
            iter,
            state: Rc::clone(&self.state), 
        }
    }
}