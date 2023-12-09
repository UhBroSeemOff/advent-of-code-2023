use std::{fs, path::PathBuf};

use crate::{Solver, Stage};

use self::{easy::find_numbers_easy_way, hard::find_numbers_hard_way};

mod easy;
mod hard;

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
        let file_content = fs::read_to_string(self.file.to_owned()).expect("file should exist");

        let lines = file_content.lines();
        let result: u32 = lines
            .map(|line| find_numbers(line, self.stage).expect("number should be parsed"))
            .sum();

        println!("Result: {result}");
        Some(())
    }
}

fn find_numbers(input: &str, stage: Stage) -> Option<u32> {
    match stage {
        Stage::Easy => find_numbers_easy_way(input),
        Stage::Hard => find_numbers_hard_way(input),
    }
}

fn extract_numeric_at(source: &str, index: usize) -> Option<u32> {
    let number_as_char = source.chars().nth(index).expect("char should exist");
    let result = number_as_char
        .to_digit(10)
        .expect("char should be convertable to numeric");

    return Some(result);
}
