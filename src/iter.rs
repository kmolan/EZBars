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
        let mut state = self.state.borrow_mut();

        // Start the timer on the first call to next()
        if state.start_time.is_none() {
            state.start_time = Some(std::time::Instant::now());
        }

        if item.is_some() {
            state.current += 1;
            state.print();
        } else {
            // We reached the end of the progress bar
            if state.clear_on_finish {
                state.clear_line();
            }
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
