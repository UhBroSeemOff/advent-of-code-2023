use std::path::PathBuf;

use crate::{Solver, Stage};

pub struct CubeGameSolver {
    file: PathBuf,
    stage: Stage,
}

impl Solver for CubeGameSolver {
    fn new(file: std::path::PathBuf, stage: crate::Stage) -> Self
    where
        Self: Sized,
    {
        CubeGameSolver { file, stage }
    }

    fn solve(&self) -> Option<()> {
        todo!()
    }
}
