use clap::Parser;
use config::Config;

mod cli;
mod collect;
mod config;
mod templates;
mod utils;
mod variables;

use crate::collect::QuickMDOutput;
use crate::templates::Template;

fn exit(message: &str, code: i32) -> ! {
    eprintln!("{}", message);
    std::process::exit(code);
}

fn dump_template(config: &Config, args: &cli::DumpArgs) {
    if let Some(lang_conf) = config.get_lang_conf(&args.lang) {
        let template = if let Some(temp) = lang_conf.get_template() {
            temp
        } else {
            exit(&format!("Template does not exist for '{}'", args.lang), 1)
        };

        println!("{}", template);
    } else {
        exit(&format!("No Language Config for '{}'", args.lang), 1)
    }
}

fn run_input(config: &Config, args: &cli::RunArgs) {
    let lang_conf = if let Some(conf) = config.get_lang_conf(&args.lang) {
        conf
    } else {
        exit(&format!("No Language Config for '{}'", args.lang), 1)
    };

    let input_vec: Vec<String>;

    if cli::is_interactive() {
        if let Some(input) = args.input.clone() {
            input_vec = input.lines().map(|s| s.to_string()).collect();
        } else {
            exit(&format!("No Input detected'{}'", args.lang), 1)
        }
    } else {
        input_vec = cli::collect_stdin();
    }

    let template = Template::new(&args.lang, lang_conf, input_vec.clone());
    let output = QuickMDOutput::start(&template).map_err(|e| {
        exit(&format!("There was an error running the program\n{}", e.to_string()), 1);
    }).unwrap();

    let prefix_opt = if args.no_prefix {
        None
    } else {
        lang_conf.get_prefix()
    };
    let input_opt = if args.show_input {
        Some(input_vec)
    } else {
        None
    };

    output.output(input_opt, args.raw, prefix_opt);
}

fn main() {
    let cli = cli::Cli::parse();

    let config = Config::from_config();
    match cli.actions {
        cli::QuickerActions::DumpTemplate(args) => dump_template(&config, &args),
        cli::QuickerActions::Run(args) => run_input(&config, &args),
    }
}
