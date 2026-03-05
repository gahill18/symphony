use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

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

#[derive(Debug)]
struct Script {
    cmds: Vec<String>,
    last_outputs: Vec<Output>,
}

impl Script {
    /* Build a Script object from a list of strings */
    fn from_lines(lines: Vec<&str>) -> Script {
        let cmds: Vec<String> = lines.iter().map(|&l| String::from(l)).collect();
        let last_outputs: Vec<Output> = Vec::new();
        return Script { cmds, last_outputs };
    }

    /* Use HTTP Get to build a Script object from a source URL argument */
    fn from_source_url(src_url: String) -> Script {
        let response = reqwest::blocking::get(src_url).unwrap();
        let text: String = if response.status() != 200 {
            panic!("Failed to get a proper response");
        } else {
            response.text().unwrap()
        };
        let lines: Vec<&str> = text.lines().collect();
        Script::from_lines(lines)
    }

    fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut outputs: Vec<Output> = Vec::new();
        for c in self.cmds.iter() {
            // TODO: Support non-sh scripts
            // TODO: Async Child Process Spawn
            let output: Output = Command::new("sh")
                .arg("-c")
                .arg(c)
                .output()
                .expect("failed");
            outputs.push(output);
        }
        self.last_outputs = outputs;
        return Ok(());
    }
}

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
        loop_continue = script.execute().is_ok();
        println!("{:?}", script);
    }
}

mod tests {
    #[test]
    fn sanity_check() {
        assert!(true);
    }
}
