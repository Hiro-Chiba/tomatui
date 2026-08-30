use std::io::{self, Write};

#[cfg(all(not(test), any(target_os = "macos", target_os = "linux")))]
use std::process::Command;

pub fn bell() {
    print!("\x07");
    let _ = io::stdout().flush();
}

#[cfg(all(not(test), any(target_os = "macos", target_os = "linux")))]
fn spawn_notification(mut command: Command) {
    let _ = std::thread::Builder::new()
        .name("tomatui-notification".to_string())
        .spawn(move || {
            let _ = command.status();
        });
}

/// Sends a best-effort notification without blocking the timer.
#[cfg(all(not(test), target_os = "macos"))]
pub fn notify(title: &str, message: &str) {
    let script = format!(
        "display notification \"{}\" with title \"{}\"",
        message.replace('"', "\\\""),
        title.replace('"', "\\\""),
    );
    let mut command = Command::new("osascript");
    command.arg("-e").arg(script);
    spawn_notification(command);
}

/// Sends a best-effort notification without blocking the timer.
#[cfg(all(not(test), target_os = "linux"))]
pub fn notify(title: &str, message: &str) {
    let mut command = Command::new("notify-send");
    command.arg(title).arg(message);
    spawn_notification(command);
}

/// Desktop notifications are disabled in tests and unsupported platforms.
#[cfg(any(test, not(any(target_os = "macos", target_os = "linux"))))]
pub fn notify(_title: &str, _message: &str) {}
