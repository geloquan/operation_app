pub mod states;

pub mod authenticate;
pub mod field;

pub mod component;

use crate::database::table::tree::TableTree;

pub struct RunningApp {
    pub operation_id: i32,
    pub tree: Option<TableTree>,
}