use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

mod architecture;

mod script;
use script::Script;

#[cfg(test)]
mod tests;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    source_url: String,
    time_to_wait: Option<u64>,
}

impl Config {
    /* Build a Config object from a direct string input */
    fn from_string(string: String) -> Config {
        let deserialized_config: Config = match serde_json::from_str(&string) {
            Ok(dc) => dc,
            Err(e) => panic!(
                "failed to deserialize config from {}\nThrew error \"{}\"",
                string, e
            ),
        };
        return deserialized_config;
    }

    /* Build a Config object read in from a file */
    fn from_path_string(ps: String) -> Config {
        let conf_string: String = match fs::read_to_string(&PathBuf::from(&ps)) {
            Ok(cs) => cs,
            Err(e) => panic!("Failed to read string from {}\nThrew error \"{}\"", ps, e),
        };
        return Config::from_string(conf_string);
    }

    /* Build a Config object read in from a url */
    fn from_url(src_url: String) -> Config {
        let response = reqwest::blocking::get(&src_url).unwrap();
        let conf_string: String = if response.status() != 200 {
            panic!("Failed to get a proper response from url {}", src_url);
        } else {
            response.text().unwrap()
        };
        return Config::from_string(conf_string);
    }
}

fn main() {
    let args: Args = Args::parse();
    let config: Config = match args.config_path {
        Some(cp) => Config::from_path_string(cp),
        None => Config::from_url(String::from(
            "https://github.com/gahill18/symphony/raw/refs/heads/main/test/default_config.json",
        )),
    };

    let mut loop_continue: bool = true;
    while loop_continue {
        let mut script: Script = Script::from_source_url(String::from(&config.source_url));
        script.execute();
        loop_continue = script.was_success();
        dbg!("{:?}", script);

        if let Some(ttw) = config.time_to_wait {
            std::thread::sleep(std::time::Duration::from_secs(ttw));
        }
    }
}
