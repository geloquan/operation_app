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