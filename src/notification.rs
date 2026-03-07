use std::process::Command;

pub fn bell() {
    print!("\x07");
}

pub fn notify(title: &str, message: &str) {
    if cfg!(target_os = "macos") {
        let script = format!(
            "display notification \"{}\" with title \"{}\"",
            message.replace('"', "\\\""),
            title.replace('"', "\\\""),
        );
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .spawn();
    }
}
