extern crate notify_rust;

pub fn persistant_notification(summary: &str, body: &str, icon: &str) -> u32 {
    notify_rust::Notification::new()
        .summary(summary)
        .body(body)
        .icon(icon)
        .timeout(0)
        .show()
}
