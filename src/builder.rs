use config::Config;
use std::fmt;
use std::error::Error;
use std::process::{Command, Output};

pub struct BuildResult {
    pub was_success: bool
}

pub fn build(config: Config) -> Result<BuildResult, String> {
    let working_dir = ".";

    let outputs = config.script.iter()
        .map(|script| execute_script(script, working_dir))
        .collect::<Vec<Output>>();

    Ok(BuildResult {
        was_success: outputs.iter().all(|o| o.status.clone().success())
    })
}

fn execute_script(script: &str, cwd: &str) -> Output {
    let command;
    let args;
    if script.contains(" ") {
        args = script.split(" ").skip(1).collect();
        command = script.split(" ").take(1).collect();
    }
    else {
        command = script.to_string();
        args = vec!();
    }

    Command::new(command)
        .args(&args[..])
        .current_dir(cwd)
        .output()
        .unwrap()
}
