use serde::{Deserialize, Serialize};

use crate::{application::operation::menu::preoperative::action::{NewEquipmentRequirement, RemoveEquipmentRequirement}, database::table::private::{self, OperationToolOnSiteToggle}};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum Actions {
    Preoperation(Preoperation)
}
#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum Preoperation {
    ToolOnSiteToggle(OperationToolOnSiteToggle),
    AddNewEquipmentRequirement(NewEquipmentRequirement),
    RemoveEquipmentRequirement(RemoveEquipmentRequirement)
}
enum Intraoperation {

}