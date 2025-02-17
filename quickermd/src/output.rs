use std::process::ExitStatus;

use crate::user_config::OutputType;
use crate::utils::u8_to_str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
// TODO: with_input, no_prefix
pub struct Output {
    format: OutputType,
    prefix: String,
    stdout: Option<String>,
    stderr: Option<String>,
    code: i32,
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
            stdout,
            stderr,
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
            stdout: Some(u8_to_str(stdout)),
            stderr: Some(u8_to_str(stderr)),
            prefix: String::new(),
        }
    }

    pub fn raw_to_string(&self) -> String {
        let mut output = String::new();

        let stdout = self.stdout.clone();
        let stderr = self.stderr.clone();
        if stdout.clone().is_some_and(|s| !s.is_empty()) {
            output.push_str(&format!("{}Output:\n{}", self.prefix, stdout.unwrap()));
        }
        if stderr.clone().is_some_and(|s| !s.is_empty()) {
            output.push_str(&format!("{}Error:\n{}", self.prefix, stderr.unwrap()));
        }

        output
    }
}

impl ToString for Output {
    fn to_string(&self) -> String {
        match self.format {
            OutputType::JSON(_) => serde_json::to_string(self).unwrap(),
            OutputType::Raw => self.raw_to_string(),
        }
        //serde_json::to_string(self).unwrap_or(serde_json::json!(r#"{ "status": "invalid" }"#).to_string())
    }
}
