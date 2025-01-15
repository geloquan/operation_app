use serde::{Deserialize, Serialize};

use crate::database::table::public::EquipmentStatus;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct NewEquipmentRequirement {
    pub operation_id: i32,
    pub tool_id: Option<i32>,
    pub name: String,
    pub to_claim: bool,
    pub to_claim_staff_id: Option<i32>,
    pub staff_search: String,
    pub quantity: i32,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RemoveEquipmentRequirement {
    pub id: i32,
    pub name: String,
    pub status: EquipmentStatus,
} 
