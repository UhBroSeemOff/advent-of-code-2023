use std::path::PathBuf;

use crate::{Solver, Stage};

use self::easy::solve_easy_trebuchet_task;

pub mod easy;

pub struct TrebuchetSolver {
    file: PathBuf,
    stage: Stage,
}

impl Solver for TrebuchetSolver {
    fn new(file: std::path::PathBuf, stage: crate::Stage) -> Self
    where
        Self: Sized,
    {
        TrebuchetSolver { file, stage }
    }

    fn solve(&self) -> Option<()> {
        match self.stage {
            Stage::Easy => solve_easy_trebuchet_task(self.file.to_owned()),
            Stage::Hard => todo!(),
        }
    }
}
