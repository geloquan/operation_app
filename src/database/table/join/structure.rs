use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct OperationSelect {
    pub operation_id: i32,
    pub operation_label: String,
    pub operation_status: String,
    pub patient_full_name: String,
    pub room: String,
    pub room_code: String
}