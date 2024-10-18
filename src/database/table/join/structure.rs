use serde::{Deserialize, Serialize};

use crate::database::table::{public::{EquipmentStatus, OperationStatus}};

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct OperationSelect {
    pub operation_id: i32,
    pub operation_label: String,
    pub operation_status: String,
    pub patient_full_name: String,
    pub room: String,
    pub room_code: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreOperativeDefault {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreOperativeToolReady {
    pub operation_tool_id: i32,
    pub equipment_name: String,
    pub tool_status: EquipmentStatus,
    pub on_site: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionLogProperty {
    pub staff: String,
    pub label: String,
    pub label_reference: String,
    pub before_val: String,
    pub after_val: String,
    pub date: String,
}