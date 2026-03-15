use std::fs;
use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    source_url: String,
    time_to_wait: u64,
}

impl Config {
    fn from_path_string(ps: String) -> Config {
        let conf_contents: String = fs::read_to_string(&PathBuf::from(ps)).unwrap();
        let deserialized_config: Config = serde_json::from_str(&conf_contents).unwrap();
        return deserialized_config;
    }
}

mod script {
    use std::process::{Command, Output};
    #[derive(Debug)]
    pub struct Script {
        cmds: Vec<String>,
        last_outputs: Vec<Output>,
    }

    impl Script {
        /* Build a Script object from a list of strings */
        pub fn from_lines(lines: Vec<&str>) -> Script {
            let cmds: Vec<String> = lines.iter().map(|&l| String::from(l)).collect();
            let last_outputs: Vec<Output> = Vec::new();
            return Script { cmds, last_outputs };
        }

        /* Use HTTP Get to build a Script object from a source URL argument */
        pub fn from_source_url(src_url: String) -> Script {
            let response = reqwest::blocking::get(src_url).unwrap();
            let text: String = if response.status() != 200 {
                panic!("Failed to get a proper response");
            } else {
                response.text().unwrap()
            };
            let lines: Vec<&str> = text.lines().collect();
            Script::from_lines(lines)
        }

        /* Return a copy of the commands vector */
        pub fn cmds(&self) -> Vec<String> {
            return self.cmds.clone();
        }

        /* Return a copy of the commands vector */
        pub fn last_outputs(&self) -> Vec<Output> {
            return self.last_outputs.clone();
        }

        /* Execute all commands stored in the cmds field, saving outputs to the last_outputs field */
        pub fn execute(&mut self) -> () {
            let mut outputs: Vec<Output> = Vec::new();
            for c in self.cmds.iter() {
                // TODO: Support non-sh scripts
                // TODO: Async Child Process Spawn
                let output: Output = Command::new("sh")
                    .arg("-c")
                    .arg(c)
                    .output()
                    .expect(&format!("command {:?} failed", c));
                outputs.push(output);
            }
            self.last_outputs = outputs;
            return ();
        }

        /* Return true if all commands ran succesfully on last execute() call, otherwise return false. */
        pub fn was_success(&self) -> bool {
            return self
                .last_outputs
                .iter()
                .map(|o| o.status)
                .all(|o| o.success());
        }
    }
}

use script::Script;
fn main() {
    let args: Args = Args::parse();
    let conf_path_string: String = args.config_path.unwrap();
    let mut loop_continue: bool = true;

    while loop_continue {
        let deserialized_config: Config = Config::from_path_string(String::from(&conf_path_string));
        let mut script: Script =
            Script::from_source_url(String::from(&deserialized_config.source_url));

        std::thread::sleep(std::time::Duration::from_secs(
            deserialized_config.time_to_wait,
        ));
        script.execute();
        loop_continue = script.was_success();

        dbg!("{:?}", script);
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;
    use crate::script::Script;
    use std::process::Output;

    #[test]
    fn sanity_check() {
        assert!(true);
    }

    #[test]
    fn config_from_path_str() {
        let path_string: String = String::from("./test/basic_config.json");
        let config: Config = Config::from_path_string(path_string);
        assert_eq!(config.time_to_wait, 5);
        assert_eq!(
            config.source_url,
            "https://github.com/gahill18/symphony/raw/refs/heads/main/test/basic_instructions.sh"
        );
    }

    #[test]
    fn script_from_lines() {
        let lines: Vec<&str> = vec!["whoami", "ls", "ps"];
        let cmds: Vec<String> = lines.iter().map(|&x| String::from(x)).collect();
        let script: Script = Script::from_lines(lines);
        let last_outputs: Vec<Output> = vec![];

        assert_eq!(cmds, script.cmds());
        assert_eq!(last_outputs, script.last_outputs());
    }

    #[test]
    fn script_from_source_url() {
        let src_url: String = String::from(
            "https://github.com/gahill18/symphony/raw/refs/heads/main/test/basic_instructions.sh",
        );
        let script: Script = Script::from_source_url(src_url);
        let cmds: Vec<String> = vec!["echo \"test instructions\"", "echo \"second line\""]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let last_outputs: Vec<Output> = vec![];

        assert_eq!(cmds, script.cmds());
        assert_eq!(last_outputs, script.last_outputs());
    }

    #[test]
    fn script_execute() {
        let src_url: String = String::from(
            "https://github.com/gahill18/symphony/raw/refs/heads/main/test/basic_instructions.sh",
        );
        let mut script: Script = Script::from_source_url(src_url);
        script.execute();

        let correct_last_stdouts: Vec<String> = vec!["test instructions\n", "second line\n"]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let actual_last_outputs: Vec<Output> = script.last_outputs();
        let actual_last_stdouts: Vec<_> = actual_last_outputs
            .iter()
            .map(|x| String::from_utf8_lossy(&x.stdout))
            .collect();
        assert_eq!(correct_last_stdouts, actual_last_stdouts);
    }

    #[test]
    fn script_was_success() {
        let successful_src_url: String = String::from(
            "https://github.com/gahill18/symphony/raw/refs/heads/main/test/basic_instructions.sh",
        );
        let failed_src_url: String = String::from(
            "https://github.com/gahill18/symphony/raw/refs/heads/main/test/fail_instructions.sh",
        );

        let mut successful_script: Script = Script::from_source_url(successful_src_url);
        successful_script.execute();

        let mut failed_script: Script = Script::from_source_url(failed_src_url);
        failed_script.execute();

        assert!(successful_script.was_success());
        assert!(!failed_script.was_success());
    }
}
