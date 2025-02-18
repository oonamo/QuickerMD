use crate::{cli, outputer::OutputArgs};
use quickermd::output::Output;

pub fn input(input: &Option<String>) -> Option<Vec<String>> {
    let is_interacitve = cli::is_interactive();
    let input_vec: Vec<String>;

    if let Some(input) = input {
        input_vec = input.lines().map(|s| s.to_string()).collect();
    } else if !is_interacitve {
        input_vec = cli::collect_stdin();
    } else {
        return None;
    }

    Some(input_vec)
}

pub fn output(input: String, output: &Output, show_input: bool) -> OutputArgs {
    let mut output_config =
        OutputArgs::get_config(input.clone(), output.get_stdout(), output.get_stderr())
            .unwrap_or_else(|_| {
                let mut conf = OutputArgs::default();
                conf.set_reserved_section_values(input, output.get_stdout(), output.get_stderr());
                conf
            });

    if !show_input {
        output_config
            .get_input()
            .resolve_value(String::with_capacity(0));
    }

    output_config
}
