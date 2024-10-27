use std::default;

use crate::{database::table::public::OperationStatus};
use action::{NewEquipmentRequirement, RemoveEquipmentRequirement};
use serde::{Deserialize, Serialize};

pub mod action;

#[derive(Clone, Default)]
pub struct Menu {
    pub selected_menu: Option<MenuOptions>,
    pub selected_action: Option<Action>,
}


#[derive(PartialEq, Clone, Debug)]
pub enum MenuOptions {
    Staff,
    ToolReady,
}
#[derive(PartialEq, Clone, Debug)]
pub enum Action {
    AddRequirement(Option<NewEquipmentRequirement>),
    RemoveRequirement(Option<RemoveEquipmentRequirement>),
    AddStaffRole,
    RemoveStaffRole,
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
    pub staff_count: i64,

    pub approved_consent: bool,
}