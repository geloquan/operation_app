use eframe::glow::MAX_SHADER_STORAGE_BLOCK_SIZE;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::protocol::frame::coding::Data;

use crate::DataMessage;

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
struct ServerExchangeFormat {
    
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
        println!("server_thread");
        loop {
            self.cloud_socket();
            self.server_socket();
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
    
    fn server_socket(&mut self) {
        while let Ok(msg) = self.server_receiver.try_recv() {
        }
    }
}