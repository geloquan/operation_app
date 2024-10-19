pub mod states;

pub mod authenticate;
pub mod field;

pub mod component;

pub mod data;

pub mod server_notification;

use crate::database::table::tree::TableTree;

pub mod forms;

pub mod operation;

pub struct RunningApp {
    pub operation_id: i32,
    pub tree: Option<TableTree>,
}