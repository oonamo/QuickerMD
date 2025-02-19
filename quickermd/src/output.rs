use std::process::ExitStatus;

use crate::utils::u8_to_str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Output {
    format: OutputType,
    stdout: String,
    stderr: String,
    code: i32,
}

#[derive(Deserialize, Serialize, Clone)]
/// Describes how the `to_string` function will behave
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
    /// Creates a new instance of `Output`
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
        }
    }
    /// Creates a new instance of `Output` using a vector of bytes
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
        }
    }
    /// Checks if `stdout` is non-empty
    pub fn has_stdout(&self) -> bool {
        !self.stdout.is_empty()
    }
    /// Checks if `stderr` is non-empty
    pub fn has_stderr(&self) -> bool {
        !self.stderr.is_empty()
    }
    /// The *raw* implementation for `to_string`
    /// Returns a raw string in the format of:
    /// ```text
    /// Output:
    /// Hello, World
    /// Error:
    /// Hello, Error
    /// ```
    pub fn raw_to_string(&self) -> String {
        let mut output = String::new();

        let stdout = self.stdout.clone();
        let stderr = self.stderr.clone();

        if self.has_stdout() {
            output.push_str(&format!("Output:\n{}", stdout));
        }

        if self.has_stderr() {
            output.push_str(&format!("Error:\n{}", stderr));
        }

        output
    }

    /// Sets the output `format`
    pub fn output_as(&mut self, output_type: OutputType) {
        self.format = output_type;
    }

    /// Returns the `stdout`
    pub fn get_stdout(&self) -> String {
        self.stdout.clone()
    }

    /// Returns the `stderr`
    pub fn get_stderr(&self) -> String {
        self.stderr.clone()
    }

    /// Returns the exit `code`
    pub fn get_exit_code(&self) -> i32 {
        self.code
    }

    /// Returns the current `format`
    pub fn get_format(&self) -> OutputType {
        self.format.clone()
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
