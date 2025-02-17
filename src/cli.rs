use clap::{Args, Parser, Subcommand};
use std::io::{self, BufRead, IsTerminal};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub actions: QuickerActions,
}

#[derive(Subcommand)]
pub enum QuickerActions {
    /// Dumps the template
    DumpTemplate(DumpArgs),

    /// Runs the template + input
    Run(RunArgs),
}

#[derive(Args)]
pub struct DumpArgs {
    /// The language to dump its template
    pub lang: String,

    /// Whether to remove the template lines
    // TODO: Refactor template to add the ability to trim those lines
    #[arg(short, long, default_value_t = false)]
    pub with_input: bool,

    /// Show the input that was used to run
    pub input: Option<String>,
}

#[derive(Args)]
pub struct RunArgs {
    pub lang: String,
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
