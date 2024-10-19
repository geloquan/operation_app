use std::default;

use crate::{application::forms::NewEquipmentRequirement, database::table::public::OperationStatus};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub struct Menu {
    pub selected_menu: Option<MenuOptions>,
    pub selected_action: Option<Action>,
}


#[derive(PartialEq, Clone)]
pub enum MenuOptions {
    Staff,
    ToolReady,
}
#[derive(PartialEq, Clone)]
pub enum Action {
    AddRequirement(Option<NewEquipmentRequirement>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {
    pub op_id: i32,
    pub op_label: String,
    pub patient_full_name: String,
    pub op_status: OperationStatus,
    pub room_name: String,
    pub total_tools: i64,
    pub on_site_tools: i64,
    pub on_site_ratio: f64,
    pub on_site_percentage: f64,
    pub start_time: String,
    pub end_time: String,
    pub staff_count: i64
}