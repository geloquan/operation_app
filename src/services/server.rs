use eframe::glow::MAX_SHADER_STORAGE_BLOCK_SIZE;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::protocol::frame::coding::Data;

use super::middleman;

pub(crate) enum Get {
    Operation
}

pub(crate) struct Server {
    receiver: ewebsock::WsReceiver,
    sender: ewebsock::WsSender,
    server_receiver: Receiver<super::server::Get>,
    middleman_sender: Sender<middleman::Get>
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
    pub fn new(receiver: ewebsock::WsReceiver, sender: ewebsock::WsSender, server_receiver: Receiver<super::server::Get>, middleman_sender: Sender<middleman::Get>) -> Self {
        Self {
            receiver,
            sender,
            server_receiver,
            middleman_sender
        }
    }
    
    pub async fn serve(&mut self) {
        println!("Server serving...");
        loop {
            self.cloud_socket();
            self.server_socket();
        }
    }

    fn cloud_socket(&mut self) {
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
    
    fn server_socket(&mut self) {
        while let Ok(msg) = self.server_receiver.try_recv() {
        }
    }
}