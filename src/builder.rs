use config::Config;
use std::process::{Command, Output};

pub struct BuildResult {
    pub script: String,
    pub success: bool,
    pub stdout: String,
    pub stderr: String
}

pub struct BuildResultStats {
    pub was_success: bool,
    pub results: Vec<BuildResult>
}

pub fn build(config: Config, working_dir: &str) -> Result<BuildResultStats, String> {

    let mut stats = Vec::new();

    for script in config.script {
        let result = execute_script(&script, working_dir);
        let build_result = BuildResult {
            script: script.clone(),
            success: result.status.clone().success(),
            stdout: String::from_utf8_lossy(&result.stdout).to_string(),
            stderr: String::from_utf8_lossy(&result.stderr).to_string()
        };

        stats.push(build_result);
    }

    Ok(BuildResultStats {
        was_success: stats.iter().all(|s| s.success),
        results: stats
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
