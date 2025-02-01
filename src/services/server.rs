use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use eframe::glow::MAX_SHADER_STORAGE_BLOCK_SIZE;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::protocol::frame::coding::Data;

use super::{middleman, MiddlemanToServer, ServerToMiddleman};

pub(crate) enum Get {
    Operation,
    LoginAuthentication
}

pub(crate) struct Server {
    receiver: ewebsock::WsReceiver,
    sender: ewebsock::WsSender,
    server_receiver_middleman: Receiver<MiddlemanToServer>,
    server_sender_middleman: Sender<ServerToMiddleman>,
    stop_flag: Arc<AtomicBool>, 
}
enum Method {
    Crate,
    Read,
    Update,
    Delete
}
struct ServerExchangeFormat<'a> {
    request: bool,
    method: Method,
    metadata: &'a str
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
                    let _ = self.server_sender_middleman.send(ServerToMiddleman::LoginAuthentication(true)).await;
                },
            }
        }
    }
}