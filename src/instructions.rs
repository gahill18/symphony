/*

JSON based format that defines functionality for the current program execution to follow

- Mode
    - Sleep(t: u64): Sleep for t seconds
    - ExecScript(c: str): Execute shell script with contents c
    - Transmit(c: str, d: str, p: Protocol): Transmit data with contents c to destination d using protocol p
    - Listen(p: u16, pt: Protocol): Listen on port p using protocol pt
- Repeats(n: u64): repeat the mode n times before fetching new instructions

*/

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::script::Script;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Mode {
    Sleep {t: u64},
    ExecScript {c: String},
    Transmit {c: String, d: String, p: Protocol},
    Listen {p: u16, pt: Protocol},
}

impl Mode {
    pub fn run(self) -> () {
        match self {
            Mode::Sleep {t} => std::thread::sleep(std::time::Duration::from_secs(t)),
            Mode::ExecScript {ref c} => {
                let mut script: Script = Script::from_str(&c);
                script.execute();
                println!("{:?}", script)
            },
            Mode::Transmit {c, d, p} => todo!("impl Transmit"),
            Mode::Listen {p, pt} => todo!("impl Listen"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Protocol {
    HTTP,
    HTTPS
}

type Repeats = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Instructions {
    mode: Mode,
    repeats: Repeats
}

impl Instructions {
    /* Build an Instructions object from a direct string input */
    fn from_string(string: String) -> Instructions {
        let deserialized_instructions: Instructions = match serde_json::from_str(&string) {
            Ok(di) => di,
            Err(e) => panic!(
                "failed to deserialize instructions from {}\nThrew error \"{}\"",
                string, e
            ),
        };
        return deserialized_instructions;
    }

    /* Build a Config object read in from a file */
    pub fn from_path_string(ps: String) -> Instructions {
        let inst_string: String = match fs::read_to_string(&PathBuf::from(&ps)) {
            Ok(is) => is,
            Err(e) => panic!("Failed to read string from {}\nThrew error \"{}\"", ps, e),
        };
        return Instructions::from_string(inst_string);
    }

    /* Build an Instructions object read in from a url */
    pub fn from_url(src_url: String) -> Instructions {
        let response = reqwest::blocking::get(&src_url).unwrap();
        let inst_string: String = if response.status() != 200 {
            panic!("Failed to get a proper response from url {}", src_url);
        } else {
            response.text().unwrap()
        };
        return Instructions::from_string(inst_string);
    }

    pub fn run(self) -> () {
        for i in 0..self.repeats {
            self.mode.clone().run()
        }

        return ();
    }
}