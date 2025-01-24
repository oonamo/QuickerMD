use crate::config::LanguageConfig;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Template<'lang> {
    conf: &'lang LanguageConfig,
    input: Vec<String>,
    file_ext: String,
}

impl<'lang> Template<'lang> {
    pub fn new(lang: &'lang str, conf: &'lang LanguageConfig, input: Vec<String>) -> Self {
        Template {
            conf,
            input,
            file_ext: lang.to_string(),
        }
    }

    pub fn to_file_path(&self, path: PathBuf) -> std::io::Result<()> {
        let mut file = File::create(path.clone())?;
        file.write_all(
            self.conf
                .to_string_from_input(self.input.clone())
                .as_bytes(),
        )?;

        Ok(())
    }

    pub fn get_conf(&self) -> &'lang LanguageConfig {
        self.conf
    }

    pub fn get_file_ext(&self) -> String {
        if let Some(ext) = self.conf.get_extension() {
            ext
        } else {
            self.file_ext.clone()
        }
    }

    pub fn input_to_str(&self) -> String {
        let mut str: String = String::new();
        for line in self.input.iter() {
            str.push_str(&line);
            str.push('\n');
        }
        str
    }

    pub fn get_run_command(&self) -> Option<String> {
        self.conf.get_run_command()
    }
}
