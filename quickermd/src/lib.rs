pub mod output;
mod runner;
pub mod user_config;
mod utils;

use crate::runner::QuickMDRunner;
use crate::user_config::{Config, LanguageConfig, Template};

/// # Quicker MD
///
/// This struct defines the entry point for most integrations
pub struct QuickerMD {
    /// The Users Config
    config: Config,
}

impl QuickerMD {
    /// Gets the config from the default config location
    ///
    /// ```no_run
    /// use quickermd::QuickerMD;
    /// # fn main() ->  Result<(), String> {
    /// 
    /// let config = QuickerMD::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            config: Config::get_config()?,
        })
    }

    /// Gets the config from a given file location
    ///
    /// ```rust
    /// use quickermd::QuickerMD;
    /// use std::path::PathBuf;
    /// # fn main() -> Result<(), String> {
    ///
    /// let config = QuickerMD::from_file_path(PathBuf::from("../examples/config.toml"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_file_path(file: std::path::PathBuf) -> Result<Self, String> {
        Ok(Self {
            config: Config::get_config_from_path(file)?,
        })
    }

    /// Gets the config for a language
    /// ```
    /// use quickermd::QuickerMD;
    /// use std::path::PathBuf;
    /// # fn main() -> Result<(), String> {
    ///
    /// let config = QuickerMD::from_file_path(PathBuf::from("../examples/config.toml"))?;
    /// let c_config = config.get_config_for_lang("c")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_config_for_lang(&self, lang: &str) -> Result<&LanguageConfig, String> {
        let config = &self.config;

        if let Some(lang_conf) = config.get_lang_conf(lang) {
            return Ok(lang_conf);
        }

        todo!("handle error")
    }

    /// Returns the template for a language
    ///
    /// **Prefer `QuickerMD::get_template`**
    pub fn get_template_for_lang(&self, lang: &str) -> Option<&Template> {
        if let Ok(conf) = self.get_config_for_lang(lang) {
            return conf.get_template();
        }
        None
    }

    /// Returns a resolved template for a language
    pub fn get_template(&self, lang: &str, input: Vec<String>) -> Option<Template> {
        if let Ok(lang_conf) = self.get_config_for_lang(lang) {
            return Some(Template::new(lang, input, lang_conf));
        }
        None
    }

    /// Runs a template
    /// ```
    /// use quickermd::QuickerMD;
    /// use std::path::PathBuf;
    /// # fn main() -> Result<(), String> {
    /// let mut config = QuickerMD::from_file_path(PathBuf::from("../examples/config.toml"))?;
    /// let output = config.run("c", r#"printf("Hello, world!\n");"#.to_string())?;
    /// 
    /// assert_eq!(output.get_exit_code(), 0);
    /// assert_eq!(output.get_stdout().replace("\r", ""), "Hello, world!\n");
    /// # Ok(())
    /// # }
    /// ```
    pub fn run(&mut self, lang: &str, input: String) -> Result<output::Output, String> {
        let config = self.get_config_for_lang(lang)?;
        let template = Template::new(lang, input.lines().map(|s| s.to_string()).collect(), config);

        let mut runner = QuickMDRunner::new(lang, &template, config, &self.config);

        runner
            .start()
            .map_err(|e| format!("There was an error running!:\n{}", e.to_string()))
    }
}

/// Formats a comment string
///
/// ```
/// use quickermd::format_comment_string;
///
/// let comment_string = "/* %s */"; // C, CSS comment string
///
/// let with = "QuickMD Output";
///
/// let output = format_comment_string(comment_string, with);
///
/// assert_eq!("/* QuickMD Output */".to_string(), output);
/// ```
pub fn format_comment_string<T: ToString>(comment_string: &str, with: T) -> String {
    comment_string.replace("%s", &with.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_config() -> QuickerMD {
        QuickerMD::from_file_path(std::path::PathBuf::from("../examples/config.toml")).unwrap()
    }

    #[test]
    fn it_works() {
        let mut quicker = get_example_config();
        quicker
            .run("js", "console.log('hello')".to_string())
            .unwrap();
    }

    #[test]
    fn it_retrieves_simple_output() {
        let mut quicker = get_example_config();

        let output = quicker
            .run("js", "console.log('hello')".to_string())
            .unwrap();

        let string = output.to_string();
        println!("{}", string);
        assert_eq!(
            output.to_string().trim(),
            r#"Output:
hello
"#
            .trim()
        );
    }

    #[test]
    fn it_runs_compiled_languages() {
        let mut quicker = get_example_config();

        let output = quicker
            .run("c", r#"printf("Hello, world!\n");"#.to_string())
            .unwrap();

        let string = output.to_string();
        println!("{}", string);

        assert_eq!(
            output.to_string().trim(),
            r#"Output:
Hello, world!
"#
            .trim()
        )
    }

    #[test]
    fn it_outputs_as_json() {
        let mut quicker = get_example_config();

        let mut output = quicker
            .run("py", "print('hello, from python!')".to_string())
            .unwrap();

        let pretty_raw_output = r#"
{
  "format": "JsonPretty",
  "stdout": "hello, from python!\n",
  "stderr": "",
  "code": 0
}"#
        .trim()
        .replace("\r\n", "\n")
        .replace("\t", "  ");

        output.output_as(output::OutputType::JsonPretty);

        let str_output = output.to_string();

        println!("{:?}", str_output);
        assert_eq!(
            pretty_raw_output,
            str_output.replace("\\r\\n", "\\n").trim()
        );

        let raw_input =
            r#"{"format":"JSON","stdout":"hello, from python!\n","stderr":"","code":0}"#;
        output.output_as(output::OutputType::JSON);

        let json_output = output.to_string();
        assert_eq!(raw_input, json_output.replace("\\r\\n", "\\n".trim()));
    }

    #[test]
    fn it_formats_a_comment_string() {
        let comment_string = "// %s";

        let new_string = format_comment_string(comment_string, "QuickMD Output");

        assert_eq!(new_string, "// QuickMD Output");
    }
}
