use clap::{Args, Parser, Subcommand, ValueEnum};
use quickermd::output::OutputType;
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

    /// Format style
    #[arg(value_enum, short, long, default_value_t = OutputFormat::Pretty)]
    pub format: OutputFormat,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum OutputFormat {
    Json,
    JsonPretty,
    Pretty,
    Comment,
    Raw,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Pretty
    }
}

impl From<String> for OutputFormat {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "json-pretty" => OutputFormat::JsonPretty,
            "pretty" => OutputFormat::Pretty,
            "comment" => OutputFormat::Comment,
            _ => OutputFormat::Raw,
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OutputFormat::Json => "json",
            OutputFormat::JsonPretty => "json-pretty",
            OutputFormat::Pretty => "pretty",
            OutputFormat::Comment => "comment",
            OutputFormat::Raw => "raw",
        };
        write!(f, "{}", str)
    }
}

impl Into<OutputType> for OutputFormat {
    fn into(self) -> OutputType {
        match self {
            OutputFormat::Json => OutputType::JSON,
            OutputFormat::JsonPretty => OutputType::JsonPretty,
            _ => OutputType::Raw,
        }
    }
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
