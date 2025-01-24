use crate::config::LanguageConfig;
use crate::utils::*;
use crate::variables::VariableParser;
use crate::Template;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tempfile::tempdir;

#[derive(Debug)]
pub struct QuickMDOutput {
    pub output_file: PathBuf,
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
}

pub fn show_output(str: &Vec<String>, prefix: &str) {
    for line in str.iter() {
        if !line.is_empty() {
            println!("{}{}", prefix, line);
        }
    }
    println!("");
}

impl<'lang> QuickMDOutput {
    pub fn start(template: &'lang Template) -> std::io::Result<Self> {
        let conf = template.get_conf();
        if conf.should_redir() {
            return QuickMDOutput::redir_input(template, conf);
        }

        let tmp_dir = tempdir()?;
        let tmp_path = tmp_dir
            .path()
            .join(format!("tmp.{}", template.get_file_ext()));

        let out_file = tmp_dir.path().join("out");
        template.to_file_path(tmp_path.clone())?;

        let input_str = template.input_to_str();

        let variables = vec![
            ("{{IN}}", tmp_path.to_str().unwrap()),
            ("{{OUT}}", out_file.to_str().unwrap()),
            ("{{INPUT}}", &input_str),
        ];

        let mut parser = VariableParser::new(variables);

        let cmd_name = conf.get_command_name();
        let mut args: Vec<String> = conf.get_command_args();

        parser.parse_with_tracker(&mut args);

        let consumed_input = parser.had_used_var("{{INPUT}}");


        let output = Command::new(cmd_name).args(args).output()?;

        let stdout = u8_to_str_vec(output.stdout);
        let stderr = u8_to_str_vec(output.stderr);

        if !output.status.success() || consumed_input || template.get_conf().explicit_no_run() {
            return Ok(QuickMDOutput {
                output_file: PathBuf::from(out_file),
                stdout,
                stderr,
            });
        }
        let ret = QuickMDOutput::run(PathBuf::from(out_file.clone()), template, &parser);

        drop(out_file);
        _ = tmp_dir.close(); // Supress error

        ret
    }

    pub fn redir_input(
        template: &'lang Template,
        conf: &'lang LanguageConfig,
    ) -> std::io::Result<Self> {
        let cmd_name = conf.get_command_name();
        let args = conf.get_command_args();
        let input = template.input_to_str();

        let mut child = Command::new(cmd_name)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Could not spawn child proccess");

        let mut stdin = child.stdin.take().expect("Failed to open stdin");

        std::thread::spawn(move || {
            stdin
                .write_all(input.as_bytes())
                .expect("Failed to write stdin");
        });
        let output = child.wait_with_output().expect("failed to read stdout");

        let stdout = u8_to_str_vec(output.stdout);
        let stderr = u8_to_str_vec(output.stderr);

        Ok(QuickMDOutput {
            output_file: PathBuf::from("stdin"),
            stdout,
            stderr,
        })
    }

    pub fn run(
        file: PathBuf,
        template: &'lang Template,
        variables: &'lang VariableParser<&str, &str>,
    ) -> std::io::Result<Self> {
        let output_file = format!("{}", file.to_str().unwrap());
        let output;

        if let Some((exe_command, mut args)) = template.get_run_command(output_file.clone()) {
            variables.parse_string_vec(&mut args);
            output = Command::new(exe_command).args(args).output()?;
        } else {
            output = Command::new(output_file).output()?;
        }

        let stdout = u8_to_str_vec(output.stdout);
        let stderr = u8_to_str_vec(output.stderr);

        Ok(QuickMDOutput {
            output_file: file,
            stdout,
            stderr,
        })
    }

    pub fn output(&self, input: Option<Vec<String>>, raw: bool, prefix: Option<String>) {
        let pre = prefix.unwrap_or("".to_string());

        if let Some(lines) = input {
            if !raw {
                println!("{}Input:", pre);
            }
            show_output(&lines, "");
        }

        if str_vec_non_empty(&self.stdout) {
            if !raw {
                println!("{}Output:", pre);
            }
            show_output(&self.stdout, &pre);
        }

        if str_vec_non_empty(&self.stderr) {
            if !raw {
                println!("{}Error:", pre)
            }
            show_output(&self.stderr, &pre);
        }
    }
}
