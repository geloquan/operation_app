use crate::{database::table::{self, data, public::{ActionLog, Operation}}, OperationApp};

use super::{receive::{ReceiveMessage, TableTarget}, types};

pub trait Update {
    fn update(&mut self, message: ReceiveMessage);
}

impl Update for OperationApp {
    fn update(&mut self, message: ReceiveMessage) {
        match message.table_name {
            TableTarget::All => todo!(),
            TableTarget::Equipment => todo!(),
            TableTarget::Room => todo!(),
            TableTarget::Tool => todo!(),
            TableTarget::Staff => todo!(),
            TableTarget::ToolReservation => todo!(),
            TableTarget::ToolDesignatedRoom => todo!(),
            TableTarget::ToolInspector => todo!(),
            TableTarget::Patient => todo!(),
            TableTarget::Operation => {
                match serde_json::from_str::<Vec<types::Update>>(&message.data) {
                    Ok(updates) => {
                        for update in updates {
                            if let Some(data) = &self.data {
                                let mut operation: std::sync::RwLockWriteGuard<'_, Vec<Operation>> = data.operation.write().unwrap();
                                for op in operation.iter_mut() {
                                    if op.id == Some(update.id) {
                                        match serde_json::from_str::<Operation>(&update.new_row_data) {
                                            Ok(new_row_data) => {
                                                *op = new_row_data;
                                                self.require_update = true;
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Err(_) => todo!(),
                }
            },
            TableTarget::PatientWardRoom => todo!(),
            TableTarget::PatientWardAssistant => todo!(),
            TableTarget::OperationStaff => todo!(),
            TableTarget::OperationTool => todo!(),
            TableTarget::Alert => todo!(),
            TableTarget::Frontdesk => todo!(),
            TableTarget::AlertFrontdesk => todo!(),
            TableTarget::AlertStaff => todo!(),
            TableTarget::ActionLog => {
                match serde_json::from_str::<ActionLog>(&message.data) {
                    Ok(action_log_data) => {
                        if let Some(data) = &self.data {
                            {
                                let mut action_log: std::sync::RwLockWriteGuard<'_, Vec<ActionLog>> = data.action_log.write().unwrap();
                                action_log.push(action_log_data.clone());
                            }
                            {
                                let mut operation_tools= data.operation_tool.write().unwrap();
                                for operation_tool in operation_tools.iter_mut() {
                                    if let (Some(tool_id), Some(row_id), Some(new_value)) = (&operation_tool.tool_id, &action_log_data.row_id, &action_log_data.new_value) {
                                        if tool_id == row_id {
                                            if let Some(on_site) = &mut operation_tool.on_site {
                                                *on_site = if new_value == "1" {
                                                    println!("set to true");
                                                    1 as i8
                                                } else {
                                                    println!("tool_id: {:?}", tool_id);
                                                    println!("row_id: {:?}", row_id);
                                                    println!("new_value: {:?}", new_value);
                                                    println!("set to false");
                                                    0 as i8
                                                };
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Err(_) => {
                        println!("error actionlog match")
                    },
                }
            }
        }
    }
}