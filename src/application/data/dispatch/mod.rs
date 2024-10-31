
use std::borrow::Borrow;

use crate::{action::{self, Preoperation}, OperationApp, SendMessage};
pub mod local;
pub mod server;
/// REVIEW 
///  table-name      property-name       boolean-change
///       |               |                    |
///       V               V                    V
/// OperationTool       OnSite               Toogle
///                                                        

pub trait Dispatch {
    fn action(&mut self, action: action::Actions);
}
// updating property
impl Dispatch for OperationApp {
    fn action(&mut self, action: action::Actions) {
        match &action {
            action::Actions::Preoperation(preoperation) => {
                match preoperation {
                    Preoperation::ToolOnSiteToggle(_) => {
                        let request_json = serde_json::to_string(&SendMessage {
                            level: "Operation".to_string(),
                            method: "Update".to_string(),
                            data: None,
                            staff_credential: self.staff.clone(),
                            action: Some(action.clone())
                        }).unwrap();
            
                        self.sender.send(ewebsock::WsMessage::Text(request_json));
                    },
                    Preoperation::AddNewEquipmentRequirement(_) => {
                        let request_json = serde_json::to_string(&SendMessage {
                            level: "Operation".to_string(),
                            method: "Update".to_string(),
                            data: None,
                            staff_credential: self.staff.clone(),
                            action: Some(action.clone())
                        }).unwrap();
            
                        self.sender.send(ewebsock::WsMessage::Text(request_json));
                    },
                    Preoperation::RemoveEquipmentRequirement(remove_equipment_requirement) => {
                        
                    },
                }
            },
        }
    }
}