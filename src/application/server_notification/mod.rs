#[derive(Debug)]
pub struct ServerNotification {
    pub record: Vec<ServerNotificationProperty>
}
#[derive(Debug)]
pub struct ServerNotificationProperty {
    pub menu: String,
    pub description: String,
    pub staff_full_name: String,
    pub date: String,
}
