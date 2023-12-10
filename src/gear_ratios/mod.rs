use std::{fs, path::PathBuf};

use crate::{options::Stage, solver::Solver};

use self::easy::find_symbol_adjacent_numbers;

mod easy;
mod hard;
mod number_data;
mod point;

pub struct GearRatiosSolver {
    file: PathBuf,
    stage: Stage,
}

impl Solver for GearRatiosSolver {
    fn new(file: PathBuf, stage: Stage) -> Self
    where
        Self: Sized,
    {
        GearRatiosSolver { file, stage }
    }

    fn solve(&self) -> Option<String> {
        let input = fs::read_to_string(self.file.to_owned()).expect("file should exist");

        let result = match self.stage {
            Stage::Easy => find_symbol_adjacent_numbers(input)?,
            Stage::Hard => todo!(),
        };

        Some(result.to_string())
    }
}
