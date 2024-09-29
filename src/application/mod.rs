use crate::database::table::tree::TableTree;

pub struct RunningApp {
    pub operation_id: i32,
    pub tree: Option<TableTree>,
}