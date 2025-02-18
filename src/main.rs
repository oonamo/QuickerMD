use clap::Parser;
use quickermd::output::{Output, OutputType};
use quickermd::QuickerMD;
use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

mod cli;
mod outputer;
mod resolver;
mod utils;
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

fn output_pretty(input: String, output: Output) -> std::io::Result<()> {
    let output_config =
        OutputArgs::get_config(input.clone(), output.get_stdout(), output.get_stderr())
            .unwrap_or_else(|_| {
                let mut conf = OutputArgs::default();
                conf.set_reserved_section_values(input, output.get_stdout(), output.get_stderr());
                conf
            });

    output_config.write_to_console()?;

    Ok(())
    //let mut bufwtr = BufferWriter::stdout(ColorChoice::Always);
    //let mut buffer = bufwtr.buffer();
    //
    //if show_input {
    //    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true))?;
    //
    //    writeln!(&mut buffer, "# Output")?;
    //    buffer.reset()?;
    //
    //    writeln!(&mut buffer, "{}", output.get_stdout())?;
    //
    //    bufwtr.print(&buffer)?;
    //}
    //
    //if output.has_stdout() {
    //    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    //
    //    writeln!(&mut buffer, "# Output")?;
    //    buffer.reset()?;
    //
    //    writeln!(&mut buffer, "{}", output.get_stdout())?;
    //
    //    bufwtr.print(&buffer)?;
    //}
    //
    //if output.has_stderr() {
    //    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
    //
    //    writeln!(&mut buffer, "# Error")?;
    //    buffer.reset()?;
    //
    //    writeln!(&mut buffer, "{}", output.get_stderr())?;
    //    bufwtr.print(&buffer)?;
    //}
    //
    //buffer.reset()?;
    //Ok(())

    // TODO: Handle error case
    //buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
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
            OutputType::JSON | OutputType::JsonPretty => {
                output.output_as(args.format.clone());
                println!("{}", output.to_string());
            }
            OutputType::Raw => {
                output_pretty(input_vec.join("\n"), output).unwrap();
            }
        }
        //if !matches!(args.format, OutputType::Raw) {
        //    output.output_as(args.format.clone().into());
        //}
        //output_pretty(input_vec.join("\n"), output).unwrap();
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
