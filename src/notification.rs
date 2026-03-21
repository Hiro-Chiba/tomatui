use std::process::Command;

pub fn bell() {
    print!("\x07");
}

/// Best-effort notification — failures are silently ignored so the timer is never blocked.
pub fn notify(title: &str, message: &str) {
    if cfg!(target_os = "macos") {
        let script = format!(
            "display notification \"{}\" with title \"{}\"",
            message.replace('"', "\\\""),
            title.replace('"', "\\\""),
        );
        let _ = Command::new("osascript").arg("-e").arg(&script).spawn();
    } else if cfg!(target_os = "linux") {
        let _ = Command::new("notify-send").arg(title).arg(message).spawn();
    }
}
