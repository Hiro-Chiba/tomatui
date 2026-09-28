import AppKit
import UserNotifications

let authorizationOptions: UNAuthorizationOptions = [.alert, .sound]
let presentationOptions: UNNotificationPresentationOptions = [.banner, .list, .sound]

func notificationRequest(title: String, body: String) -> UNNotificationRequest {
    let content = UNMutableNotificationContent()
    content.title = title
    content.body = body
    content.sound = .default
    return UNNotificationRequest(identifier: UUID().uuidString, content: content, trigger: nil)
}

final class NotificationDelegate: NSObject, NSApplicationDelegate, UNUserNotificationCenterDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        guard CommandLine.arguments.count == 3 else {
            NSApp.terminate(nil)
            return
        }
        let center = UNUserNotificationCenter.current()
        center.delegate = self
        center.requestAuthorization(options: authorizationOptions) { granted, _ in
            guard granted else {
                self.finish()
                return
            }
            let request = notificationRequest(title: CommandLine.arguments[1], body: CommandLine.arguments[2])
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
        completionHandler(presentationOptions)
    }

    private func finish() {
        // Allow foreground delivery callbacks to run before terminating the app.
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) { NSApp.terminate(nil) }
    }
}

#if NOTIFICATION_TESTS
// Exercise the real framework objects without requesting permission or delivering notifications.
let request = notificationRequest(title: "Work complete", body: "Take a break")
assert(request.content.title == "Work complete")
assert(request.content.body == "Take a break")
assert(request.trigger == nil)
assert(request.content.sound == UNNotificationSound.default, "notification must use the default sound")
assert(authorizationOptions.contains(.sound), "authorization must include sound")
assert(presentationOptions.contains(.sound), "foreground presentation must include sound")
print("macOS notification sound checks passed")
#else
let app = NSApplication.shared
let delegate = NotificationDelegate()
app.delegate = delegate
app.run()
#endif
