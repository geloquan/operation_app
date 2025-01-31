
use std::{cell::RefCell, rc::Rc, sync::{atomic::AtomicBool, Arc}, thread, time::Duration};

use egui::debug_text::print;
use ewebsock::WsMessage;
use tokio::{sync::mpsc::{self, Receiver, Sender}, task::JoinHandle};

mod server;
pub(crate) mod app;
pub mod middleman;


trait Init {
    fn init(message: std::sync::Arc<std::sync::RwLock<super::models::StreamDatabase>>) -> Result<Self, &'static str> where Self: Sized;
}


pub(crate) struct Service {
    pub server: Rc<RefCell<JoinHandle<()>>>,
    pub app: Rc<RefCell<app::App>>,
    pub middleman: Rc<RefCell<JoinHandle<()>>>
}

impl Service {
    pub async fn init(message: Arc<std::sync::RwLock<super::models::StreamDatabase>>) -> Result<Self, &'static str> {
        let (ui_sender, ui_receiver): (Sender<app::Get>, Receiver<app::Get>) = mpsc::channel(32);
        let (middleman_sender, middleman_receiver): (Sender<middleman::Get>, Receiver<middleman::Get>) = mpsc::channel(32);
        let (server_sender, server_receiver): (Sender<server::Get>, Receiver<server::Get>) = mpsc::channel(32);
        let (cloud_sender, cloud_receiver) = ewebsock::connect("ws://192.168.1.9:8080", ewebsock::Options::default()).unwrap();
    
    
        let middleman_message = Arc::clone(&message);
        let ui_sender_1 = ui_sender.clone();
        let middleman_thread = tokio::spawn(async move {
            let data: Arc<std::sync::RwLock<super::models::StreamDatabase>> = middleman_message;

            let mut man = middleman::Middleman::new(middleman_receiver, ui_sender_1, server_sender, data, Arc::new(AtomicBool::new(false)));
            man.serve().await;

            println!("middleman_thread ended");
        });
    
        let middleman_sender_2 = middleman_sender.clone();
        let server_thread = tokio::spawn(async move {
            server::Server::new(cloud_receiver, cloud_sender, server_receiver, middleman_sender_2, Arc::new(AtomicBool::new(false))).serve().await;
            println!("server_thread ended");
        });
        
        let ui_message = Arc::clone(&message);
        let data: Arc<std::sync::RwLock<super::models::StreamDatabase>> = ui_message;
        let app = Rc::new(RefCell::new(app::App::new(ui_receiver, middleman_sender, data)));
        let server = Rc::new(RefCell::new(server_thread));
        let middleman = Rc::new(RefCell::new(middleman_thread));
        
        Ok(
            Self {
                app,
                server,
                middleman
            }
        )
    }
}