use crate::{database::table::{self, data, public::Operation}, OperationApp};

use super::{receive::TableTarget, types};

pub trait Update {
    fn update(&mut self, table_name: TableTarget, string_data: &str);
}

impl Update for OperationApp {
    fn update(&mut self, table_name: TableTarget, string_data: &str) {
        match table_name {
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
                match serde_json::from_str::<Vec<types::Update>>(&string_data) {
                    Ok(updates) => {
                        for update in updates {
                            if let Some(data) = &self.data {
                                let mut operation = data.operation.write().unwrap();
                                for op in operation.iter_mut() {
                                    if op.id == Some(update.id) {
                                        *op = update.new_row_data.clone(); 
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
        }
    }
}