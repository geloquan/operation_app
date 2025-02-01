
use std::{cell::RefCell, rc::Rc, sync::{atomic::AtomicBool, Arc, RwLock}, thread, time::Duration};

use egui::debug_text::print;
use ewebsock::WsMessage;
use tokio::{sync::mpsc::{self, Receiver, Sender}, task::JoinHandle};

use crate::{widget::login, StreamDatabase};

mod server;
pub(crate) mod app;
pub mod middleman;

pub(crate) enum Node {
    Server,
    App,
    Middleman
}


trait Init {
    fn init(message: std::sync::Arc<std::sync::RwLock<super::models::StreamDatabase>>) -> Result<Self, &'static str> where Self: Sized;
}


pub(crate) struct Service {
    pub server: Rc<RefCell<JoinHandle<()>>>,
    pub app: Rc<RefCell<app::App>>,
    pub middleman: Rc<RefCell<JoinHandle<()>>>
}

pub(crate) enum MiddlemanToUi {
}
pub(crate) enum MiddlemanToServer {
    LoginAuthentication(login::Login)
}
pub(crate) enum UiToMiddleman {
    LoginAuthentication(login::Login)
}
pub(crate) enum ServerToMiddleman {
    LoginAuthentication(bool)
}


impl Service {
    pub async fn init(message: Arc<std::sync::RwLock<super::models::StreamDatabase>>) -> Result<Self, &'static str> {
        let (middleman_sender_ui, ui_receiver_middleman): (Sender<MiddlemanToUi>, Receiver<MiddlemanToUi>) = mpsc::channel(32);
        let (ui_sender_middleman, middleman_receiver_ui): (Sender<UiToMiddleman>, Receiver<UiToMiddleman>) = mpsc::channel(32);
        let (server_sender_middleman, middleman_receiver_server): (Sender<ServerToMiddleman>, Receiver<ServerToMiddleman>) = mpsc::channel(32);
        let (middleman_sender_server, server_receiver_middleman): (Sender<MiddlemanToServer>, Receiver<MiddlemanToServer>) = mpsc::channel(32);
        let (cloud_sender, cloud_receiver) = ewebsock::connect("ws://192.168.1.9:8080", ewebsock::Options::default()).unwrap();
    
    
        let middleman_message = Arc::clone(&message);
        let middleman_thread = tokio::spawn(async move {
            let data: Arc<std::sync::RwLock<super::models::StreamDatabase>> = middleman_message;

            let middleman_receiver_ui = middleman_receiver_ui;
            let middleman_receiver_server = middleman_receiver_server;
            let middleman_sender_ui = middleman_sender_ui;
            let middleman_sender_server = middleman_sender_server;

            let mut middleman = middleman::Middleman::new(
                middleman_receiver_ui,
                middleman_receiver_server,
                middleman_sender_ui,
                middleman_sender_server,
                data,
                Arc::new(AtomicBool::new(false))
            );
            middleman.serve().await;

        });
    
        let server_thread = tokio::spawn(async move {
            let server_sender_middleman = server_sender_middleman;
            let server_receiver_middleman = server_receiver_middleman;
            server::Server::new(cloud_receiver, cloud_sender, server_receiver_middleman, server_sender_middleman, Arc::new(AtomicBool::new(false))).serve().await;
        });
        
        let ui_message = Arc::clone(&message);
        let data: Arc<RwLock<StreamDatabase>> = ui_message;
        let app = Rc::new(RefCell::new(
            app::App::new(ui_receiver_middleman, ui_sender_middleman, data)
        ));
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