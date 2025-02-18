use clap::Parser;
use quickermd::output::{Output, OutputType};
use quickermd::QuickerMD;
use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

mod cli;
mod outputer;
mod resolver;
mod utils;
use crate::cli::OutputFormat;
use crate::outputer::OutputArgs;

fn dump_template(quicker: &QuickerMD, args: &cli::DumpArgs) {
    let template;
    let mut has_input = false;

    if let Some(input) = resolver::input(&args.input) {
        has_input = true;
        template = quicker.get_template(&args.lang, input);
    } else {
        template = quicker.get_template(&args.lang, Vec::with_capacity(0));
    }

    if let Some(tmpl) = template {
        let output;
        if has_input {
            output = tmpl.to_string();
        } else {
            output = tmpl.get_template_lines().join("\n");
        }

        println!("{}", output);
    } else {
        utils::exit(&format!("No template for `{}`", args.lang), 1);
    }
}

fn output_pretty(input: String, output: &Output, show_input: bool) -> std::io::Result<()> {
    let output_config = resolver::output(input, output, show_input);
    output_config.write_pretty_to_console()?;

    Ok(())
}

fn output_as_comment(quicker: &mut QuickerMD, input: String, output: &Output, args: &cli::RunArgs) {
    let output_config = resolver::output(input, output, args.show_input);
    let comment = quicker
        .get_config_for_lang(&args.lang)
        .unwrap()
        .get_prefix()
        .unwrap_or("".to_string());

    output_config.write_as_comment(&comment);
}

fn output_raw(input: String, output: &Output, args: &cli::RunArgs) {
    let output_config = resolver::output(input, output, args.show_input);
    output_config.write_as_comment("");
}

fn run_input(quicker: &mut QuickerMD, args: &cli::RunArgs) {
    let Some(input_vec) = resolver::input(&args.input) else {
        utils::exit("No Input Found", 1);
    };

    let result = quicker
        .run(&args.lang, input_vec.join("\n").to_string())
        .map_err(|e| format!("Error running `{}`:\n{}", args.lang, e.to_string()));

    if let Ok(mut output) = result {
        match &args.format {
            OutputFormat::Json | &OutputFormat::JsonPretty => {
                output.output_as(args.format.clone().into());
                println!("{}", output.to_string());
            }
            OutputFormat::Raw => output_raw(input_vec.join("\n"), &output, args),
            OutputFormat::Pretty => {
                output_pretty(input_vec.join("\n"), &output, args.show_input).unwrap()
            }
            OutputFormat::Comment => {
                output_as_comment(quicker, input_vec.join("\n"), &output, args)
            }
        }
    } else {
        println!("{}", result.err().unwrap());
    }
}

fn main() {
    let cli = cli::Cli::parse();

    // TODO: Handle error
    let mut quicker = QuickerMD::new().unwrap();

    match cli.actions {
        cli::QuickerActions::DumpTemplate(args) => dump_template(&quicker, &args),
        cli::QuickerActions::Run(args) => run_input(&mut quicker, &args),
    }
}
