mod cube_game;
mod options;
mod solver;
mod trebuchet;

use clap::Parser;

use cube_game::CubeGameSolver;
use trebuchet::TrebuchetSolver;

use crate::{options::*, solver::Solver};

pub fn solve_task() -> Option<()> {
    let options = Options::parse();

    let solver: Box<dyn Solver> = match options.task {
        Task::Trebuchet => Box::new(TrebuchetSolver::new(
            options.file,
            options.stage.unwrap_or_default(),
        )),
        Task::CubeGame => Box::new(CubeGameSolver::new(
            options.file,
            options.stage.unwrap_or_default(),
        )),
    };

    let answer = solver.solve()?;

    println!("Result: {answer}");

    Some(())
}
