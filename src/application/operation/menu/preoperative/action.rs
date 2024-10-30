use serde::{Deserialize, Serialize};

use crate::database::table::public::EquipmentStatus;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct NewEquipmentRequirement {
    pub operation_id: i32,
    pub name: String,
    pub on_site: bool,
    pub quantity: i32,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RemoveEquipmentRequirement {
    pub id: i32,
    pub name: String,
    pub status: EquipmentStatus,
} 
