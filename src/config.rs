use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    langs: HashMap<String, LanguageConfig>,
}

#[derive(Deserialize, Debug)]
pub struct LanguageConfig {
    template: Option<String>,
    #[serde(rename = "command")]
    compile_command: Vec<String>,

    prefix: Option<String>,
    extension: Option<String>,

    #[serde(rename = "run")]
    run_command: Option<RunCommandType>,

    #[serde(default)]
    redir_input: bool,

    #[serde(skip)]
    template_lines: Vec<String>,

    #[serde(skip)]
    template_start: usize,

    #[serde(skip)]
    template_end: usize,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RunCommandType {
    Bool(bool),
    StringVec(Vec<String>),
}

impl Config {
    pub fn from_config() -> Self {
        let path = ProjectDirs::from("", "", "QuickMD")
            .expect("Could not resolve project directory. OS may be unsupported")
            .config_local_dir()
            .join("config.toml");

        // TODO: Gracefully handle non existing config
        let config_contents = std::fs::read_to_string(path).expect("Config does not exist!");
        let mut config: Config = toml::from_str(&config_contents).expect("Could not parse to toml");

        for (_, lang_conf) in config.langs.iter_mut() {
            if let Some(lang_temp) = lang_conf.template.clone() {
                for (line, content) in lang_temp.split("\n").enumerate() {
                    if content.trim().contains("<<< TEMPLATE START") {
                        lang_conf.template_start = line;
                    } else if content.trim().contains("<<< TEMPLATE END") {
                        lang_conf.template_end = line;
                    }
                    lang_conf.template_lines.push(content.to_owned());
                }
            }
        }

        return config;
    }

    pub fn get_lang_conf(&self, lang: &str) -> Option<&LanguageConfig> {
        self.langs.get(lang)
    }
}

impl LanguageConfig {
    pub fn get_command_name(&self) -> String {
        self.compile_command[0].clone()
    }
    pub fn get_command_args(&self) -> Vec<String> {
        self.compile_command.iter().skip(1).cloned().collect()
    }
    pub fn explicit_no_run(&self) -> bool {
        match self.run_command.clone() {
            Some(command_type) => match command_type {
                RunCommandType::Bool(val) => !val,
                _ => false,
            }
            _ => false,
        }
    }
    pub fn get_run_command(&self, file: String) -> Option<(String, Vec<String>)> {
        if let Some(command_type) = self.run_command.clone() {
            return match command_type {
                RunCommandType::Bool(b) => {
                    if b {
                        Some((file, Vec::with_capacity(0)))
                    } else {
                        None
                    }
                }
                RunCommandType::StringVec(command) => Some((
                    command[0].clone(),
                    command.iter().take(1).cloned().collect(),
                )),
            };
        }
        None
    }
    pub fn get_prefix(&self) -> Option<String> {
        self.prefix.clone()
    }
    pub fn should_redir(&self) -> bool {
        self.redir_input
    }
    pub fn get_extension(&self) -> Option<String> {
        self.extension.clone()
    }
    pub fn to_string_from_input(&self, input: Vec<String>) -> String {
        let mut str = String::new();

        if self.redir_input {
            return str;
        }

        for line in self.template_lines.iter().take(self.template_start) {
            str.push_str(line);
            str.push('\n');
        }

        for line in input.iter() {
            str.push_str(line);
            str.push('\n');
        }

        for line in self.template_lines.iter().skip(self.template_end + 1) {
            str.push_str(line);
            str.push('\n');
        }

        str
    }
}
