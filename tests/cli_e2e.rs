use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tomatui"))
        .args(args)
        .output()
        .expect("run the built CLI")
}

#[test]
fn help_and_version_exit_without_a_terminal() {
    let output = run(&["--help"]);
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    for example in ["tomatui 30m", "tomatui -m", "[WORK]", "[BREAK]"] {
        assert!(text.contains(example), "missing {example}: {text}");
    }
    let output = run(&["--version"]);
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains(env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn invalid_input_fails_before_entering_terminal_mode() {
    for args in [
        vec!["0m"],
        vec!["30m", "--work", "20"],
        vec!["30m", "10m", "--break", "5"],
        vec!["--on-end", "unknown"],
        vec!["18446744073709551615h"],
        vec!["stats", "history", "--days", "3661"],
    ] {
        let output = run(&args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(output.stdout.is_empty(), "{args:?}");
        assert!(String::from_utf8(output.stderr).unwrap().contains("error:"));
    }
}
