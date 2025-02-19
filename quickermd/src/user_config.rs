use directories::ProjectDirs;
use serde::Deserialize;
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
    comment: Option<String>,

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
    /// Gets the config from the default file location
    pub fn get_config() -> Result<Self, String> {
        let path = ProjectDirs::from("", "", "QuickMD")
            .ok_or("Could not resolve project directory")?
            .config_local_dir()
            .join("config.toml");

        Config::get_config_from_path(path)
    }

    /// Gets the config from a given path
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

    // Gets the associated config for the `lang`
    pub fn get_lang_conf(&self, lang: &str) -> Option<&LanguageConfig> {
        self.langs.get(lang)
    }

    // Gets the associated config for the `lang`
    pub fn get_mut_lang_conf(&mut self, lang: &str) -> Option<&mut LanguageConfig> {
        self.langs.get_mut(lang)
    }
}

impl LanguageConfig {
    /// Gets the command name for the `LanguageConfig`
    /// 
    /// ## Example
    /// ```toml
    /// [langs.c]
    /// command = ["gcc", "-o", "{{OUT}}", "{{IN}}"]
    /// ```
    /// ```
    /// use quickermd::QuickerMD;
    /// 
    /// let config = QuickerMD::new().unwrap();
    /// let c_config = config.get_config_for_lang("c").unwrap();
    /// assert_eq!(c_config.get_command_name(), "gcc");
    /// ```
    pub fn get_command_name(&self) -> String {
        self.compile_command[0].clone()
    }

    /// Gets the command arguments for the `LanguageConfig`
    /// 
    /// ## Example
    /// ```toml
    /// [langs.c]
    /// command = ["gcc", "{{IN}}", "-o", "{{OUT}}"]
    /// ```
    /// ```
    /// use quickermd::QuickerMD;
    /// 
    /// let config = QuickerMD::new().unwrap();
    /// let c_config = config.get_config_for_lang("c").unwrap();
    /// assert_eq!(c_config.get_command_args(), vec![
    ///     "{{IN}}".to_string(),
    ///     "-o".to_string(),
    ///     "{{OUT}}".to_string(),
    /// ]);
    /// ```
    pub fn get_command_args(&self) -> Vec<String> {
        self.compile_command.iter().skip(1).cloned().collect()
    }

    /// Returns the command for running the template, or `None`
    /// if the user explicitly set no run command
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

    #[cfg(test)]
    pub fn get_raw_comment_string(&self) -> Option<String> {
        self.comment.clone()
    }

    /// Sets the comment string for the `LanguageConfig`
    pub fn set_raw_comment_string(&mut self, comment: String) {
        self.comment = Some(comment);
    }

    /// Gets the comment string for the `LanguageConfig`
    pub fn get_comment_string(&self) -> Option<String> {
        if let Some(comment) = self.comment.clone() {
            if !comment.contains("%s") {
                if comment.chars().last().unwrap().is_whitespace() {
                    return Some(format!("{}%s", comment));
                }
                return Some(format!("{} %s", comment));
            }

            return Some(comment);
        }

        None
    }

    /// Resolves the comment string from an input
    pub fn resolve_comment_string(&self, with: &str) -> Option<String> {
        if let Some(comment_string) = self.comment.clone() {
            return Some(comment_string.replace("%s", with));
        }
        None
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
            Some(command_type) => match command_type {
                RunCommandType::Bool(val) => !val,
                _ => false,
            },
            None => false,
        }
    }
}

impl Template {
    pub fn new(lang: &str, input: Vec<String>, lang_conf: &LanguageConfig) -> Self {
        let input_len = input.len();

        let mut template = Template {
            lines: lang_conf
                .get_raw_template()
                .unwrap_or("".to_string())
                .lines()
                .map(|s| s.to_string())
                .collect(),
            resolved_template: String::new(),
            is_resolved: false,
            input,
        };

        if input_len != 0 {
            template.resolve();
        }

        template
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

        let resolved_string = self.lines.join("\n");
        resolved_string.replace("{{INPUT}}", &self.input.join("\n"))
    }

    fn resolve(&mut self) {
        self.resolved_template = self.resolve_no_mut();
        self.is_resolved = self.lines.len() != 0;
    }

    pub fn get_template_lines(&self) -> Vec<String> {
        self.lines.clone()
    }

    pub fn get_resolved_template(&self) -> String {
        if self.is_resolved {
            return self.resolved_template.clone();
        }
        self.resolve_no_mut()
    }

    pub fn set_input(&mut self, input: String) {
        self.set_input_from_vec(input.lines().map(|s| s.to_string()).collect());
    }

    pub fn set_input_from_vec(&mut self, input: Vec<String>) {
        self.input = input;
    }
}

impl ToString for Template {
    fn to_string(&self) -> String {
        self.get_resolved_template()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn get_exmaple_config() -> Config {
        Config::get_config_from_path(PathBuf::from("../examples/config.toml")).unwrap()
    }

    #[test]
    fn it_gets_a_config_from_path() {
        _ = get_exmaple_config();
    }

    #[test]
    fn it_gets_an_existing_language() {
        let config = get_exmaple_config();

        _ = config.get_lang_conf("c").unwrap();
    }

    #[test]
    #[should_panic]
    fn it_panics_on_non_existing_language() {
        let config = get_exmaple_config();

        config.get_lang_conf("NON EXISTING LANGUAGE").unwrap();
    }

    #[test]
    fn it_correctly_resolves_template() {
        let config = get_exmaple_config();
        let c_conf = config.get_lang_conf("c").unwrap();

        let c_raw_template = c_conf.get_raw_template().unwrap();

        assert_eq!(
            r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
  {{INPUT}}
}
"#
            .trim(),
            c_raw_template.trim(),
            "Raw template has been changed. Update this test"
        );

        let c_template = Template::new(
            "c",
            vec![r#"printf("Hello, QuickerMD!\n");"#.to_string()],
            c_conf,
        );

        let resolve_template = c_template.get_resolved_template();

        assert_eq!(
            resolve_template.trim(),
            r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
  printf("Hello, QuickerMD!\n");
}
"#
            .trim()
        )
    }

    #[test]
    pub fn it_adds_a_whitespace_to_resolved_string_if_it_dne() {
        let mut config = get_exmaple_config();

        let js_conf = config.get_mut_lang_conf("js").unwrap();

        js_conf.set_raw_comment_string("//".to_string());

        assert_eq!(js_conf.get_raw_comment_string().unwrap(), "//");
        assert_eq!(js_conf.get_comment_string().unwrap(), "// %s");
    }

    #[test]
    pub fn it_does_not_whitespace_to_comment_if_ends_with_ws() {
        let mut config = get_exmaple_config();

        let py_conf = config.get_mut_lang_conf("py").unwrap();

        py_conf.set_raw_comment_string("# ".to_string());

        assert_eq!(py_conf.get_raw_comment_string().unwrap(), "# ");
        assert_eq!(py_conf.get_comment_string().unwrap(), "# %s");
    }

    #[test]
    pub fn it_adds_formats_specifier_to_comment_if_it_dne() {
        let mut config = get_exmaple_config();

        let c_conf = config.get_mut_lang_conf("c").unwrap();

        c_conf.set_raw_comment_string("// ".to_string());

        assert_eq!(c_conf.get_raw_comment_string().unwrap(), "// ");
        assert_eq!(c_conf.get_comment_string().unwrap(), "// %s");
    }

    #[test]
    pub fn it_does_not_add_format_specifier_if_exists() {
        let mut config = get_exmaple_config();

        let c_conf = config.get_mut_lang_conf("c").unwrap();

        c_conf.set_raw_comment_string("/* %s */".to_string());

        assert_eq!(c_conf.get_raw_comment_string().unwrap(), "/* %s */");
        assert_eq!(c_conf.get_comment_string().unwrap(), "/* %s */");
    }
}
