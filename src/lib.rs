pub mod bar;
pub mod iter;

pub use bar::ProgressBar;
pub use iter::{ProgressBarIter, ProgressIterator};

// The classic Python-style helper
pub fn tqdm<I: IntoIterator>(iterable: I) -> ProgressBarIter<I::IntoIter> 
where 
    I::IntoIter: ExactSizeIterator 
{
    ProgressBar::new().wrap(iterable)
}