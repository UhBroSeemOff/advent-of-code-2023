mod data;
mod parser;

use std::{fs, path::PathBuf};

use crate::{Solver, Stage};

use self::{
    data::{Game, GameSet},
    parser::Parser,
};

pub struct CubeGameSolver {
    file: PathBuf,
    stage: Stage,
}

const PRESET: GameSet = GameSet {
    red: 12,
    green: 13,
    blue: 14,
};

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

    fn solve(&self) -> Option<()> {
        let file_content = fs::read_to_string(self.file.to_owned()).expect("file should exist");

        let lines = file_content.lines();

        let games: Vec<Game> = lines.map(|line| Game::parse(line)).collect();

        let possible_games = games.iter().filter(|game| is_game_possible(game));
        let result: u32 = possible_games.map(|game| game.id).sum();

        println!("Result: {result}");

        return Some(());
    }
}

fn is_game_possible(input: &Game) -> bool {
    input.subsets.iter().all(is_set_possible)
}

fn is_set_possible(input: &GameSet) -> bool {
    input.blue <= PRESET.blue && input.green <= PRESET.green && input.red <= PRESET.red
}
