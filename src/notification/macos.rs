use super::desktop::run;
use std::{
    fs, io,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const HELPER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tomatui-notify"));
const ICON: &[u8] = include_bytes!("../../assets/tomatui-icon.icns");
const INFO: &[u8] = include_bytes!("macos/Info.plist");

pub(super) fn send(title: &str, message: &str, icon_path: &Path) -> io::Result<()> {
    let directory = icon_path
        .parent()
        .ok_or_else(|| io::Error::other("notification icon has no parent directory"))?;
    let bundle = install_bundle(directory)?;
    run(Command::new("open")
        .args(["-g", "-n"])
        .arg(bundle)
        .arg("--args")
        .arg(title)
        .arg(message))
}

fn bundle_name() -> String {
    // Content-addressed bundles let updates and concurrent timer processes coexist.
    let hash = HELPER
        .iter()
        .chain(ICON)
        .chain(INFO)
        .fold(0xcbf29ce484222325_u64, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
        });
    format!("Tomatui-{hash:016x}.app")
}

fn install_bundle(directory: &Path) -> io::Result<PathBuf> {
    let bundle = directory.join(bundle_name());
    if bundle.is_dir() {
        return Ok(bundle);
    }
    fs::create_dir_all(directory)?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)?
        .as_nanos();
    let staging = directory.join(format!(".Tomatui-{}-{nonce}.app", std::process::id()));
    let result = (|| {
        write_bundle(&staging)?;
        run(Command::new("/usr/bin/codesign")
            .args([
                "--force",
                "--sign",
                "-",
                "--identifier",
                "io.github.hiro-chiba.tomatui",
            ])
            .arg(&staging))?;
        match fs::rename(&staging, &bundle) {
            Ok(()) => Ok(bundle.clone()),
            // A second timer may have installed this exact bundle in the meantime.
            Err(_) if bundle.is_dir() => Ok(bundle.clone()),
            Err(error) => Err(error),
        }
    })();
    let _ = fs::remove_dir_all(&staging);
    result
}

fn write_bundle(bundle: &Path) -> io::Result<()> {
    let contents = bundle.join("Contents");
    fs::create_dir_all(contents.join("MacOS"))?;
    fs::create_dir_all(contents.join("Resources"))?;
    fs::write(contents.join("Info.plist"), INFO)?;
    fs::write(contents.join("Resources/tomatui-icon.icns"), ICON)?;
    let executable = contents.join("MacOS/tomatui-notify");
    fs::write(&executable, HELPER)?;
    fs::set_permissions(executable, fs::Permissions::from_mode(0o755))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_requests_default_sound_and_sound_permissions() {
        let executable = std::env::temp_dir().join(format!(
            "tomatui-notification-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let compilation = Command::new("xcrun")
            .args(["swiftc", "-D", "NOTIFICATION_TESTS"])
            .arg(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/notification/macos/notify.swift"
            ))
            .arg("-o")
            .arg(&executable)
            .output()
            .expect("compile macOS notification sound tests");
        assert!(
            compilation.status.success(),
            "{}",
            String::from_utf8_lossy(&compilation.stderr)
        );
        let result = Command::new(&executable)
            .output()
            .expect("run macOS notification sound tests");
        fs::remove_file(executable).unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&result.stdout).trim(),
            "macOS notification sound checks passed"
        );
    }

    #[test]
    fn bundle_contains_embedded_helper_icon_and_identity() {
        let directory = std::env::temp_dir().join(format!(
            "tomatui-bundle-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let bundle = install_bundle(&directory).unwrap();
        assert_eq!(install_bundle(&directory).unwrap(), bundle);
        run(Command::new("/usr/bin/codesign")
            .args(["--verify", "--strict"])
            .arg(&bundle))
        .unwrap();
        let contents = bundle.join("Contents");
        assert_eq!(fs::read(contents.join("Info.plist")).unwrap(), INFO);
        assert_eq!(
            fs::read(contents.join("Resources/tomatui-icon.icns")).unwrap(),
            ICON
        );
        let executable = contents.join("MacOS/tomatui-notify");
        // Signing changes the embedded Mach-O signature in the executable.
        assert!(fs::metadata(&executable).unwrap().len() > 0);
        assert_eq!(
            fs::metadata(executable).unwrap().permissions().mode() & 0o777,
            0o755
        );
        fs::remove_dir_all(directory).unwrap();
    }
}
