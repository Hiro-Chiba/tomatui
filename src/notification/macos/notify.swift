import AppKit
import UserNotifications

final class NotificationDelegate: NSObject, NSApplicationDelegate, UNUserNotificationCenterDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        guard CommandLine.arguments.count == 3 else {
            NSApp.terminate(nil)
            return
        }
        let center = UNUserNotificationCenter.current()
        center.delegate = self
        center.requestAuthorization(options: [.alert]) { granted, _ in
            guard granted else {
                self.finish()
                return
            }
            let content = UNMutableNotificationContent()
            content.title = CommandLine.arguments[1]
            content.body = CommandLine.arguments[2]
            let request = UNNotificationRequest(identifier: UUID().uuidString, content: content, trigger: nil)
            center.add(request) { _ in self.finish() }
        }
        // Do not leave an invisible helper running if the permission dialog is ignored.
        DispatchQueue.main.asyncAfter(deadline: .now() + 120) { NSApp.terminate(nil) }
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification,
        withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
    ) {
        completionHandler([.banner, .list])
    }

    private func finish() {
        // Allow foreground delivery callbacks to run before terminating the app.
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) { NSApp.terminate(nil) }
    }
}

let app = NSApplication.shared
let delegate = NotificationDelegate()
app.delegate = delegate
app.run()
