use std::path::PathBuf;

use crate::options::Stage;

pub trait Solver {
    fn new(file: PathBuf, stage: Stage) -> Self
    where
        Self: Sized;
    fn solve(&self) -> Option<String>;
}
