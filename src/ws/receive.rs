
use egui::accesskit::Action;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{application::{authenticate::StaffCredential, operation::menu::intraoperative::Menu}, cipher::{decrypt_message, generate_fixed_key, EncryptedText}, component::design, database::table::{data::TableData, private, public::ActionLogLabel}, ws::process::Update, OperationApp, SendMessage};


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
    AuthHandshake,
    Ascend,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct ReceiveMessage {
    pub table_name: Option<TableTarget>,
    pub operation: Operation,
    pub action: Option<ActionLogLabel>,
    pub status_code: String,
    pub data: String,
}
pub trait Handle {
    fn handle_incoming(&mut self); 
} 
impl Handle for OperationApp {
    fn handle_incoming(&mut self) {
        if let Some(msg) = self.receiver.try_recv() {
            match msg {
                ewebsock::WsEvent::Opened => {
                    
                },
                ewebsock::WsEvent::Message(text) => {
                    match text {
                        ewebsock::WsMessage::Binary(vec) => todo!(),
                        ewebsock::WsMessage::Text(text) => {
                            let text_len = text.len();
                            match serde_json::from_str::<EncryptedText>(&text) {
                                Ok(encrypted_text) => {
                                    if let Ok(key) = &generate_fixed_key() {
                                        if let Ok(decrypted_text) = decrypt_message(key, &encrypted_text.nonce, &encrypted_text.cipher_text) {
                                            match serde_json::from_str::<ReceiveMessage>(&decrypted_text) {
                                                Ok(message) => {
                                                    match message.operation {
                                                        Operation::Initialize => {
                                                            println!("initialize: {:?}", message);
                                                            if let Some(data) = &mut self.data {
                                                                data.initialize(message.data);
                                                            } else {
                                                                let mut new_table_data = TableData::new();
                                                                new_table_data.initialize(message.data);
                                                                self.data = Some(new_table_data);
                                                            }
                                                        },
                                                        Operation::Update => {
                                                            println!("update: {:?}", message);
                                                            self.update(message);
                                                        },
                                                        Operation::AuthHandshake => {
                                                            println!("statuscode {:?}", message.status_code);
                                                            if let Ok(staff_credential) = serde_json::from_str::<StaffCredential>(&message.data) {
                                                                self.staff = Some(staff_credential);
                                                            } else {
                                                                self.staff = None;
                                                            }
                                                            if message.status_code == "success" { 
                                                                self.credential_panel.state = design::State::Valid
                                                            }
                                                            else if message.status_code == "failed" { 
                                                                self.credential_panel.state = design::State::Error 
                                                            }
                                                        }
                                                        Operation::Ascend => {
                                                            println!("message: {:?}", message);
                                                            if let Ok(operation_ascend) = serde_json::from_str::<private::OperationAscend>(&message.data) {
                                                                self.require_update = true;
                                                                if let (Some(operation_state), Some(operation_id)) = (&mut self.operation_state, &self.operation_id) {
                                                                    if operation_ascend.operation_id == operation_id.to_owned() {
                                                                        match operation_state {
                                                                            crate::application::operation::State::Preoperation(_) => *operation_state = crate::application::operation::State::Intraoperation(
                                                                                Menu {
                                                                                    selected_action: None,
                                                                                    selected_menu: None
                                                                                }
                                                                            ),
                                                                            crate::application::operation::State::Intraoperation(_) => *operation_state = crate::application::operation::State::Postoperation,
                                                                            crate::application::operation::State::Postoperation => todo!(),
                                                                        } 
                                                                    }
                                                                }
                                                            }
                                                        },
                                                    }
                                                },
                                                Err(_) => {
                                                    println!("err parsing: ReceiveMessage");
                                                },
                                            }
                                        }
                                    }
                                },
                                Err(_) => {
                                    
                                },
                            }
                        },
                        ewebsock::WsMessage::Unknown(_) => todo!(),
                        ewebsock::WsMessage::Ping(vec) => todo!(),
                        ewebsock::WsMessage::Pong(vec) => todo!(),
                    }
                },
                ewebsock::WsEvent::Error(_) => {
                    let options = ewebsock::Options::default();
                    let (mut sender, receiver) = ewebsock::connect("ws://192.168.1.6:8080", options).unwrap();
                    
                    let request_json = serde_json::to_string(&SendMessage {
                        level: "Operation".to_string(),
                        method: "Initial".to_string(),
                        data: Some(json!({"content": "Hello from button('Send Message')!"})),
                        staff_credential: self.staff.clone(),
                        action: None
                    }).unwrap();
                    sender.send(ewebsock::WsMessage::Text(request_json));

                    self.sender = sender;
                    self.receiver = receiver;
                },
                ewebsock::WsEvent::Closed => {

                },
            }
        }
    }
}