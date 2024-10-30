use egui_extras::Table;

use crate::{database::table::{self, public::{ActionLog, Operation}}, OperationApp};

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
            match serde_json::from_str::<ActionLog>(&message.data) {
                Ok(action_log_data) => {
                    if let Some(data) = &self.data {
                        match message.action.unwrap() {
                            table::public::ActionLogLabel::OnSiteToggle => {
                                {
                                    let mut action_log: std::sync::RwLockWriteGuard<'_, Vec<ActionLog>> = data.action_log.write().unwrap();
                                    action_log.push(action_log_data.clone());
                                }
                                
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
                            },
                            table::public::ActionLogLabel::AddEquipmentRequirement => {
                                
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