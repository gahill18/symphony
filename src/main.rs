use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

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

fn read_config_file(file_path: &Path) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn main() {
    let args: Args = Args::parse();
    let conf_path: String = args.config_path.unwrap();
    let conf_contents: String = read_config_file(&PathBuf::from(conf_path)).unwrap();
    let deserialized_config: Config = serde_json::from_str(&conf_contents).unwrap();
    println!("{:?}", deserialized_config);
}

mod tests {
    #[test]
    fn sanity_check() {
        assert!(true);
    }
}
