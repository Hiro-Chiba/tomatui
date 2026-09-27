use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const ICON: &[u8] = include_bytes!("../../assets/tomatui-icon.png");
static WORKERS: Mutex<Vec<JoinHandle<()>>> = Mutex::new(Vec::new());
static NEXT_FILE: AtomicU64 = AtomicU64::new(0);

pub(super) fn notify(title: &str, message: &str) {
    let title = title.to_owned();
    let message = message.to_owned();
    let Ok(mut workers) = WORKERS.lock() else {
        return;
    };
    workers.retain(|worker| !worker.is_finished());
    if let Ok(worker) = thread::Builder::new()
        .name("tomatui-notification".into())
        .spawn(move || {
            let Some(directory) = dirs::data_local_dir() else {
                return;
            };
            let Ok(icon) = install_icon(&directory.join("tomatui").join("notifications")) else {
                return;
            };
            #[cfg(target_os = "macos")]
            let _ = super::macos::send(&title, &message, &icon);
            #[cfg(target_os = "linux")]
            let _ = super::linux::send(&title, &message, &icon);
            #[cfg(target_os = "windows")]
            let _ = super::windows::send(&title, &message, &icon);
        })
    {
        workers.push(worker);
    }
}

pub(super) fn flush() {
    let workers = WORKERS
        .lock()
        .map(|mut workers| std::mem::take(&mut *workers))
        .unwrap_or_default();
    for worker in workers {
        let _ = worker.join();
    }
}

// Desktop services may hang. Never let one prevent the timer from exiting.
pub(super) fn run(command: &mut Command) -> io::Result<()> {
    run_with_timeout(command, Duration::from_secs(10))
}

fn run_with_timeout(command: &mut Command, timeout: Duration) -> io::Result<()> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                return if status.success() {
                    Ok(())
                } else {
                    Err(io::Error::other(format!(
                        "notification command exited with {status}"
                    )))
                };
            }
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(20)),
            result => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(result.err().unwrap_or_else(|| {
                    io::Error::new(io::ErrorKind::TimedOut, "notification command timed out")
                }));
            }
        }
    }
}

fn install_icon(directory: &Path) -> io::Result<PathBuf> {
    fs::create_dir_all(directory)?;
    let icon = directory.join("tomatui-icon.png");
    if fs::read(&icon).is_ok_and(|bytes| bytes == ICON) {
        return Ok(icon);
    }
    // Replace atomically so another timer never reads a partially written PNG.
    let id = NEXT_FILE.fetch_add(1, Ordering::Relaxed);
    let temporary = directory.join(format!(".icon-{}-{id}.tmp", std::process::id()));
    let result = (|| {
        fs::write(&temporary, ICON)?;
        fs::rename(&temporary, &icon)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;
    Ok(icon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stalled_notification_command_is_stopped() {
        #[cfg(unix)]
        let mut command = {
            let mut command = Command::new("sh");
            command.args(["-c", "exec sleep 30"]);
            command
        };
        #[cfg(windows)]
        let mut command = {
            let mut command = Command::new("powershell.exe");
            command.args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "Start-Sleep -Seconds 30",
            ]);
            command
        };
        let start = Instant::now();
        let error = run_with_timeout(&mut command, Duration::from_millis(100)).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::TimedOut);
        assert!(start.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn icon_install_is_repeatable_and_repairs_stale_files() {
        let directory = std::env::temp_dir().join(format!(
            "tomatui-icon-test-{}-{}",
            std::process::id(),
            NEXT_FILE.fetch_add(1, Ordering::Relaxed)
        ));
        let icon = install_icon(&directory).unwrap();
        assert_eq!(fs::read(&icon).unwrap(), ICON);
        let modified = fs::metadata(&icon).unwrap().modified().unwrap();
        assert_eq!(install_icon(&directory).unwrap(), icon);
        assert_eq!(fs::metadata(&icon).unwrap().modified().unwrap(), modified);
        fs::write(&icon, b"stale").unwrap();
        install_icon(&directory).unwrap();
        assert_eq!(fs::read(&icon).unwrap(), ICON);
        assert_eq!(fs::read_dir(&directory).unwrap().count(), 1);
        fs::remove_dir_all(directory).unwrap();
    }
}
