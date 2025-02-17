use crate::cli;

pub fn input(input: &Option<String>) -> Option<Vec<String>> {
    let is_interacitve = cli::is_interactive();
    let input_vec: Vec<String>;

    if let Some(input) = input {
        input_vec = input.lines().map(|s| s.to_string()).collect();
    } else if !is_interacitve {
        input_vec = cli::collect_stdin();
    } else {
        return None
    }

    Some(input_vec)
}
