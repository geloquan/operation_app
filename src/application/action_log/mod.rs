
pub struct ActionLog {
    pub record: Vec<ActionLogProperty>
}
pub struct ActionLogProperty {
    pub menu: String,
    pub status: String,
    pub description: String,
    pub date: String,
}