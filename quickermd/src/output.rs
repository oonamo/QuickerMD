use std::process::ExitStatus;

use crate::utils::u8_to_str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
// TODO: with_input, no_prefix
pub struct Output {
    format: OutputType,
    prefix: String,
    stdout: String,
    stderr: String,
    code: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum OutputType {
    JSON,
    JsonPretty,
    Raw,
}

impl std::fmt::Display for OutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OutputType::JSON => "json",
            OutputType::JsonPretty => "json-pretty",
            OutputType::Raw => "raw",
        };

        write!(f, "{}", str)
    }
}

impl From<String> for OutputType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "json" => OutputType::JSON,
            _ => OutputType::Raw,
        }
    }
}

impl Output {
    pub fn new(
        format: OutputType,
        stdout: Option<String>,
        stderr: Option<String>,
        code: i32,
    ) -> Self {
        Output {
            format,
            stdout: stdout.unwrap_or("".to_string()),
            stderr: stderr.unwrap_or("".to_string()),
            code,
            prefix: String::new(),
        }
    }
    pub fn from_u8(
        format: OutputType,
        stdout: &Vec<u8>,
        stderr: &Vec<u8>,
        code: ExitStatus,
    ) -> Self {
        Output {
            format,
            code: code.code().unwrap_or(0),
            stdout: u8_to_str(stdout),
            stderr: u8_to_str(stderr),
            prefix: String::new(),
        }
    }
    pub fn has_stdout(&self) -> bool {
        !self.stdout.is_empty()
    }
    pub fn has_stderr(&self) -> bool {
        !self.stderr.is_empty()
    }
    pub fn raw_to_string(&self) -> String {
        let mut output = String::new();

        let stdout = self.stdout.clone();
        let stderr = self.stderr.clone();

        if self.has_stdout() {
            output.push_str(&format!("{}Output:\n{}", self.prefix, stdout));
        }

        if self.has_stderr() {
            output.push_str(&format!("{}Error:\n{}", self.prefix, stderr));
        }

        output
    }

    pub fn output_as(&mut self, output_type: OutputType) {
        self.format = output_type;
    }
}

impl ToString for Output {
    fn to_string(&self) -> String {
        match self.format {
            OutputType::JSON => serde_json::to_string(self).unwrap(),
            OutputType::JsonPretty => serde_json::to_string_pretty(self).unwrap(),
            OutputType::Raw => self.raw_to_string(),
        }
    }
}
