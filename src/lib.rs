mod cube_game;
mod trebuchet;

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use cube_game::CubeGameSolver;
use trebuchet::TrebuchetSolver;

pub fn solve_task() -> Option<()> {
    let options = Options::parse();

    let solver: Box<dyn Solver> = match options.task {
        Task::Trebuchet => Box::new(TrebuchetSolver::new(options.file, options.stage)),
        Task::CubeGame => Box::new(CubeGameSolver::new(options.file, options.stage)),
    };

    solver.solve()?;
    Some(())
}

pub trait Solver {
    fn new(file: PathBuf, stage: Stage) -> Self
    where
        Self: Sized;
    fn solve(&self) -> Option<()>;
}

#[derive(ValueEnum, Clone, Copy, Debug)]
#[clap(rename_all = "kebab_case")]
enum Task {
    Trebuchet,
    CubeGame,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default)]
#[clap(rename_all = "kebab_case")]
pub enum Stage {
    #[default]
    Easy,
    Hard,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct Command {}

#[derive(Parser)]
struct Options {
    #[clap(value_enum)]
    pub task: Task,
    file: std::path::PathBuf,
    #[clap(short, long, value_enum)]
    pub stage: Stage,
}
