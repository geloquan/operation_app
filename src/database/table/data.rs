use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};


use super::*;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TableRow {
    Equipment(public::Equipment), 
    Room(public::Room),           
    Tool(public::Tool),           
    Staff(public::Staff),         
    ToolReservation(public::ToolReservation), 
    ToolDesignatedRoom(public::ToolDesignatedRoom),
    ToolInspector(public::ToolInspector),  
    Patient(public::Patient),              
    Operation(public::Operation),          
    PatientWardRoom(public::PatientWardRoom), 
    PatientWardAssistant(public::PatientWardAssistant), 
    OperationStaff(public::OperationStaff),  
    OperationTool(public::OperationTool), 
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RawTable {
    pub equipment: Vec<public::Equipment>,
    pub room: Vec<public::Room>,
    pub tool: Vec<public::Tool>,
    pub staff: Vec<public::Staff>,                          
    pub tool_reservation: Vec<public::ToolReservation>,     
    pub tool_designated_room: Vec<public::ToolDesignatedRoom>, 
    pub tool_inspector: Vec<public::ToolInspector>,        
    pub patient: Vec<public::Patient>,                     
    pub operation: Vec<public::Operation>,                 
    pub patient_ward_room: Vec<public::PatientWardRoom>,   
    pub patient_ward_assistant: Vec<public::PatientWardAssistant>, 
    pub operation_staff: Vec<public::OperationStaff>,       
    pub operation_tool: Vec<public::OperationTool>,
    pub alert: Vec<public::Alert>,
    pub frontdesk: Vec<public::Frontdesk>,
    pub alert_frontdesk: Vec<public::AlertFrontdesk>,
    pub alert_staff: Vec<public::AlertStaff>,
    pub action_log: Vec<public::ActionLog>
}
#[derive(Debug, Clone)]
pub struct TableData {
    pub equipment: Arc<RwLock<Vec<public::Equipment>>>,
    pub room: Arc<RwLock<Vec<public::Room>>>,
    pub tool: Arc<RwLock<Vec<public::Tool>>>,
    pub staff: Arc<RwLock<Vec<public::Staff>>>,
    pub tool_reservation: Arc<RwLock<Vec<public::ToolReservation>>>,
    pub tool_designated_room: Arc<RwLock<Vec<public::ToolDesignatedRoom>>>,
    pub tool_inspector: Arc<RwLock<Vec<public::ToolInspector>>>,
    pub patient: Arc<RwLock<Vec<public::Patient>>>,
    pub operation: Arc<RwLock<Vec<public::Operation>>>,
    pub patient_ward_room: Arc<RwLock<Vec<public::PatientWardRoom>>>,
    pub patient_ward_assistant: Arc<RwLock<Vec<public::PatientWardAssistant>>>,
    pub operation_staff: Arc<RwLock<Vec<public::OperationStaff>>>,
    pub operation_tool: Arc<RwLock<Vec<public::OperationTool>>>,
    pub alert: Arc<RwLock<Vec<public::Alert>>>,
    pub frontdesk:Arc<RwLock< Vec<public::Frontdesk>>>,
    pub alert_frontdesk: Arc<RwLock<Vec<public::AlertFrontdesk>>>,
    pub alert_staff: Arc<RwLock<Vec<public::AlertStaff>>>,
    pub action_log: Arc<RwLock<Vec<public::ActionLog>>>
}
impl TableData {
    pub fn new() -> Self {
        TableData {
            equipment: Arc::new(RwLock::new(Vec::new())),
            room: Arc::new(RwLock::new(Vec::new())),
            tool: Arc::new(RwLock::new(Vec::new())),
            staff: Arc::new(RwLock::new(Vec::new())),
            tool_reservation: Arc::new(RwLock::new(Vec::new())),
            tool_designated_room: Arc::new(RwLock::new(Vec::new())),
            tool_inspector: Arc::new(RwLock::new(Vec::new())),
            patient: Arc::new(RwLock::new(Vec::new())),
            operation: Arc::new(RwLock::new(Vec::new())),
            patient_ward_room: Arc::new(RwLock::new(Vec::new())),
            patient_ward_assistant: Arc::new(RwLock::new(Vec::new())),
            operation_staff: Arc::new(RwLock::new(Vec::new())),
            operation_tool: Arc::new(RwLock::new(Vec::new())),
            alert: Arc::new(RwLock::new(Vec::new())),
            frontdesk: Arc::new(RwLock::new(Vec::new())),
            alert_frontdesk: Arc::new(RwLock::new(Vec::new())),
            alert_staff: Arc::new(RwLock::new(Vec::new())),
            action_log: Arc::new(RwLock::new(Vec::new())),
        }
    }
    pub fn initialize(&mut self, raw_string: String) {
        let raw_table: RawTable = serde_json::from_str(&raw_string).expect("parse error");
        println!("raw table: {:?}", raw_table);
        self.equipment = Arc::new(RwLock::new(raw_table.equipment.clone()));
        self.room = Arc::new(RwLock::new(raw_table.room.clone()));
        self.tool = Arc::new(RwLock::new(raw_table.tool.clone()));
        self.staff = Arc::new(RwLock::new(raw_table.staff.clone()));
        self.tool_reservation = Arc::new(RwLock::new(raw_table.tool_reservation.clone()));
        self.tool_designated_room = Arc::new(RwLock::new(raw_table.tool_designated_room.clone()));
        self.tool_inspector = Arc::new(RwLock::new(raw_table.tool_inspector.clone()));
        self.patient = Arc::new(RwLock::new(raw_table.patient.clone()));
        self.operation = Arc::new(RwLock::new(raw_table.operation.clone()));
        self.patient_ward_room = Arc::new(RwLock::new(raw_table.patient_ward_room.clone()));
        self.patient_ward_assistant = Arc::new(RwLock::new(raw_table.patient_ward_assistant.clone()));
        self.operation_staff = Arc::new(RwLock::new(raw_table.operation_staff.clone()));
        self.operation_tool = Arc::new(RwLock::new(raw_table.operation_tool.clone()));
        self.alert = Arc::new(RwLock::new(raw_table.alert.clone()));
        self.frontdesk = Arc::new(RwLock::new(raw_table.frontdesk.clone()));
        self.alert_frontdesk = Arc::new(RwLock::new(raw_table.alert_frontdesk.clone()));
        self.alert_staff = Arc::new(RwLock::new(raw_table.alert_staff.clone()));
        self.action_log = Arc::new(RwLock::new(raw_table.action_log.clone()));
    }
    //pub fn update(&self, raw_string: String, database_table: DatabaseTable) {
    //    match serde_json::from_str::<UpdateEquipmentRow>(&raw_string) {
    //        Ok(update_table_data) => {
    //            let mut rows = self.equipment.write().unwrap();
    //            //if let Some(row) = rows.iter_mut().find(|r| r.id.unwrap() == update_table_data.id as i32) {
    //            //    *row = update_table_data.new_row_data;
    //            //} else {
    //            //}
    //        },
    //        Err(_) => todo!(),
    //    }
    //}
}

 