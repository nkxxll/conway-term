use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Database(DB),
    Play(Play),
}

#[derive(Args)]
pub(crate) struct DB {
    #[arg(
        short,
        long,
        default_value_t = false,
        action,
        help = "list all saved games"
    )]
    pub(crate) list: bool,
    #[arg(short, long, help = "get saved game at index")]
    pub(crate) get: Option<usize>,
}

#[derive(Args)]
pub(crate) struct Play {
    #[arg(short, long, help = "number of rounds")]
    pub(crate) rounds: Option<usize>,
    #[arg(
        short,
        long,
        default_value_t = false,
        requires = "rounds",
        action,
        help = "run silent"
    )]
    pub(crate) silent: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        requires = "rounds",
        action,
        help = "save the run in a database"
    )]
    pub(crate) based: bool,
}
