use clap::Parser;
use std::io::{self, BufRead, IsTerminal};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Language to use config from
    #[arg(short, long)]
    pub lang: String,

    /// Input to be passed in to the command
    #[arg(value_name = "INPUT")]
    pub input: Option<String>,

    /// Show the input that was used to run
    #[arg(short, long, default_value_t = false)]
    pub show_input: bool,

    /// Only show the raw input and output
    #[arg(short, long, default_value_t = false)]
    pub raw: bool,

    /// Don't show the prefix
    #[arg(short, long, default_value_t = false)]
    pub no_prefix: bool,
}

pub fn is_interactive() -> bool {
    io::stdin().is_terminal()
}

pub fn collect_stdin() -> Vec<String> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    reader
        .lines()
        .map(|s| s.expect("Could not convert line to string"))
        .collect()
}
