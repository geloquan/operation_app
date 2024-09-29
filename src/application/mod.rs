use super::database::table::data::TableData;
pub struct RunningApp {
    pub operation_id: i32,
    pub data: Option<TableData>,
}