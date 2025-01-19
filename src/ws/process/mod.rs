use egui_extras::Table;

use crate::{action::LogReturn, database::table::{self, join::structure::EquipmentRequestedProperty, private::OperationToolOnSiteToggle, public::{ActionLog, ActionLogGroup, Operation, OperationTool}}, OperationApp};

use super::{receive::{ReceiveMessage, TableTarget}, types};

pub trait Update {
    fn update(&mut self, message: ReceiveMessage);
}

impl Update for OperationApp {
    fn update(&mut self, message: ReceiveMessage) {
        if message.action.is_none() {
            return;
        };

        if let Some(TableTarget::ActionLog) = message.table_name {
            match serde_json::from_str::<LogReturn>(&message.data) {
                Ok(log_return) => {
                    
                    if let Some(data) = &self.data {
                        let action_log_return = &log_return.0;
                        let action_log_group_return = &log_return.1;

                        {
                            let mut action_log: std::sync::RwLockWriteGuard<'_, Vec<ActionLog>> = data.action_log.write().unwrap();
                            action_log.push(action_log_return[0].clone());
                        }
                        {
                            let mut action_log_group: std::sync::RwLockWriteGuard<'_, Vec<ActionLogGroup>> = data.action_log_group.write().unwrap();
                            action_log_group.push(action_log_group_return[0].clone());
                        }

                        match message.action.unwrap() {
                            table::public::ActionLogLabel::OnSiteToggle => {
                                let mut operation_tools= data.operation_tool.write().unwrap();
                                

                                for operation_tool in operation_tools.iter_mut() {
                                    if let (Some(tool_id), Some(new_value), Some(old_value)) = (&operation_tool.tool_id, &action_log_return[0].new_value, &action_log_return[0].old_value) {
                                        let operation_tool_value = serde_json::to_value(&operation_tool).expect("Failed to convert struct to Value");
                                        let new_operation_tool_value: OperationToolOnSiteToggle = serde_json::from_value(new_value.to_owned()).expect("Failed to convert struct from Value");
                                        let new_operation_tool = OperationTool {
                                            id: Some(new_operation_tool_value.operation_tool_id),
                                            operation_id: Some(new_operation_tool_value.operation_id),
                                            tool_id: Some(new_operation_tool_value.tool_id),
                                            on_site: if new_operation_tool_value.on_site_value { Some(1) } else { Some(0) },
                                        };
                                        if tool_id == old_value.get("operation_tool_id").unwrap() {
                                            *operation_tool = new_operation_tool;
                                        }
                                    }
                                }
                            },
                            table::public::ActionLogLabel::AddEquipmentRequirement => {
                                let mut equipment_requests= data.equipment_request.write().unwrap();

                                for action_log_ret in action_log_return {
                                    let equipment_request_value = serde_json::to_value(&action_log_ret.new_value).expect("Failed to convert struct to Value");
                                    let new_equipment_request_value: table::public::EquipmentRequest = serde_json::from_value(equipment_request_value.to_owned()).expect("Failed to convert struct from Value");
                                    println!("new_equipment_request_value: {:?}", new_equipment_request_value);
                                    equipment_requests.push(new_equipment_request_value);
                                }
                                
                            },
                            table::public::ActionLogLabel::RemoveEquipmentRequirement => {

                            },
                        };
                    }
                },
                Err(_) => {
                    println!("error actionlog match")
                },
            }
        };
    }
}