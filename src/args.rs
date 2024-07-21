use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
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
}
