use clap::Parser;
use config::Config;

mod cli;
mod collect;
mod config;
mod templates;
mod utils;
mod variables;
mod resolver;

use crate::collect::QuickMDOutput;
use crate::templates::Template;

fn dump_template(config: &Config, args: &cli::DumpArgs) {
    let lang_conf = resolver::lang_conf(config, &args.lang);
    let template_string = if let Some(temp) = lang_conf.get_template() {
        temp
    } else {
        utils::exit(&format!("Template does not exist for '{}'", args.lang), 1)
    };

    let input_vec = resolver::input(&args.input);

    if let Some(input) = input_vec {
        let template = Template::new(&args.lang, lang_conf, input.clone());
        println!("{}", template.get_conf().to_string_from_input(input));
        return;
    } else if args.remove_template_lines {
        let template = Template::new(&args.lang, lang_conf, Vec::with_capacity(0));
        println!("{}", template.get_conf().to_string_from_input(Vec::with_capacity(0)));
        return;
    }

    println!("{}", template_string);
}

fn run_input(config: &Config, args: &cli::RunArgs) {
    let lang_conf = resolver::lang_conf(config, &args.lang);
    let Some(input_vec) = resolver::input(&args.input) else { utils::exit("No Input Found", 1); };

    let template = Template::new(&args.lang, lang_conf, input_vec.clone());
    let output = QuickMDOutput::start(&template)
        .map_err(|e| {
            utils::exit(
                &format!("There was an error running the program\n{}", e.to_string()),
                1,
            );
        })
        .unwrap();

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
