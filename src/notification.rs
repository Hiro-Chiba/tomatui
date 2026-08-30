use std::io::{self, Write};

#[cfg(all(not(test), any(target_os = "macos", target_os = "linux")))]
use std::process::Command;

const TERMINAL_BELL: char = '\x07';
#[cfg(all(not(test), any(target_os = "macos", target_os = "linux")))]
const NOTIFICATION_THREAD_NAME: &str = "tomatui-notification";
#[cfg(all(not(test), target_os = "macos"))]
const MACOS_NOTIFICATION_COMMAND: &str = "osascript";
#[cfg(all(not(test), target_os = "linux"))]
const LINUX_NOTIFICATION_COMMAND: &str = "notify-send";

pub fn bell() {
    print!("{TERMINAL_BELL}");
    let _ = io::stdout().flush();
}

#[cfg(all(not(test), any(target_os = "macos", target_os = "linux")))]
fn spawn_notification(mut command: Command) {
    let _ = std::thread::Builder::new()
        .name(NOTIFICATION_THREAD_NAME.to_string())
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
    let mut command = Command::new(MACOS_NOTIFICATION_COMMAND);
    command.arg("-e").arg(script);
    spawn_notification(command);
}

/// Sends a best-effort notification without blocking the timer.
#[cfg(all(not(test), target_os = "linux"))]
pub fn notify(title: &str, message: &str) {
    let mut command = Command::new(LINUX_NOTIFICATION_COMMAND);
    command.arg(title).arg(message);
    spawn_notification(command);
}

/// Desktop notifications are disabled in tests and unsupported platforms.
#[cfg(any(test, not(any(target_os = "macos", target_os = "linux"))))]
pub fn notify(_title: &str, _message: &str) {}
