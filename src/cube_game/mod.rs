mod data;
mod easy;
mod hard;
mod parser;

use std::{fs, path::PathBuf};

use crate::{cube_game::easy::find_possible_games_sum, Solver, Stage};

use self::{data::Game, hard::find_minimal_cubes_quantities, parser::Parser};

pub struct CubeGameSolver {
    file: PathBuf,
    stage: Stage,
}

enum Colors {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Solver for CubeGameSolver {
    fn new(file: std::path::PathBuf, stage: crate::Stage) -> Self
    where
        Self: Sized,
    {
        CubeGameSolver { file, stage }
    }

    fn solve(&self) -> Option<String> {
        let file_content = fs::read_to_string(self.file.to_owned()).expect("file should exist");

        let lines = file_content.lines();

        let games: Vec<Game> = lines.map(|line| Game::parse(line)).collect();

        let result = match self.stage {
            Stage::Easy => find_possible_games_sum(&games)?,
            Stage::Hard => find_minimal_cubes_quantities(&games)?,
        };

        return Some(result.to_string());
    }
}
