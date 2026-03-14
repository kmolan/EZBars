use std::cell::RefCell;
use std::rc::Rc;

use crate::bar::{ProgressBar, SharedState};

pub struct ProgressBarIter<I> {
    pub(crate) iter: I,
    pub(crate) state: Rc<RefCell<SharedState>>,
}

impl<I> Iterator for ProgressBarIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next();

        if item.is_some() {
            let mut state = self.state.borrow_mut();
            state.current += 1;
            state.print();
        } else {
            println!(); // Don't overwrite the final bar
        }

        item
    }
}

// Extension trait to allow `(0..100).progress()` syntax!
pub trait ProgressIterator: ExactSizeIterator + Sized {
    fn progress(self) -> ProgressBarIter<Self>;
}

impl<I: ExactSizeIterator> ProgressIterator for I {
    fn progress(self) -> ProgressBarIter<Self> {
        ProgressBar::new().wrap(self)
    }
}