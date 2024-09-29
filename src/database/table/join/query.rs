use super::structure::OperationSelect;
use crate::{application, OperationApp};

impl OperationApp {
    pub fn get_operation(&mut self) -> Option<OperationSelect> {
        if let Some(ref data) = self.data {
            let operation = data.operation.read().unwrap();
            let patient = data.patient.read().unwrap();
            let room = data.room.read().unwrap();
    
            let operation_select: Option<OperationSelect> = operation.iter().find_map(|op| {
                if let Some(op_id) = op.id {
                    if let Some(running_app) = &self.state {
                        if &op_id == &running_app.operation_id {
                            let operation_label = op.label.clone().unwrap_or_else(|| "N/A".to_string());
                            let operation_status = op.status.clone().unwrap().to_string();
                            let patient_full_name = patient.iter()
                                .find(|p| op.patient_id.map(|id| id == p.id.unwrap()).unwrap_or(false))
                                .map(|p| format!(
                                    "{} {}",
                                    p.first_name.clone().unwrap_or_else(|| "N/A".to_string()),
                                    p.last_name.clone().unwrap_or_else(|| "N/A".to_string())
                                ))
                                .unwrap_or_else(|| "N/A".to_string());
        
                            let room_return = room.iter()
                                .find(|r| op.room_id.map(|id| id == r.id.unwrap()).unwrap_or(false))
                                .map(|r| 
                                    (
                                        r.name.clone().unwrap_or_else(|| "N/A".to_string()),
                                        r.alias_code.clone().unwrap_or_else(|| "N/A".to_string())
                                    )
                                )
                                .unwrap_or_else(|| ("N/A".to_string(), "N/A".to_string()));
        
                            return Some(OperationSelect {
                                operation_id: op_id,
                                operation_label,
                                operation_status,
                                patient_full_name,
                                room: room_return.0,
                                room_code: room_return.1,
                            });
                        }
                    }
                }
                None
            });
    
            operation_select
        } else {
            None
        }
    }

    pub fn filter_operation(&mut self) {
        println!("filter_operation()");
        if let Some(ref data) = self.data {
            println!("Some(ref data)");
            let operation = data.operation.read().unwrap();
            let patient = data.patient.read().unwrap();
            let room = data.room.read().unwrap();
    
            let operation_select: Vec<OperationSelect> = operation.iter().filter_map(|op| {
                if let Some(op_id) = op.id {
                    let operation_label = op.label.clone().unwrap_or_else(|| "N/A".to_string());
                    let operation_status = op.status.clone().unwrap().to_string();
                    let patient_full_name = patient.iter()
                        .find(|p| op.patient_id.map(|id| id == p.id.unwrap()).unwrap_or(false))
                        .map(|p| format!(
                            "{} {}",
                            p.first_name.clone().unwrap_or_else(|| "N/A".to_string()),
                            p.last_name.clone().unwrap_or_else(|| "N/A".to_string())
                        ))
                        .unwrap_or_else(|| "N/A".to_string());
    
                    let room_return = room.iter()
                        .find(|r| op.room_id.map(|id| id == r.id.unwrap()).unwrap_or(false))
                        .map(|r| 
                            (
                                r.name.clone().unwrap_or_else(|| "N/A".to_string()),
                                r.alias_code.clone().unwrap_or_else(|| "N/A".to_string())
                            )
                        )
                        .unwrap_or_else(|| ("N/A".to_string(), "N/A".to_string()));
    
                    if operation_label.to_lowercase().contains(&self.search.search_operation) ||
                        operation_status.to_lowercase().contains(&self.search.search_operation) ||
                        patient_full_name.to_lowercase().contains(&self.search.search_operation) ||
                        room_return.0.to_lowercase().contains(&self.search.search_operation) ||
                        room_return.1.to_lowercase().contains(&self.search.search_operation) {
                        return Some(OperationSelect {
                            operation_id: op_id,
                            operation_label,
                            operation_status,
                            patient_full_name,
                            room: room_return.0,
                            room_code: room_return.1,
                        });
                    }
                }
                None
            }).collect();

            println!("operation_select {:?}", operation_select);
    
            self.search.search_operation_result = operation_select.clone();
        } 
    }
}