use crate::cli;
use crate::utils::exit;
use crate::config::{Config, LanguageConfig};

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

pub fn lang_conf<'lang>(
    config: &'lang Config,
    lang: &str
    ) -> &'lang LanguageConfig {
    if let Some(lang_conf) = config.get_lang_conf(lang) {
        lang_conf
    } else {
        exit(&format!("Language Config does not exist for '{}'", lang), 1);
    }
}
