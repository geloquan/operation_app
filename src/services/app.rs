use tokio::sync::mpsc::{Receiver, Sender};
use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}};

use crate::views::View;

use super::{middleman, MiddlemanToUi, UiToMiddleman};

pub(crate) struct App {
    ui_receiver_middleman: Receiver<MiddlemanToUi>, 
    ui_sender_middleman: Sender<UiToMiddleman>, 
    data: Arc<RwLock<crate::models::StreamDatabase>>,
}
impl App {
    pub fn new(
        ui_receiver_middleman: Receiver<MiddlemanToUi>, 
        ui_sender_middleman: Sender<UiToMiddleman>, 
        data: Arc<RwLock<crate::models::StreamDatabase>>,
    ) -> Self {
        Self {
            ui_receiver_middleman,
            ui_sender_middleman,
            data
        }
    }
    pub async fn serve(&mut self) {
        loop {
            while let Ok(msg) = self.ui_receiver_middleman.try_recv() {
                match msg {
                }
            }
        }
    }
    pub fn send(&self, msg: UiToMiddleman) {
        let middleman_sender: Sender<UiToMiddleman> = self.ui_sender_middleman.clone();
        tokio::spawn(async move {
            let _ = middleman_sender.send(msg).await;
        });
    }
    pub fn get_app_state(&self) -> View {
        self.data.read().unwrap().get_app_state()
    }
}