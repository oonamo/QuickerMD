use clap::Parser;
use config::Config;

mod collect;
mod config;
mod templates;
mod utils;
mod variables;
mod cli;

use crate::collect::QuickMDOutput;
use crate::templates::Template;

fn main() {
    let cli = cli::Cli::parse();

    let lang = cli.lang;
    let raw = cli.raw;
    let show_input = cli.show_input;

    let input_vec: Vec<String>;

    if cli::is_interactive() {
        if let Some(input) = cli.input {
            input_vec = input.lines().map(|s| s.to_string()).collect();
        } else {
            exit("No input found. Please provide an input!", 1);
        }
    } else {
        input_vec = cli::collect_stdin();
    }

    let config = Config::from_config();

    if let Some(lang_conf) = config.get_lang_conf(&lang) {
        let template = Template::new(&lang, lang_conf, input_vec.clone());
        let result = QuickMDOutput::start(&template);
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
            exit(
                &format!("An error occured while running!: \n{}", result.unwrap_err()),
                1,
            );
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
