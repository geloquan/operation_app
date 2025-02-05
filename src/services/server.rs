use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use eframe::glow::MAX_SHADER_STORAGE_BLOCK_SIZE;
use serde::Serialize;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::protocol::frame::coding::Data;

use crate::{models::exchange_format::{self, Method, ServerExchangeFormat, SessionToken}, widget::login::Login};

use super::{middleman, MiddlemanToServer, ServerToMiddleman};

pub(crate) struct Server {
    receiver: ewebsock::WsReceiver,
    sender: ewebsock::WsSender,
    server_receiver_middleman: Receiver<MiddlemanToServer>,
    server_sender_middleman: Sender<ServerToMiddleman>,
    stop_flag: Arc<AtomicBool>, 
}
impl Server {
    pub fn new(
        receiver: ewebsock::WsReceiver, 
        sender: ewebsock::WsSender, 
        server_receiver_middleman: Receiver<MiddlemanToServer>,
        server_sender_middleman: Sender<ServerToMiddleman>,
        stop_flag: Arc<AtomicBool>
    ) -> Self {
        Self {
            receiver,
            sender,
            server_receiver_middleman,
            server_sender_middleman,
            stop_flag
        }
    }
    
    pub async fn serve(&mut self) {
        println!("Server serving...");
        while !self.stop_flag.load(Ordering::Relaxed) {
            self.cloud_socket().await;
            self.server_socket().await;
            tokio::task::yield_now().await;
        }
    }

    async fn cloud_socket(&mut self) {
        while let Some(msg) = self.receiver.try_recv() {
            match msg {
                ewebsock::WsEvent::Opened => {
                    println!("Connected to Server");
                },
                ewebsock::WsEvent::Message(message) => {
                    match message {
                        ewebsock::WsMessage::Binary(items) => todo!(),
                        ewebsock::WsMessage::Text(msg) => {
                            let exchange_format = serde_json::from_str::<ServerExchangeFormat>(&msg).unwrap();
                            println!("exchange_format: {:?}", exchange_format);
                            match exchange_format.method {
                                Method::CheckUser => {
                                    let _ = self.server_sender_middleman.send(
                                        ServerToMiddleman::LoginAuthentication(
                                            Ok(serde_json::from_str::<SessionToken>(
                                                &exchange_format.metadata).unwrap()
                                            ))
                                        ).await;
                                },
                            }
                        },
                        ewebsock::WsMessage::Unknown(_) => todo!(),
                        ewebsock::WsMessage::Ping(items) => todo!(),
                        ewebsock::WsMessage::Pong(items) => todo!(),
                    }
                },
                ewebsock::WsEvent::Error(error) => {
                    
                },
                ewebsock::WsEvent::Closed => {
                    println!("Disconnect to Server");
                },
            }
        }
    }
    
    async fn server_socket(&mut self) {
        while let Ok(msg) = self.server_receiver_middleman.try_recv() {
            println!("server_socket received a message...");
            match msg {
                MiddlemanToServer::LoginAuthentication(login) => {
                    let exchange_format = ServerExchangeFormat {
                        metadata: serde_json::to_string(&login).unwrap(),
                        method: Method::CheckUser,
                        request: true
                    };

                    self.sender.send(ewebsock::WsMessage::Text(serde_json::to_string(&exchange_format).unwrap()));
                },
            }
        }
    }
}