use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::{collections::HashMap, fs::File, path::PathBuf};

/// The Config Struct
#[derive(Deserialize, Debug)]
pub struct Config {
    langs: HashMap<String, LanguageConfig>,
}

#[derive(Deserialize, Debug)]
pub struct LanguageConfig {
    #[serde(rename = "template")]
    /// The user's defined template
    raw_template: Option<String>,

    /// The prefix to output the data with,
    /// if the user wishes
    prefix: Option<String>,

    /// The file extension to use when creating a file
    /// for this language
    extension: Option<String>,

    #[serde(rename = "command")]
    /// The command that will be ran
    /// as the first step
    compile_command: Vec<String>,

    #[serde(rename = "run")]
    /// The second run step
    run_command: Option<RunCommandType>,

    #[serde(default)]
    /// Whether to redirect input into compile_command
    redir_input: bool,

    #[serde(skip)]
    // The parsed template
    template: Template,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RunCommandType {
    Bool(bool),
    StringVec(Vec<String>),
}

#[derive(Debug)]
pub struct Template {
    /// The parsed lines for template
    lines: Vec<String>,

    /// The user input
    input: Vec<String>,

    /// The resolved template
    resolved_template: String,

    is_resolved: bool,
}

#[derive(Deserialize, Serialize)]
pub enum OrderedType {
    Ascending,
    Descending,
}

#[derive(Deserialize, Serialize)]
pub enum OutputType {
    JSON(Option<OrderedType>),
    Raw,
}

impl Default for Template {
    fn default() -> Self {
        Self {
            lines: Vec::with_capacity(0),
            input: Vec::with_capacity(0),
            resolved_template: String::new(),
            is_resolved: false,
        }
    }
}

impl Config {
    pub fn get_config() -> Result<Self, String> {
        let path = ProjectDirs::from("", "", "QuickMD")
            .ok_or("Could not resolve project directory")?
            .config_local_dir()
            .join("config.toml");

        Config::get_config_from_path(path)
    }
    pub fn get_config_from_path(path: PathBuf) -> Result<Self, String> {
        let config_contents = std::fs::read_to_string(path.clone()).map_err(|e| {
            format!(
                "Error reading file '{}':\n{}",
                path.to_str().unwrap_or_default(),
                e.to_string()
            )
        })?;

        let config = toml::from_str::<Config>(&config_contents).map_err(|e| {
            format!(
                "There was an error reading the config '{}':\n{}",
                path.to_str().unwrap_or_default(),
                e.to_string()
            )
        })?;

        Ok(config)
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

    pub fn get_redir_input(&self) -> bool {
        self.redir_input
    }

    pub fn get_extension(&self) -> Option<String> {
        self.extension.clone()
    }

    pub fn get_raw_template(&self) -> Option<String> {
        self.raw_template.clone()
    }

    pub fn get_template(&self) -> Option<&Template> {
        if let Some(_) = self.raw_template {
            return Some(&self.template);
        }
        None
    }

    pub fn explicit_no_run(&self) -> bool {
        match &self.run_command {
            Some(command_type) =>{
                match command_type {
                    RunCommandType::Bool(val) => !val,
                    _ => false,
                }
            } ,
            None => false,
        }
    }
}

impl Template {
    pub fn new(lang: &str, input: Vec<String>, lang_conf: &LanguageConfig) -> Self {
        Template {
            lines: lang_conf
                .get_raw_template()
                .unwrap_or("".to_string())
                .lines()
                .map(|s| s.to_string())
                .collect(),
            resolved_template: String::new(),
            is_resolved: false,
            input,
        }
    }

    pub fn get_input(&self) -> Vec<String> {
        self.input.clone()
    }

    pub fn write_to_file(&self, path: PathBuf) -> std::io::Result<()> {
        let mut file = File::create(path.clone())?;
        file.write_all(self.to_string().as_bytes())
    }

    fn resolve_no_mut(&self) -> String {
        if self.is_resolved {
            return self.resolved_template.clone();
        }

        let new_string = String::new();
        let input_collected = false;

        let mut resolved_string = self.lines.join("\n");
        resolved_string.replace("{{INPUT}}", &self.input.join("\n"))
    }

    fn resolve(&mut self) {
        self.resolved_template = self.resolve_no_mut();
        self.is_resolved = self.lines.len() != 0;
    }

    pub fn get_resolved_template(&self) -> String {
        if self.is_resolved {
            return self.resolved_template.clone();
        }
        self.resolve_no_mut()
    }
}

impl ToString for Template {
    fn to_string(&self) -> String {
        self.get_resolved_template()
    }
}
