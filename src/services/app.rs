use tokio::sync::mpsc::{Receiver, Sender};
use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use super::middleman;

pub(crate) enum Get {
    Operation
}
pub(crate) struct App {
    receiver: Receiver<Get>, 
    middleman_sender: Sender<super::middleman::Get>, 
    data: Arc<RwLock<crate::models::StreamDatabase>>,
}
impl App {
    pub fn new(
        receiver: Receiver<Get>, 
        middleman_sender: Sender<super::middleman::Get>, 
        data: Arc<RwLock<crate::models::StreamDatabase>>,
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
    pub fn send(&self, middleman_sender: super::middleman::Get) {
        let clonee: Sender<crate::Get> = self.middleman_sender.clone();
        tokio::spawn(async move {
            let _ = clonee.send(middleman_sender).await;
        });
    }
}