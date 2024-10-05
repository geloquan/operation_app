
use crate::{ws::{receive::TableTarget, types}, OperationApp};
pub mod local;
pub mod server;
/// REVIEW 
///  table-name      property-name       boolean-change
///       |               |                    |
///       V               V                    V
/// OperationTool       OnSite               Toogle
///                                                        
/// 
/// 
///                                                        
/// 
/// 
///                                                        
/// 
/// 
///                                                        
/// 
/// 
///                                                        
/// 
/// 
///                                                        
/// 
/// 


pub enum Options {
    OperationToolOnSiteToogle(i32)
}
pub trait Dispatch {
    fn action(&mut self, table_taget_name: TableTarget, option: Options);
}

impl Dispatch for OperationApp {
    fn action(&mut self, table_taget_name: TableTarget, option: Options) {
        match table_taget_name {
            TableTarget::All => todo!(),
            TableTarget::Equipment => todo!(),
            TableTarget::Room => todo!(),
            TableTarget::Tool => todo!(),
            TableTarget::Staff => todo!(),
            TableTarget::ToolReservation => todo!(),
            TableTarget::ToolDesignatedRoom => todo!(),
            TableTarget::ToolInspector => todo!(),
            TableTarget::Patient => todo!(),
            TableTarget::Operation => todo!(),
            TableTarget::PatientWardRoom => todo!(),
            TableTarget::PatientWardAssistant => todo!(),
            TableTarget::OperationStaff => todo!(),
            TableTarget::OperationTool => {
                if let (Options::OperationToolOnSiteToogle(s), Some(data)) = (option, &self.data) {
                    let mut operation_tool = data.operation_tool.write().unwrap();
                    for operation_tool in operation_tool.iter_mut() {
                        if let (Some(op_tool_id), Some(op_onsite)) = (&operation_tool.id, &mut operation_tool.on_site) {
                            if op_tool_id == &s {
                                if *op_onsite == 0 {
                                    *op_onsite = 1;
                                } else if *op_onsite == 1 {
                                    *op_onsite = 0;
                                }
                            }
                        }                        
                    }
                }
            },
            TableTarget::Alert => todo!(),
            TableTarget::Frontdesk => todo!(),
            TableTarget::AlertFrontdesk => todo!(),
            TableTarget::AlertStaff => todo!(),
        }
    }
}