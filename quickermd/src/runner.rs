use std::io::Write;
use std::process::Stdio;
use std::{path::PathBuf, process::Command};
use variable_parser::VariableParser;

use crate::output::Output;
use crate::user_config::*;

pub struct QuickMDRunner<'runner> {
    template: &'runner Template,
    lang_conf: &'runner LanguageConfig,
    config: &'runner Config,
    output_file: PathBuf,
    lang: &'runner str,
}

impl<'runner> QuickMDRunner<'runner> {
    pub fn new(
        lang: &'runner str,
        template: &'runner Template,
        lang_conf: &'runner LanguageConfig,
        config: &'runner Config,
    ) -> Self {
        Self {
            lang,
            template,
            lang_conf,
            config,
            output_file: PathBuf::new(),
        }
    }
    pub fn start(&mut self) -> std::io::Result<Output> {
        if self.lang_conf.get_redir_input() {
            return self.redirect_input();
        }

        let tmp_dir = tempfile::tempdir()?;
        let tmp_path = tmp_dir.path().join(format!(
            "tmp.{}",
            self.lang_conf
                .get_extension()
                .unwrap_or(self.lang.to_string())
        ));

        let outfile = tmp_dir.path().join("out");
        self.template.write_to_file(tmp_path.clone());

        let variables = vec![
            ("{{IN}}", tmp_path.to_str().unwrap()),
            ("{{OUT}}", outfile.to_str().unwrap()),
            ("{{INPUT}}", tmp_path.to_str().unwrap()),
        ];

        let mut parser = VariableParser::new(variables);

        let cmd_name = self.lang_conf.get_command_name();
        let mut args = self.lang_conf.get_command_args();

        parser.parse_with_tracker(&mut args);

        let consumed_input = parser.had_used_var("{{INPUT}}");

        let output = Command::new(cmd_name).args(args).output()?;

        if !output.status.success() || consumed_input || self.lang_conf.explicit_no_run() {
            drop(outfile);
            _ = tmp_dir.close();

            return Ok(Output::from_u8(
                OutputType::Raw,
                &output.stdout,
                &output.stderr,
                output.status,
            ));
        }

        let ret = self.run_explicit(&outfile, &parser);

        drop(outfile);
        _ = tmp_dir.close();

        ret
    }

    fn run_explicit(
        &self,
        file: &PathBuf,
        variables: &'runner VariableParser<&str>,
    ) -> std::io::Result<Output> {
        let output_file = format!("{}", file.to_str().unwrap());
        let output;

        if let Some((exe_command, mut args)) = self.lang_conf.get_run_command(output_file.clone()) {
            variables.parse_string_vec(&mut args);
            output = Command::new(exe_command).args(args).output()?;
        } else {
            output = Command::new(output_file).output()?;
        }

        Ok(Output::from_u8(OutputType::Raw, &output.stdout, &output.stderr, output.status))
    }

    fn redirect_input(&self) -> std::io::Result<Output> {
        let cmd_name = self.lang_conf.get_command_name();
        let args = self.lang_conf.get_command_args();

        let mut child = Command::new(cmd_name)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut stdin = child.stdin.take().expect("Failed to open stdin");

        let input = self.template.get_input();

        std::thread::spawn(move || {
            stdin
                .write_all(input.join("\n").as_bytes())
                .expect("Failed to write stdin");
        });

        let output = child.wait_with_output().expect("Failed to create output");

        Ok(Output::from_u8(
            OutputType::Raw,
            &output.stdout,
            &output.stderr,
            output.status,
        ))
    }
}
