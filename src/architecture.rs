use std::process::{Command, Output};

pub enum Architecture {
    X86_64,
    ARM,
    UNDEF,
}

/* Use uname -m to get the current system architecture */
pub fn get_sys_architecture() -> Architecture {
    let output: Output = Command::new("sh")
        .arg("-c")
        .arg("uname -m")
        .output()
        .expect("Failed to obtain system architecture");
    if let Ok(stdout_text) = String::from_utf8(output.stdout) {
        dbg!("uname -m output: {}", &stdout_text);
        return match stdout_text.as_str() {
            "x86_64\n" => Architecture::X86_64,
            _ => Architecture::UNDEF,
        };
    } else {
        todo!("implement get_system_architecture()");
    }
}
