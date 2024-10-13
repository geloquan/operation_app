
use ewebsock::WsReceiver;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_tungstenite::tungstenite::http::status;
use crate::{application::authenticate::StaffCredential, cipher::{decrypt_message, generate_fixed_key, EncryptedText}, component::design, database::{self, table::{self, data::TableData}}, ws::process::Update, OperationApp, SendMessage};


#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub enum TableTarget {
    All,
    Equipment,
    Room,
    Tool,
    Staff,
    ToolReservation,
    ToolDesignatedRoom,
    ToolInspector,
    Patient,
    Operation,
    PatientWardRoom,
    PatientWardAssistant,
    OperationStaff,
    OperationTool,
    Alert,
    Frontdesk,
    AlertFrontdesk,
    AlertStaff,
    ActionLog
}
#[derive(Deserialize, Debug, Serialize)]
pub enum Operation {
    Initialize,
    Update,
    AuthHandshake
}
#[derive(Deserialize, Debug, Serialize)]
pub struct ReceiveMessage {
    pub table_name: TableTarget,
    pub operation: Operation,
    pub status_code: String,
    pub data: String,
}
pub trait Handle {
    fn handle_incoming(&mut self); 
} 
impl Handle for OperationApp {
    fn handle_incoming(&mut self) {}
}