use std::path::Path;
use std::process::{Command, Stdio};

fn command(title: &str, message: &str, icon: &Path) -> Command {
    let mut command = Command::new("notify-send");
    command
        .arg("--app-name=Tomatui")
        .arg("--icon")
        .arg(icon)
        .arg("--")
        .arg(title)
        .arg(message)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
}

#[cfg(target_os = "linux")]
pub(super) fn send(title: &str, message: &str, icon: &Path) -> std::io::Result<()> {
    super::desktop::run(&mut command(title, message, icon))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_and_text_are_passed_as_literal_arguments() {
        let command = command(
            "--title",
            "休憩 ' & $()",
            Path::new("/tmp/with spaces/icon.png"),
        );
        let args: Vec<_> = command.get_args().collect();
        assert_eq!(
            args,
            [
                "--app-name=Tomatui",
                "--icon",
                "/tmp/with spaces/icon.png",
                "--",
                "--title",
                "休憩 ' & $()"
            ]
        );
    }
}
