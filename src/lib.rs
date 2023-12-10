mod cube_game;
mod gear_ratios;
mod options;
mod solver;
mod trebuchet;

use clap::Parser;

use cube_game::CubeGameSolver;
use trebuchet::TrebuchetSolver;

use crate::{gear_ratios::GearRatiosSolver, options::*, solver::Solver};

pub fn solve_task() -> Option<()> {
    let options = Options::parse();
    let file = options.file;
    let stage = options.stage.unwrap_or_default();

    let solver: Box<dyn Solver> = match options.task {
        Task::Trebuchet => Box::new(TrebuchetSolver::new(file, stage)),
        Task::CubeGame => Box::new(CubeGameSolver::new(file, stage)),
        Task::GearRatios => Box::new(GearRatiosSolver::new(file, stage)),
    };

    let answer = solver.solve()?;

    println!("Result: {answer}");

    Some(())
}
