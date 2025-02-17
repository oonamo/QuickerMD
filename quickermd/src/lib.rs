mod output;
mod runner;
mod user_config;
mod utils;

use crate::runner::QuickMDRunner;
use crate::user_config::{Config, LanguageConfig, Template};

pub struct QuickerMD {
    config: Config,
}

impl QuickerMD {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            config: Config::get_config()?,
        })
    }

    pub fn get_config_for_lang(&self, lang: &str) -> Result<&LanguageConfig, String> {
        let config = &self.config;

        if let Some(lang_conf) = config.get_lang_conf(lang) {
            return Ok(lang_conf);
        }

        todo!("handle error")
    }

    fn get_template_for_lang(&self, lang: &str) -> Option<&Template> {
        if let Ok(conf) = self.get_config_for_lang(lang) {
            return conf.get_template();
        }
        None
    }

    pub fn run(&mut self, lang: &str, input: String) -> Result<output::Output, String> {
        let config = self.get_config_for_lang(lang)?;
        //let template = config.get_template();
        let template = Template::new(lang, input.lines().map(|s| s.to_string()).collect(), config);

        let mut runner = QuickMDRunner::new(lang, &template, config, &self.config);

        runner
            .start()
            .map_err(|e| format!("There was an error runnin!:\n{}", e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("Get ready...");
        let quicker_r = QuickerMD::new();
        if let Err(e) = quicker_r {
            println!("Error: {}", e);
            panic!("{}", e);
        }
        //let mut quicker = QuickerMD::new();
        quicker_r
            .unwrap()
            .run("js", "console.log('hello')".to_string())
            .unwrap();
    }

    #[test]
    fn can_retrieve_simple_output() {
        let mut quicker = QuickerMD::new().unwrap();

        let output = quicker
            .run("js", "console.log('hello')".to_string())
            .unwrap();

        let string = output.to_string();
        println!("{}", string);
        assert_eq!(
            output.to_string(),
            r#"Output:
hello
"#
        );
    }
}
