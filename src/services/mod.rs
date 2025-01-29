
use std::{sync::Arc, thread, time::Duration};

use egui::debug_text::print;
use ewebsock::WsMessage;
use tokio::sync::mpsc::{self, Sender, Receiver};

mod server;
mod app;
pub mod middleman;


trait Init {
    fn init(message: std::sync::Arc<std::sync::RwLock<super::models::StreamDatabase>>) -> Result<Self, &'static str> where Self: Sized;
}

pub(crate) struct App {
    receiver: tokio::sync::mpsc::Receiver<String>,
    sender: tokio::sync::mpsc::Sender<String>
}

pub(crate) struct Service {
    server: server::Server,
    app: App,
    middleman: App
}

impl Service {
    pub async fn init(message: Arc<std::sync::RwLock<super::models::StreamDatabase>>) -> Result<Sender<middleman::Get>, &'static str> {
        let (ui_sender, ui_receiver): (Sender<app::Get>, Receiver<app::Get>) = mpsc::channel(32);
        let (middleman_sender, middleman_receiver): (Sender<middleman::Get>, Receiver<middleman::Get>) = mpsc::channel(32);
        let (server_sender, server_receiver): (Sender<server::Get>, Receiver<server::Get>) = mpsc::channel(32);
        let (cloud_sender, cloud_receiver) = ewebsock::connect("ws://192.168.1.9:8080", ewebsock::Options::default()).unwrap();
    
        let middleman_sender_1 = middleman_sender.clone();
        let ui_message = Arc::clone(&message);
        let _ui_thread = tokio::spawn(async move {
            let data: Arc<std::sync::RwLock<super::models::StreamDatabase>> = ui_message;
            let middleman_sender = middleman_sender_1;
            
            let mut app = app::App::new(ui_receiver, middleman_sender, data);
            app.serve().await
        });
    
        let middleman_message = Arc::clone(&message);
        let ui_sender_1 = ui_sender.clone();
        let _middleman_thread = tokio::spawn(async move {
            let data: Arc<std::sync::RwLock<super::models::StreamDatabase>> = middleman_message;

            let mut man = middleman::Middleman::new(middleman_receiver, ui_sender_1, server_sender, data);
            man.serve().await
        });
    
        let middleman_sender_2 = middleman_sender.clone();
        let _server_thread = tokio::spawn(async move {
            server::Server::new(cloud_receiver, cloud_sender, server_receiver, middleman_sender_2).serve().await
        });
        //middleman_sender
        //    .send(super::models::StreamDatabase {
        //        message: "Hello from main".to_string(),
        //    }).await;
        
        Ok(middleman_sender.clone())
    }
}