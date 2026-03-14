use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

use crate::iter::ProgressBarIter;
use crate::theme::Theme; // Import our new theme!

pub(crate) struct SharedState {
    pub(crate) total: usize,
    pub(crate) current: usize,
    pub(crate) width: usize,
    pub(crate) theme: Theme, // Replaced fill_char and empty_char
    pub(crate) desc: String,
    pub(crate) postfix: String,
}

impl SharedState {
    pub(crate) fn print(&self) {
        let percent = if self.total == 0 { 1.0 } else { self.current as f64 / self.total as f64 };
        
        let bar_string = self.theme.render(self.width, self.current, self.total);

        let prefix = if self.desc.is_empty() { String::new() } else { format!("{}: ", self.desc) };
        let suffix = if self.postfix.is_empty() { String::new() } else { format!(", {}", self.postfix) };

        // 1. Dynamically decide on boundary characters
        let (left_bracket, right_bracket) = match self.theme {
            Theme::Spinner | Theme::Claude => ("", ""), // No boundaries for spinners!
            _ => ("|", "|"),                            // Standard boundaries for bars
        };

        let stats = match self.theme {
            Theme::Spinner | Theme::Claude => String::new(), // Hide stats for spinners
            _ => format!(" {}% [{}/{}]", (percent * 100.0) as usize, self.current, self.total),
        };

        // 2. Update the print! macro to use our new dynamic brackets
        print!(
            "\r{}{}{}{}{}{}{}",
            prefix, left_bracket, bar_string, right_bracket, stats, suffix, "\x1b[K"
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