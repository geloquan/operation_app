use serde::{Deserialize, Serialize};

use crate::database::table::public::{EquipmentStatus, OperationStatus, StaffRole};

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
    pub action_log_group_id: i32,
    pub staff: String,
    pub label: String,
    pub date: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OperationStaffProperty {
    pub staff_id: i32,
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub role: StaffRole,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EquipmentRequestedProperty {
    pub id: i32,
    pub equipment_name: String,
    pub staff_name: String,
    pub count: i32,
}