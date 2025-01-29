use tokio::sync::mpsc::{Receiver, Sender};

use super::middleman;

pub(crate) enum Get {
    Operation
}
pub(crate) struct App {
    receiver: Receiver<Get>, 
    middleman_sender: Sender<super::middleman::Get>, 
    data: std::sync::Arc<std::sync::RwLock<crate::models::StreamDatabase>>,
}
impl App {
    pub fn new(
        receiver: Receiver<Get>, 
        middleman_sender: Sender<super::middleman::Get>, 
        data: std::sync::Arc<std::sync::RwLock<crate::models::StreamDatabase>>,
    ) -> Self {
        Self {
            receiver,
            middleman_sender,
            data
        }
    }
    pub async fn serve(&mut self) {
        loop {
            while let Ok(msg) = self.receiver.try_recv() {
                match msg {
                    Get::Operation => {
                    },
                }
            }
        }
    }
}