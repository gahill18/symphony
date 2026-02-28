use std::fs;
use std::path::PathBuf;

use clap::Parser;
use reqwest::get;
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

fn main() {
    let args: Args = Args::parse();
    let conf_path_string: String = args.config_path.unwrap();
    let deserialized_config: Config = Config::from_path_string(conf_path_string);
    println!("{:?}", deserialized_config);

    let response = reqwest::blocking::get(deserialized_config.source_url).unwrap();
    if response.status() == 200 {
        println!("{:?}", response.text().unwrap());
    }
}

mod tests {
    #[test]
    fn sanity_check() {
        assert!(true);
    }
}
