use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
    cmds: Vec<Command>,
}

impl Script {
    fn from_source_url(src_url: String) -> Script {
        let response = reqwest::blocking::get(src_url).unwrap();
        let lines: Vec<&str> = if response.status() != 200 {
            String::new()
        } else {
            response.text().unwrap()
        }
        .lines()
        .collect();

        let cmds: Vec<Command> = Vec::new();
        if response.status() == 200 {
            // TODO: Async Child Process Spawn
            let output = Command::new("sh")
                .arg("-c")
                .arg(response.text().unwrap())
                .output()
                .expect("failed");
            println!("{:?}", output);
        }
        todo!("Return vector of commands")
    }
}

fn main() {
    let args: Args = Args::parse();
    let conf_path_string: String = args.config_path.unwrap();
    let deserialized_config: Config = Config::from_path_string(conf_path_string);
    let script = Script::from_source_url(deserialized_config.source_url);
}

mod tests {
    #[test]
    fn sanity_check() {
        assert!(true);
    }
}
