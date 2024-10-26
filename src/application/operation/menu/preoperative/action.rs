use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct NewEquipmentRequirement {
    pub name: String,
    pub on_site: bool,
    pub quantity: i32,
}