use egui_extras::Table;

use crate::{database::table::{self, public::{ActionLog, Operation}}, OperationApp};

use super::{receive::{ReceiveMessage, TableTarget}, types};

pub trait Update {
    fn update(&mut self, message: ReceiveMessage);
}

impl Update for OperationApp {
    fn update(&mut self, message: ReceiveMessage) {
        if let TableTarget::ActionLog = message.table_name {
            match serde_json::from_str::<ActionLog>(&message.data) {
                Ok(action_log_data) => {
                    println!("action log:{:?}", action_log_data);
                    match action_log_data.table_name {
                        Some(ref table) => {
                            match table {
                                table::Tables::Equipment => todo!(),
                                table::Tables::Room => todo!(),
                                table::Tables::Tool => todo!(),
                                table::Tables::Staff => todo!(),
                                table::Tables::ToolReservation => todo!(),
                                table::Tables::ToolDesignatedRoom => todo!(),
                                table::Tables::ToolInspector => todo!(),
                                table::Tables::Patient => todo!(),
                                table::Tables::Operation => todo!(),
                                table::Tables::PatientWardRoom => todo!(),
                                table::Tables::PatientWardAssistant => todo!(),
                                table::Tables::OperationStaff => todo!(),
                                table::Tables::OperationTool => {
                                    if let Some(data) = &self.data {
                                        {
                                            println!("data {:?}", data);
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
                                                                1 as i8
                                                            } else {
                                                                0 as i8
                                                            };
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                table::Tables::Alert => todo!(),
                                table::Tables::Frontdesk => todo!(),
                                table::Tables::AlertFrontdesk => todo!(),
                                table::Tables::AlertStaff => todo!(),
                                table::Tables::ActionLog => todo!(),
                            }
                        },
                        None => todo!(),
                    }
                },
                Err(_) => {
                    println!("error actionlog match")
                },
            }
        };
    }
}