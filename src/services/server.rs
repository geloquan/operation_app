use eframe::glow::MAX_SHADER_STORAGE_BLOCK_SIZE;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::protocol::frame::coding::Data;

use crate::DataMessage;

use super::middleman;

pub(crate) struct Server {
    receiver: ewebsock::WsReceiver,
    sender: ewebsock::WsSender,
    server_receiver: Receiver<DataMessage>,
    middleman_sender: Sender<DataMessage>
}
struct ServerExchangeFormat {
    
}
impl Server {
    pub fn new(receiver: ewebsock::WsReceiver, sender: ewebsock::WsSender, server_receiver: Receiver<DataMessage>, middleman_sender: Sender<DataMessage>) -> Self {
        Self {
            receiver,
            sender,
            server_receiver,
            middleman_sender
        }
    }
    pub fn serve(&mut self) {
        loop {
            self.cloud_socket();
            self.middleman_socket();
        }
    }
    fn cloud_socket(&mut self) {
        while let Some(msg) = self.receiver.try_recv() {
            println!("server_receiver got: {:?}", msg);
            match msg {
                ewebsock::WsEvent::Opened => {
                    
                },
                ewebsock::WsEvent::Message(message) => {

                },
                ewebsock::WsEvent::Error(error) => {
                    
                },
                ewebsock::WsEvent::Closed => {

                },
            }
        }
    }
    fn middleman_socket(&mut self) {
        while let Ok(msg) = self.server_receiver.try_recv() {
            println!("server_receiver got: {:?}", msg);
        }
    }
}

//#[derive(Deserialize, Debug, Serialize)]
//pub struct ReceiveMessage {
//    pub table_name: Option<TableTarget>,
//    pub operation: Operation,
//    pub action: Option<ActionLogLabel>,
//    pub status_code: String,
//    pub data: String,
//}
//
//impl super::Init for Server {
//    fn init(middleman: Sender<DataMessage>, server: ewebsock::WsReceiver) {
//
//        if let Some(msg) = server.try_recv() {
//            match msg {
//                ewebsock::WsEvent::Opened => {
//                    
//                },
//                ewebsock::WsEvent::Message(text) => {
//                    match text {
//                        ewebsock::WsMessage::Binary(vec) => todo!(),
//                        ewebsock::WsMessage::Text(text) => {
//                            let text_len = text.len();
//                            match serde_json::from_str::<EncryptedText>(&text) {
//                                Ok(encrypted_text) => {
//                                    if let Ok(key) = &generate_fixed_key() {
//                                        if let Ok(decrypted_text) = decrypt_message(key, &encrypted_text.nonce, &encrypted_text.cipher_text) {
//                                            match serde_json::from_str::<ReceiveMessage>(&decrypted_text) {
//                                                Ok(message) => {
//                                                    match message.operation {
//                                                        Operation::Initialize => {
//                                                            println!("initialize: {:?}", message);
//                                                            if let Some(data) = &mut self.data {
//                                                                data.initialize(message.data);
//                                                            } else {
//                                                                let mut new_table_data = TableData::new();
//                                                                new_table_data.initialize(message.data);
//                                                                self.data = Some(new_table_data);
//                                                            }
//                                                        },
//                                                        Operation::Update => {
//                                                            println!("update: {:?}", message);
//                                                            self.update(message);
//                                                        },
//                                                        Operation::AuthHandshake => {
//                                                            println!("statuscode {:?}", message.status_code);
//                                                            if let Ok(staff_credential) = serde_json::from_str::<StaffCredential>(&message.data) {
//                                                                self.staff = Some(staff_credential);
//                                                            } else {
//                                                                self.staff = None;
//                                                            }
//                                                            if message.status_code == "success" { 
//                                                                self.credential_panel.state = design::State::Valid
//                                                            }
//                                                            else if message.status_code == "failed" { 
//                                                                self.credential_panel.state = design::State::Error 
//                                                            }
//                                                        }
//                                                        Operation::Ascend => {
//                                                            println!("message: {:?}", message);
//                                                            if let Ok(operation_ascend) = serde_json::from_str::<private::OperationAscend>(&message.data) {
//                                                                self.require_update = true;
//                                                                if let (Some(operation_state), Some(operation_id)) = (&mut self.operation_state, &self.operation_id) {
//                                                                    if operation_ascend.operation_id == operation_id.to_owned() {
//                                                                        match operation_state {
//                                                                            crate::application::operation::State::Preoperation(_) => *operation_state = crate::application::operation::State::Intraoperation(
//                                                                                Menu {
//                                                                                    selected_action: None,
//                                                                                    selected_menu: None
//                                                                                }
//                                                                            ),
//                                                                            crate::application::operation::State::Intraoperation(_) => *operation_state = crate::application::operation::State::Postoperation,
//                                                                            crate::application::operation::State::Postoperation => todo!(),
//                                                                        } 
//                                                                    }
//                                                                }
//                                                            }
//                                                        },
//                                                    }
//                                                },
//                                                Err(_) => {
//                                                    println!("err parsing: ReceiveMessage");
//                                                },
//                                            }
//                                        }
//                                    }
//                                },
//                                Err(_) => {
//                                    
//                                },
//                            }
//                        },
//                        ewebsock::WsMessage::Unknown(_) => todo!(),
//                        ewebsock::WsMessage::Ping(vec) => todo!(),
//                        ewebsock::WsMessage::Pong(vec) => todo!(),
//                    }
//                },
//                ewebsock::WsEvent::Error(_) => {
//                    let options = ewebsock::Options::default();
//                    let (mut sender, receiver) = ewebsock::connect("ws://192.168.1.6:8080", options).unwrap();
//                    
//                    let request_json = serde_json::to_string(&SendMessage {
//                        level: "Operation".to_string(),
//                        method: "Initial".to_string(),
//                        data: Some(json!({"content": "Hello from button('Send Message')!"})),
//                        staff_credential: self.staff.clone(),
//                        action: None
//                    }).unwrap();
//                    sender.send(ewebsock::WsMessage::Text(request_json));
//
//                    self.sender = sender;
//                    self.receiver = receiver;
//                },
//                ewebsock::WsEvent::Closed => {
//
//                },
//            }
//        }
//    } 
//}