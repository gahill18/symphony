#[cfg(test)]
mod tests;
mod architecture;
mod instructions;
mod script;

use instructions::Instructions;
use script::Script;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    instructions_url: Option<String>,
}

fn main() {
    let args: Args = Args::parse();
    let mut inst: Instructions = match args.instructions_url {
        Some(cp) => Instructions::from_url(cp),
        None => Instructions::from_url(String::from(
            "https://github.com/gahill18/symphony/raw/refs/heads/main/test/default.sfn",
        )),
    };

    while inst.run() {
        inst = inst.get_next_instructions();
    };
}
