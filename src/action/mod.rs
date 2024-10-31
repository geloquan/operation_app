use serde::{Deserialize, Serialize};

use crate::{application::{data::dispatch::Dispatch, operation::{self, menu::preoperative::action::{NewEquipmentRequirement, RemoveEquipmentRequirement}}}, database::table::{private::{self, OperationToolOnSiteToggle}, public::{ActionLog, ActionLogGroup}}, OperationApp};

pub type LogReturn = (Vec<ActionLog>, Vec<ActionLogGroup>);
    
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


pub trait HandleAction {
    fn handle_action(&mut self); 
} 
impl HandleAction for OperationApp {
    fn handle_action(&mut self) {
        while let Ok(action) = &self.app_rx.try_recv() {
            let main_panel_reload: bool = match action {
                Actions::Preoperation(preoperation) => {
                    match preoperation {
                        Preoperation::ToolOnSiteToggle(_) => {
                            self.action(action.to_owned());
                            false
                        },
                        Preoperation::AddNewEquipmentRequirement(_) => {
                            self.action(action.to_owned());
                            true
                        },
                        Preoperation::RemoveEquipmentRequirement(_) => {
                            true
                        },
                    }
                },
            };
            if main_panel_reload {
                if let Some(operation_state) = &mut self.operation_state {
                    match operation_state {
                        operation::State::Preoperation(menu) => {
                            menu.selected_action = None;
                            menu.selected_menu = None;
                        },
                        operation::State::Intraoperation(menu) => {
                            menu.selected_action = None;
                            menu.selected_menu = None;
                        },
                        operation::State::Postoperation => todo!(),
                    }
                }
            }
        }
    }
}