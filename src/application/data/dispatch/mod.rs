
use crate::{action, ws::{receive::TableTarget, types}, OperationApp, SendMessage};
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
        if let action::Actions::OperationToolOnSiteToggle(operation_tool_on_site_toggle) = &action {
            let request_json = serde_json::to_string(&SendMessage {
                level: "Operation".to_string(),
                method: "Update".to_string(),
                data: None,
                staff_credential: self.staff.clone(),
                action: Some(action)
            }).unwrap();

            self.sender.send(ewebsock::WsMessage::Text(request_json));
        }
        //match table_taget_name {
        //    TableTarget::All => todo!(),
        //    TableTarget::Equipment => todo!(),
        //    TableTarget::Room => todo!(),
        //    TableTarget::Tool => todo!(),
        //    TableTarget::Staff => todo!(),
        //    TableTarget::ToolReservation => todo!(),
        //    TableTarget::ToolDesignatedRoom => todo!(),
        //    TableTarget::ToolInspector => todo!(),
        //    TableTarget::Patient => todo!(),
        //    TableTarget::Operation => todo!(),
        //    TableTarget::PatientWardRoom => todo!(),
        //    TableTarget::PatientWardAssistant => todo!(),
        //    TableTarget::OperationStaff => todo!(),
        //    TableTarget::OperationTool => {
        //        if let (Options::OperationToolOnSiteToogle(s), Some(data)) = (option, &self.data) {
        //            let mut operation_tool = data.operation_tool.write().unwrap();
        //            for operation_tool in operation_tool.iter_mut() {
        //                if let (Some(op_tool_id), Some(op_onsite)) = (&operation_tool.id, &mut operation_tool.on_site) {
        //                    if op_tool_id == &s {
        //                        if *op_onsite == 0 {
        //                            *op_onsite = 1;
        //                        } else if *op_onsite == 1 {
        //                            *op_onsite = 0;
        //                        }
        //                    }
        //                }                        
        //            }
        //        }
        //    },
        //    TableTarget::Alert => todo!(),
        //    TableTarget::Frontdesk => todo!(),
        //    TableTarget::AlertFrontdesk => todo!(),
        //    TableTarget::AlertStaff => todo!(),
        //    TableTarget::ActionLog => {
//
        //    }
        //}
    }
}