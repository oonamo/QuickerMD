use clap::Parser;
use quickermd::QuickerMD;

mod cli;
mod resolver;
mod utils;

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

fn run_input(quicker: &mut QuickerMD, args: &cli::RunArgs) {
    let Some(input_vec) = resolver::input(&args.input) else {
        utils::exit("No Input Found", 1);
    };

    let result = quicker
        .run(&args.lang, input_vec.join("\n").to_string())
        .map_err(|e| format!("Error running `{}`:\n{}", args.lang, e.to_string()));

    if let Ok(output) = result {
        println!("{}", output.to_string());
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
