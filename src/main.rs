use clap::Parser;
use config::Config;
use std::io::{self, BufRead, IsTerminal};

mod collect;
mod config;
mod templates;
mod utils;

use crate::collect::QuickCOutput;
use crate::templates::Template;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Language to use config from
    #[arg(short, long)]
    lang: String,

    /// Input to be passed in to the command
    #[arg(value_name = "INPUT")]
    input: Option<String>,

    /// Show the input that was used to run
    #[arg(short, long, default_value_t = false)]
    show_input: bool,

    /// Only show the raw input and output
    #[arg(short, long, default_value_t = false)]
    raw: bool,

    /// Don't show the prefix
    #[arg(short, long, default_value_t = false)]
    no_prefix: bool,
}

pub fn is_interactive() -> bool {
    io::stdin().is_terminal()
}

pub fn collect_stdin() -> Vec<String> {
    let mut buf = Vec::new();

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        buf.push(line.expect("Could not convert to line"));
    }

    buf
}

fn main() {
    let cli = Cli::parse();

    let lang = cli.lang;
    let raw = cli.raw;
    let show_input = cli.show_input;

    let input_vec: Vec<String>;

    if is_interactive() {
        if let Some(input) = cli.input {
            input_vec = input.lines().map(|s| s.to_string()).collect();
        } else {
            exit("No input found. Please provide an input!", 1);
        }
    } else {
        input_vec = collect_stdin();
    }

    let config = Config::from_config();

    if let Some(lang_conf) = config.get_lang_conf(&lang) {
        let template = Template::new(&lang, lang_conf, input_vec.clone());
        let result = QuickCOutput::controller(&template);
        if let Ok(output) = result {
            let prefix_opt: Option<String>;

            if cli.no_prefix {
                prefix_opt = None;
            } else {
                prefix_opt = lang_conf.get_prefix();
            }

            let input_opt;

            if show_input {
                input_opt = Some(input_vec);
            } else {
                input_opt = None;
            }

            output.output(input_opt, raw, prefix_opt);
        } else {
            exit("Could not write tmp file!", 1);
        }
    } else {
        exit(
            &format!("Language {} does not exist in your config!", lang),
            1,
        );
    }
}

fn exit(message: &str, code: i32) -> ! {
    eprintln!("{}", message);
    std::process::exit(code);
}
