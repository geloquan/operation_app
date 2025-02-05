use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use tokio::sync::mpsc::{Receiver, Sender};

use crate::{views::View, widget::login::Login};

use super::{MiddlemanToServer, MiddlemanToUi, ServerToMiddleman, UiToMiddleman};

pub(crate) struct Middleman {
    middleman_receiver_ui: Receiver<UiToMiddleman>,
    middleman_receiver_server: Receiver<ServerToMiddleman>,
    middleman_sender_ui: Sender<MiddlemanToUi>,
    middleman_sender_server: Sender<MiddlemanToServer>,
    data: std::sync::Arc<std::sync::RwLock<crate::models::StreamDatabase>>,
    stop_flag: Arc<AtomicBool>, 
}

impl Middleman {
    pub fn new(
        middleman_receiver_ui: Receiver<UiToMiddleman>,
        middleman_receiver_server: Receiver<ServerToMiddleman>,
        middleman_sender_ui: Sender<MiddlemanToUi>,
        middleman_sender_server: Sender<MiddlemanToServer>,
        data: std::sync::Arc<std::sync::RwLock<crate::models::StreamDatabase>>,
        stop_flag: Arc<AtomicBool>,
    ) -> Self {
        Self {
            middleman_receiver_ui,
            middleman_receiver_server,
            middleman_sender_ui,
            middleman_sender_server,
            data,
            stop_flag,
        }
    }
    pub async fn serve(&mut self) {
        println!("Middleman serving...");

        while !self.stop_flag.load(Ordering::Relaxed) {
            self.app_socket().await;
            self.server_socket().await;
            tokio::task::yield_now().await; // Allow other tasks to run
        }
    }
    async fn app_socket(&mut self) {
        while let Ok(msg) = self.middleman_receiver_ui.try_recv() {
            println!("middleman_thread got msg");
            match msg {
                UiToMiddleman::LoginAuthentication(login) => {
                    let _ = self.middleman_sender_server.send(MiddlemanToServer::LoginAuthentication(login)).await;
                },
            }
        }
    }
    async fn server_socket(&mut self) {
        while let Ok(msg) = &self.middleman_receiver_server.try_recv() {
            match msg {
                ServerToMiddleman::LoginAuthentication(session_token) => {
                    match session_token {
                        Ok(session_token) => {
                            let mut data = self.data.write().unwrap();
                            data.new_session_token(session_token);
                            data.new_app_state(View::OperationSelect);
                        },
                        Err(e) => {
                            
                        },
                    }
                },
                ServerToMiddleman::OperationList(operation_list) => {
                    match operation_list {
                        Ok(operation_list) => {
                            let mut data = self.data.write().unwrap();
                            data.new_operation_list(operation_list.clone());
                        },
                        Err(e) => {
                            
                        },
                    }
                },
                ServerToMiddleman::OperationData => {
                    
                },
            }
        }
    }
    pub fn app_state(&self) -> View {
        self.data.read().unwrap().get_app_state()
    }
    fn pre_login(&mut self, login: Login) {

    }
    fn post_login(&mut self, success: bool) {
        
    }
    //fn new_session(&mut self, )
}