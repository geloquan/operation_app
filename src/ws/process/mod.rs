use std::{ops::DerefMut};

use crate::{database::table::{self, data::{self, TableData}, public::{ActionLog, Operation}}, OperationApp};

use super::{receive::{ReceiveMessage, TableTarget}, types};

fn table(data: &TableData, action_log_data: &ActionLog) {
    //match action_log_data.table_name
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

pub trait Update {
    fn update(&mut self, message: ReceiveMessage);
}

impl Update for OperationApp {
    fn update(&mut self, message: ReceiveMessage) {
        if let TableTarget::ActionLog = message.table_name {
            match serde_json::from_str::<ActionLog>(&message.data) {
                Ok(action_log_data) => {
                    if let Ok(mut data) = self.data.write() {
                        if let Some(data) = data.deref_mut() {
    
                            {
                                let mut action_log = data.action_log.write().unwrap();
                                action_log.push(action_log_data.clone());
                            }
                            
                            table(data, &action_log_data);
                            
                        }
                    }
                },
                Err(_) => {
                    println!("error actionlog match")
                },
            }
        } else {
            return;
        }
    }
}