use super::structure::{OperationSelect, PreOperativeDefault, PreOperativeToolReady};
use crate::{application, database::table::{data, public::{EquipmentStatus, OperationStatus}}, OperationApp};

impl OperationApp {
    pub fn select_operation(&mut self, id: &i32) {
        self.operation_id = Some(*id);
    }
    pub fn get_selected_operation(&mut self) -> Option<PreOperativeDefault> {
        if let Some(ref data) = self.data {
            let operation = data.operation.read().unwrap();
            let patient = data.patient.read().unwrap();
            let room = data.room.read().unwrap();
            let operation_tool = data.operation_tool.read().unwrap();
            
            let operation_staff = data.operation_staff.read().unwrap();
    
            let operation_select: Option<PreOperativeDefault> = operation.iter().find_map(|op| {
                if let Some(op_id) = op.id {
                    if let Some(operation_id) = &self.operation_id {
                        if &op_id == operation_id {
                            let op_id = op_id;
                            let op_id_opt = Some(op_id);
                            let op_label = op.label.clone().unwrap_or_else(|| "N/A".to_string());
                            let op_status = op.status.clone().unwrap_or_else(|| OperationStatus::Discharge);
                    
                            let patient_full_name = patient.iter()
                                .find(|p| op.patient_id.map(|id| id == p.id.unwrap()).unwrap_or(false))
                                .map(|p| format!("{} {}", p.first_name.clone().unwrap_or_else(|| "N/A".to_string()), p.last_name.clone().unwrap_or_else(|| "N/A".to_string()))) // CONCAT operation
                                .unwrap_or_else(|| "N/A".to_string()); 
                    
                            let room_name = room.iter()
                                .find(|r| op.room_id.map(|id| id == r.id.unwrap()).unwrap_or(false))
                                .map(|r| r.name.clone().unwrap_or_else(|| "N/A".to_string()))
                                .unwrap_or_else(|| "N/A".to_string()); 
                    
                            let total_tools = operation_tool.iter()
                                .filter(|ot| op_id_opt.map(|id| id == ot.operation_id.unwrap()).unwrap_or(false))
                                .count() as i64;
                    
                            let on_site_tools = operation_tool.iter()
                                .filter(|ot| op_id_opt.map(|id| id == ot.operation_id.unwrap() && match ot.on_site { Some(1) => true, _ => false }).unwrap_or(false))
                                .count() as i64;



                            let staff_count = operation_staff.iter()
                            .filter(|ops| op_id_opt.map(|id| id == ops.operation_id.unwrap()).unwrap_or(false))
                            .count() as i64;
                    
                            let on_site_ratio = if total_tools > 0 {
                                on_site_tools as f64 / total_tools as f64
                            } else {
                                0.0
                            };
                                    
                            let on_site_percentage = on_site_ratio * 100.0;

                            return Some(PreOperativeDefault {
                                op_id,
                                op_label,
                                patient_full_name,
                                op_status,
                                room_name,
                                total_tools,
                                on_site_tools,
                                on_site_ratio,
                                on_site_percentage,
                                start_time: op.start_time.clone().unwrap_or_else(|| "N/A".to_string()), 
                                end_time: op.end_time.clone().unwrap_or_else(|| "N/A".to_string()),   
                                staff_count
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
        if &self.search.search_operation == "" {
            self.search.search_operation_result.clear();
        } else if let Some(ref data) = self.data {
            println!("data {:?}", data);
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

    pub fn get_preoperative_tool_ready(&mut self) -> Option<Vec<PreOperativeToolReady>> {
        
        if let (Some(data), Some(operation_id)) = (&self.data, &self.operation_id) {
            
            let operation_tools = data.operation_tool.read().unwrap();
            let operations = data.operation.read().unwrap();
            let tools = data.tool.read().unwrap();
            let equipment = data.equipment.read().unwrap();

            let mut list: Vec<PreOperativeToolReady> = operation_tools.iter()
                .filter(|op_tool| {
                    operations.iter().any(|op| op.id.unwrap_or_else(|| 0) == op_tool.operation_id.unwrap_or_else(|| -1) && &op.id.unwrap_or_else(|| 0) == operation_id)
                })
                .filter_map(|op_tool| {
                    if let Some(op_tool_id) = op_tool.id {
                        let tool = tools.iter().find(|t| t.id == op_tool.tool_id);
                        let equipment_item = tool.and_then(|t| equipment.iter().find(|e| e.id == t.info_id));
                        
                        let tool_name = equipment_item
                            .map_or("Unknown Tool".to_string(), |e| e.name.clone().unwrap_or_else(|| "N/A".to_string()));
            
                        let tool_status = tool
                            .map_or(EquipmentStatus::ForInspection, |t| t.status.clone().unwrap_or_else(|| EquipmentStatus::ForInspection));
            
                        Some(PreOperativeToolReady {
                            operation_tool_id: op_tool_id,
                            equipment_name: tool_name,
                            on_site: op_tool.on_site.map_or(false, |value| value == 1), // Assuming `on_site` is an Option<bool>
                            tool_status,
                        })
                    } else {
                        None
                    }
                })
                .collect();
            list.sort_by(|a, b| {
                a.on_site.cmp(&b.on_site)
            });
            Some(list)
        } else {
            None
        }
    
    }
}