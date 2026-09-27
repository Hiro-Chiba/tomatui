use std::path::Path;
use std::process::{Command, Stdio};

// Keep user-controlled text out of PowerShell source. XML text nodes and URI
// conversion also preserve punctuation, Unicode, and spaces in local paths.
const SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
function New-TomatuiToast([string] $title, [string] $message, [string] $iconPath) {
    [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] > $null
    [Windows.UI.Notifications.ToastNotification, Windows.UI.Notifications, ContentType = WindowsRuntime] > $null
    [Windows.Data.Xml.Dom.XmlDocument, Windows.Data.Xml.Dom.XmlDocument, ContentType = WindowsRuntime] > $null
    $xml = [Windows.Data.Xml.Dom.XmlDocument]::new()
    $xml.LoadXml('<toast activationType="protocol" launch="https://github.com/Hiro-Chiba/tomatui"><visual><binding template="ToastGeneric"><text/><text/><image placement="appLogoOverride"/></binding></visual><audio silent="true"/></toast>')
    $textNodes = $xml.GetElementsByTagName('text')
    $null = $textNodes.Item(0).AppendChild($xml.CreateTextNode($title))
    $null = $textNodes.Item(1).AppendChild($xml.CreateTextNode($message))
    $xml.GetElementsByTagName('image').Item(0).SetAttribute('src', ([System.Uri]::new($iconPath)).AbsoluteUri)
    return [Windows.UI.Notifications.ToastNotification]::new($xml)
}
try {
    $appId = 'HiroChiba.Tomatui'
    $iconPath = $env:TOMATUI_NOTIFICATION_ICON
    $key = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey('Software\Classes\AppUserModelId\' + $appId)
    try {
        $key.SetValue('DisplayName', 'Tomatui', [Microsoft.Win32.RegistryValueKind]::ExpandString)
        $key.SetValue('IconUri', $iconPath, [Microsoft.Win32.RegistryValueKind]::ExpandString)
        # A stub CLSID keeps notifications in Action Center without a COM server.
        # Such unpackaged toasts use protocol activation when clicked.
        $key.SetValue('CustomActivator', '{2358C6E4-4CCE-4ED6-938B-4D17F254373A}')
    } finally {
        $key.Dispose()
    }
    $toast = New-TomatuiToast $env:TOMATUI_NOTIFICATION_TITLE $env:TOMATUI_NOTIFICATION_MESSAGE $iconPath
    [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier($appId).Show($toast)
} catch {
    exit 1
}
"#;

fn command(title: &str, message: &str, icon_path: &Path) -> Command {
    let mut command = Command::new("powershell.exe");
    command
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            SCRIPT,
        ])
        .env("TOMATUI_NOTIFICATION_TITLE", title)
        .env("TOMATUI_NOTIFICATION_MESSAGE", message)
        .env("TOMATUI_NOTIFICATION_ICON", icon_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command
}

#[cfg(target_os = "windows")]
pub(super) fn send(title: &str, message: &str, icon_path: &Path) -> std::io::Result<()> {
    super::desktop::run(&mut command(title, message, icon_path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn notification_content_is_data_not_powershell_source() {
        let title = "Tomatui's <timer> & \"break\"";
        let message = "休憩です\n$(Start-Process malicious.exe); `quoted`";
        let icon = Path::new(r"C:\Users\O'Brien\App Data\トマト & timer.png");
        let command = command(title, message, icon);
        let args: Vec<_> = command.get_args().collect();
        assert_eq!(args.last().copied(), Some(OsStr::new(SCRIPT)));
        assert!(!SCRIPT.contains(title));
        assert!(!SCRIPT.contains(message));
        let env: std::collections::HashMap<_, _> = command.get_envs().collect();
        assert_eq!(
            env[OsStr::new("TOMATUI_NOTIFICATION_TITLE")],
            Some(OsStr::new(title))
        );
        assert_eq!(
            env[OsStr::new("TOMATUI_NOTIFICATION_MESSAGE")],
            Some(OsStr::new(message))
        );
        assert_eq!(
            env[OsStr::new("TOMATUI_NOTIFICATION_ICON")],
            Some(icon.as_os_str())
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn powershell_builds_toast_without_registering_or_showing_it() {
        // Extract only the pure builder from the parsed production script. Do
        // not execute its registry registration or notification Show call.
        let title = "Tomatui's <timer> & \"break\"";
        let message = "休憩です\n$(Start-Process malicious.exe); `quoted`";
        let icon = r"C:\Users\O'Brien\App Data\トマト & timer.png";
        let output = Command::new("powershell.exe")
            .args([
                "-NoLogo",
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                r#"
$ErrorActionPreference = 'Stop'
$errors = $null
$tokens = $null
$ast = [System.Management.Automation.Language.Parser]::ParseInput($env:TOMATUI_TEST_SCRIPT, [ref]$tokens, [ref]$errors)
if ($errors.Count -gt 0) { throw ($errors | Out-String) }
$builder = $ast.Find({ param($node) $node -is [System.Management.Automation.Language.FunctionDefinitionAst] -and $node.Name -eq 'New-TomatuiToast' }, $true)
if ($null -eq $builder) { throw 'Toast builder is missing' }
. ([scriptblock]::Create($builder.Extent.Text))
$toast = New-TomatuiToast $env:TOMATUI_TEST_TITLE $env:TOMATUI_TEST_MESSAGE $env:TOMATUI_TEST_ICON
$xml = $toast.Content
$text = $xml.GetElementsByTagName('text')
if ($text.Item(0).InnerText -cne $env:TOMATUI_TEST_TITLE) { throw 'Title changed' }
if ($text.Item(1).InnerText -cne $env:TOMATUI_TEST_MESSAGE) { throw 'Message changed' }
$image = $xml.GetElementsByTagName('image').Item(0)
if ($image.GetAttribute('placement') -ne 'appLogoOverride') { throw 'Missing app logo' }
if ($image.GetAttribute('src') -cne ([System.Uri]::new($env:TOMATUI_TEST_ICON)).AbsoluteUri) { throw 'Icon URI changed' }
if ($xml.GetElementsByTagName('audio').Item(0).GetAttribute('silent') -ne 'true') { throw 'Toast must not duplicate the terminal bell' }
if ($xml.DocumentElement.GetAttribute('activationType') -ne 'protocol') { throw 'Stub activator requires protocol activation' }
if ($xml.DocumentElement.GetAttribute('launch') -ne 'https://github.com/Hiro-Chiba/tomatui') { throw 'Unexpected click destination' }
"#,
            ])
            .env("TOMATUI_TEST_SCRIPT", SCRIPT)
            .env("TOMATUI_TEST_TITLE", title)
            .env("TOMATUI_TEST_MESSAGE", message)
            .env("TOMATUI_TEST_ICON", icon)
            .stdin(Stdio::null())
            .output()
            .expect("Windows PowerShell should be installed");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
