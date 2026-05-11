use crate::architecture;
use crate::script::Script;
use crate::instructions::Instructions;
use std::process::Output;

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

#[test]
fn architecture_get_sys_architecture() {
    let arch = architecture::get_sys_architecture();
    // Confirm no panic
    assert!(true)
}

#[test]
fn instructions_from_path_string() {
    let ps: String = String::from("./test/default.sfn");
    let inst: Instructions = Instructions::from_path_string(ps);
    // Confirm no panic
    assert!(true)
}

#[test]
fn instructions_from_url() {
    let url: String = String::from("https://github.com/gahill18/symphony/raw/refs/heads/refactor-instruction-formatting-and-parsing/test/default.sfn");
    let inst: Instructions = Instructions::from_url(url);
    // Confirm no panic
    assert!(true);
}

#[test]
fn instructions_run() {
    let ps: String = String::from("./test/default.sfn");
    let inst: Instructions = Instructions::from_path_string(ps);
    // Warning - by default this will infinitely loop! Need to Ctrl-C
    inst.run();
    // Confirm no panic
    assert!(true);
}
