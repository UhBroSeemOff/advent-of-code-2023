pub mod trebuchet;

use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use trebuchet::TrebuchetSolver;

pub fn solve_task() -> Option<()> {
    let options = Options::parse();

    let solver = match options.command_group.task {
        Task::Trebuchet => TrebuchetSolver::new(options.file, options.stage),
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
struct Command {
    #[clap(value_enum)]
    pub task: Task,
}

#[derive(Parser)]
struct Options {
    #[clap(flatten)]
    command_group: Command,
    file: std::path::PathBuf,
    #[clap(short, long, value_enum)]
    pub stage: Stage,
}
