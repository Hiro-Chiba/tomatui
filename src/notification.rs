use std::io::{self, Write};

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
mod desktop;
#[cfg(any(test, target_os = "linux"))]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(any(test, target_os = "windows"))]
mod windows;

pub fn bell() {
    print!("\x07");
    let _ = io::stdout().flush();
}

/// Sends a best-effort notification without blocking the timer.
pub fn notify(title: &str, message: &str) {
    // Tests must never display notifications or alter OS registration.
    if cfg!(test) {
        return;
    }
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    desktop::notify(title, message);
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let _ = (title, message);
}

/// Finish dispatching notifications before the timer process exits.
pub fn flush() {
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    desktop::flush();
}
