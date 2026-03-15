pub mod bar;
pub mod iter;
pub mod multi;
pub mod style;

pub use bar::ProgressBar;
pub use iter::{ProgressBarIter, ProgressIterator};
pub use multi::MultiProgress;
pub use style::Style;

pub fn tqdm<I: IntoIterator>(iterable: I) -> ProgressBarIter<I::IntoIter>
where
    I::IntoIter: ExactSizeIterator,
{
    ProgressBar::new().wrap(iterable)
}
