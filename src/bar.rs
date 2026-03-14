use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

use crate::iter::ProgressBarIter;

// Hidden from the user, but accessible to internal modules
pub(crate) struct SharedState {
    pub(crate) total: usize,
    pub(crate) current: usize,
    pub(crate) width: usize,
    pub(crate) fill_char: char,
    pub(crate) empty_char: char,
    pub(crate) desc: String,
    pub(crate) postfix: String,
}

impl SharedState {
    pub(crate) fn print(&self) {
        let percent = if self.total == 0 { 1.0 } else { self.current as f64 / self.total as f64 };
        let filled_len = (percent * self.width as f64).round() as usize;
        let empty_len = self.width.saturating_sub(filled_len);

        let filled_bar = self.fill_char.to_string().repeat(filled_len);
        let empty_bar = self.empty_char.to_string().repeat(empty_len);

        let prefix = if self.desc.is_empty() { String::new() } else { format!("{}: ", self.desc) };
        let suffix = if self.postfix.is_empty() { String::new() } else { format!(", {}", self.postfix) };

        print!(
            "\r{}|{}{}| {}% [{}/{}{}]{}",
            prefix, filled_bar, empty_bar, (percent * 100.0) as usize,
            self.current, self.total, suffix, "\x1b[K"
        );
        io::stdout().flush().unwrap();
    }

    pub(crate) fn write(&self, msg: &str) {
        print!("\r\x1b[K"); // Clear current line
        println!("{}", msg);  // Print message
        self.print();         // Redraw bar
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
                fill_char: '█',
                empty_char: '-',
                desc: String::new(),
                postfix: String::new(),
            })),
        }
    }

    pub fn width(self, width: usize) -> Self {
        self.state.borrow_mut().width = width;
        self
    }

    pub fn fill_char(self, c: char) -> Self {
        self.state.borrow_mut().fill_char = c;
        self
    }

    pub fn empty_char(self, c: char) -> Self {
        self.state.borrow_mut().empty_char = c;
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