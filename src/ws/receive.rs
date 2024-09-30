
use ewebsock::WsReceiver;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{application::authenticate::StaffCredential, cipher::{decrypt_message, generate_fixed_key, EncryptedText}, database::{self, table::{self, data::TableData}}, OperationApp, SendMessage};

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
    OperationTool
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
    fn handle_incoming(&mut self) {
        if let Some(msg) = self.receiver.try_recv() {
            match msg {
                ewebsock::WsEvent::Opened => {
                    
                },
                ewebsock::WsEvent::Message(text) => {
                    match text {
                        ewebsock::WsMessage::Binary(vec) => todo!(),
                        ewebsock::WsMessage::Text(text) => {
                            match serde_json::from_str::<EncryptedText>(&text) {
                                Ok(encrypted_text) => {
                                    if let Ok(key) = &generate_fixed_key() {
                                        if let Ok(decrypted_text) = decrypt_message(key, &encrypted_text.nonce, &encrypted_text.cipher_text) {
                                            match serde_json::from_str::<ReceiveMessage>(&decrypted_text) {
                                                Ok(message) => {
                                                    match message.operation {
                                                        Operation::Initialize => {
                                                            if let Some(data) = &mut self.data {
                                                                data.initialize(message.data);
                                                            } else {
                                                                let mut new_table_data = TableData::new();
                                                                new_table_data.initialize(message.data);
                                                                self.data = Some(new_table_data);
                                                            }
                                                        },
                                                        Operation::Update => {},
                                                        Operation::AuthHandshake => {
                                                            if self.staff.is_none() {
                                                                println!("statuscode {:?}", message.status_code);
                                                                //match serde_json::from_str::<table::private::StaffAuthGrant>(&message.data) {
                                                                //    Ok(staff) => {
                                                                //        self.staff = Some(StaffCredential {
                                                                //            id: staff.id.unwrap_or_default(),
                                                                //            email: staff.email.unwrap_or_default(),
                                                                //            full_name: staff.full_name.unwrap_or_default(),
                                                                //            session_key: staff.session_token.unwrap_or_default(),
                                                                //        })
                                                                //    },
                                                                //    Err(_) => {
                                                                //        
                                                                //    },
                                                                //} 
                                                            }
                                                        }
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
                    let (mut sender, receiver) = ewebsock::connect("ws://127.0.0.15:8080", options).unwrap();
                    
                    let request_json = serde_json::to_string(&SendMessage {
                        level: "Operation".to_string(),
                        method: "Initial".to_string(),
                        data: Some(json!({"content": "Hello from button('Send Message')!"})),
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