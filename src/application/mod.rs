pub mod states;

pub mod authenticate;
pub mod field;

pub mod component;

pub mod menu;

pub mod data;

pub mod server_notification;

use crate::database::table::tree::TableTree;

pub struct RunningApp {
    pub operation_id: i32,
    pub tree: Option<TableTree>,
}