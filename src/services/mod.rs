
use std::{sync::Arc, thread, time::Duration};

use egui::debug_text::print;
use ewebsock::WsMessage;
use tokio::sync::mpsc::{self, Sender, Receiver};

mod server;
mod app;
mod middleman;


trait Init {
    fn init(message: std::sync::Arc<std::sync::RwLock<crate::DataMessage>>) -> Result<Self, &'static str> where Self: Sized;
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
    pub async fn init(message: std::sync::Arc<std::sync::RwLock<crate::DataMessage>>) -> Result<(), &'static str> {
        let (ui_sender, mut ui_receiver): (Sender<crate::DataMessage>, Receiver<crate::DataMessage>) = mpsc::channel(32);
        let (middleman_sender, mut middleman_receiver): (Sender<crate::DataMessage>, Receiver<crate::DataMessage>) = mpsc::channel(32);
        let (server_sender, mut server_receiver): (Sender<crate::DataMessage>, Receiver<crate::DataMessage>) = mpsc::channel(32);
        let (mut cloud_sender, cloud_receiver) = ewebsock::connect("ws://192.168.1.9:8080", ewebsock::Options::default()).unwrap();
    
        let middleman_sender_1 = middleman_sender.clone();
        let ui_message = Arc::clone(&message);
        let ui_thread = tokio::spawn(async move {
            let data: Arc<std::sync::RwLock<crate::DataMessage>> = ui_message;
            let middleman_sender = middleman_sender_1;

            loop {
                while let Ok(message) = ui_receiver.try_recv() {
                    println!("ui_receiver got: {:?}", message);
                    
                }
            }
        });
    
        let middleman_message = Arc::clone(&message);
        let middleman_thread = tokio::spawn(async move {
            let data: Arc<std::sync::RwLock<crate::DataMessage>> = middleman_message;

            println!("middleman_thread");
            loop {
                while let Ok(message) = middleman_receiver.try_recv() {
                    println!("middleman_receiver got: {:?}", message);
                    let mut data = data.write().unwrap();
                    *data = crate::DataMessage{ message: message.message.to_owned() };
                }

            }
        });
    
        let middleman_sender_2 = middleman_sender.clone();
        let server_thread = tokio::spawn(async move {
            println!("server_thread");
            server::Server::new(cloud_receiver, cloud_sender, server_receiver, middleman_sender_2).serve();
        });

        thread::sleep(Duration::from_millis(1_000));
        middleman_sender
            .send(crate::DataMessage {
                message: "Hello from main".to_string(),
            }).await;
        
        Ok(())
    }
}