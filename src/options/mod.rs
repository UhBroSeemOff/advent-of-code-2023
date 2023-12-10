use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Copy, Debug)]
#[clap(rename_all = "kebab_case")]
pub enum Task {
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

#[derive(Parser)]
pub struct Options {
    #[clap(value_enum)]
    pub task: Task,
    pub file: std::path::PathBuf,
    #[clap(short, long, value_enum)]
    pub stage: Option<Stage>,
}
