use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=src/notification/macos/notify.swift");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("macos") {
        return;
    }
    let arch = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64",
        Ok("x86_64") => "x86_64",
        _ => panic!("unsupported macOS notification helper architecture"),
    };
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("tomatui-notify");
    let status = Command::new("xcrun")
        .args([
            "swiftc",
            "-O",
            "-target",
            &format!("{arch}-apple-macosx11.0"),
        ])
        .arg("src/notification/macos/notify.swift")
        .arg("-o")
        .arg(output)
        .status()
        .expect("building macOS notifications requires Xcode Command Line Tools (xcrun swiftc)");
    assert!(
        status.success(),
        "could not compile macOS notification helper"
    );
}
