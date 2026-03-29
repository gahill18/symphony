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

    /* Execute all commands stored in the cmds field, saving outputs to the last_outputs field */
    pub fn execute_async(&mut self) -> () {
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
