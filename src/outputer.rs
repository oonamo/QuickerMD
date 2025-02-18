use crate::cli::OutputFormat;
use directories::ProjectDirs;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::{collections::HashMap, io::Write, path::PathBuf};
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OutputArgs<'output> {
    order: Vec<String>,

    show_input: bool,

    #[serde(default = "default_input")]
    input: Section,

    #[serde(default = "default_output")]
    output: Section,

    #[serde(default = "default_error")]
    error: Section,

    #[serde(skip)]
    raw_input: &'output str,

    #[serde(skip)]
    style: OutputFormat,
}

pub fn default_input() -> Section {
    Section {
        name: "Input".to_string(),
        icon: "#".to_string(),
        color: "blue".to_string(),
        bold: true,
        italic: false,
        strikethrough: false,
        underline: false,
        value: String::default(),
    }
}

pub fn default_output() -> Section {
    Section {
        name: "Output".to_string(),
        icon: "#".to_string(),
        color: "green".to_string(),
        bold: true,
        italic: false,
        strikethrough: false,
        underline: false,
        value: String::default(),
    }
}

pub fn default_error() -> Section {
    Section {
        name: "Error".to_string(),
        icon: "#".to_string(),
        color: "red".to_string(),
        bold: true,
        italic: false,
        strikethrough: false,
        underline: false,
        value: String::default(),
    }
}

impl<'output> Default for OutputArgs<'output> {
    fn default() -> Self {
        Self {
            order: vec![
                "input".to_string(),
                "output".to_string(),
                "error".to_string(),
            ],
            show_input: false,
            input: default_input(),
            output: default_output(),
            error: default_error(),
            raw_input: "",
            style: OutputFormat::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Section {
    name: String,

    #[serde(default = "default_icon")]
    icon: String,

    #[serde(default)]
    color: String,

    #[serde(default)]
    bold: bool,

    #[serde(default)]
    italic: bool,

    #[serde(default)]
    strikethrough: bool,

    #[serde(default)]
    underline: bool,

    #[serde(skip)]
    value: String,
}

impl Section {
    pub fn resolve_value(&mut self, value: String) {
        self.value = value;
    }
}

pub fn default_icon() -> String {
    "#".to_string()
}

impl<'output> OutputArgs<'output> {
    pub fn write_as_comment(&self, comment_string: &str) {
        for item in self.order.iter() {
            let section = match item.as_str() {
                "input" => &self.input,
                "output" => &self.output,
                "error" => &self.error,
                _ => unreachable!("Should of been checked when resolving config"),
            };

            if section.name == "input" {
                println!("{}", section.value);
            } else {
                println!("{} {}", comment_string, section.name);
            }

            for line in section.value.lines() {
                println!("{} {}", comment_string, line);
            }
        }
    }
    pub fn write_pretty_to_console(&self) -> std::io::Result<()> {
        let bufwtr = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();

        for item in self.order.iter() {
            let section = match item.as_str() {
                "input" => &self.input,
                "output" => &self.output,
                "error" => &self.error,
                _ => unreachable!("Should of been checked when resolving config"),
            };

            self.write_section(&mut buffer, section)?;
        }

        bufwtr.print(&buffer)?;
        Ok(())
    }

    pub fn get_input(&mut self) -> &mut Section {
        &mut self.input
    }

    fn write_section(&self, mut buffer: &mut Buffer, args: &Section) -> std::io::Result<()> {
        if !args.value.is_empty() {
            let color = Color::from_str(&args.color).unwrap_or_else(|_| Color::Blue);
            let mut color_spec = ColorSpec::new();

            if args.bold {
                color_spec.set_bold(true);
            }

            if args.italic {
                color_spec.set_italic(true);
            }

            if args.underline {
                color_spec.set_underline(true);
            }

            if args.strikethrough {
                color_spec.strikethrough();
            }

            color_spec.set_fg(Some(color));

            buffer.set_color(&color_spec)?;

            writeln!(&mut buffer, "{} {}", args.icon, args.name)?;
            buffer.reset()?;

            writeln!(&mut buffer, "{}", args.value.clone())?;
        }

        Ok(())
    }

    pub fn get_config_from_path(
        path: PathBuf,
        input: String,
        output: String,
        error: String,
    ) -> Result<Self, String> {
        let read_result = std::fs::read_to_string(path.clone());
        let config_contents;

        if let Err(e) = read_result {
            if ErrorKind::NotFound == e.kind() {
                let mut output_arg = OutputArgs::default();
                output_arg.set_reserved_section_values(input, output, error);
                return Ok(output_arg);
            } else {
                return Err(format!(
                    "Error reading file `{}`:\n{}",
                    path.to_str().unwrap_or_default(),
                    e.to_string()
                ));
            }
        } else {
            config_contents = read_result.unwrap();
        }

        let mut config = toml::from_str::<OutputArgs>(&config_contents).map_err(|e| {
            format!(
                "There was an error reading the config '{}':\n{}",
                path.to_str().unwrap_or_default(),
                e.to_string()
            )
        })?;

        config.set_reserved_section_values(input, output, error);

        Ok(config)
    }

    pub fn get_config(input: String, output: String, error: String) -> Result<Self, String> {
        let path = ProjectDirs::from("", "", "QuickMD")
            .ok_or("Could not resolve project directory")?
            .config_local_dir()
            .join("output.toml");

        OutputArgs::get_config_from_path(path, input, output, error)
    }

    pub fn set_reserved_section_values(&mut self, input: String, output: String, error: String) {
        self.input.resolve_value(input);
        self.output.resolve_value(output);
        self.error.resolve_value(error);
    }
}
