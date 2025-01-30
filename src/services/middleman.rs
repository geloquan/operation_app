use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use egui::debug_text::print;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{models::Config};


//enum Table {
    //    FruitQueried
    //}
    //enum Get {
        //    Table
        //}
        //enum Crate {
            //    Table
            //}
            //enum Delete {
                //    Table
                //}
pub(crate) struct Middleman {
    receiver: Receiver<Get>, 
    ui_sender: Sender<super::app::Get>, 
    server_sender: Sender<super::server::Get>, 
    data: std::sync::Arc<std::sync::RwLock<crate::models::StreamDatabase>>,
    stop_flag: Arc<AtomicBool>, 
}

pub(crate) enum Get {
    Operation
}

impl Middleman {
    pub fn new(
        receiver: Receiver<Get>, 
        ui_sender: Sender<super::app::Get>, 
        server_sender: Sender<super::server::Get>, 
        data: std::sync::Arc<std::sync::RwLock<crate::models::StreamDatabase>>,
        stop_flag: Arc<AtomicBool>, 
    ) -> Self {
        Self {
            receiver,
            ui_sender,
            server_sender,
            data,
            stop_flag 
        }
    }
    pub async fn serve(&mut self) {
        println!("Middleman serving...");
        
        while !self.stop_flag.load(Ordering::Relaxed) {
            while let Ok(msg) = self.receiver.try_recv() {
                println!("middleman_thread got msg");
                match msg {
                    Get::Operation => {
                        let operation_model = crate::models::Model::get_operation(Config::default(), &self.data);
                        println!("operation model: {:?}", operation_model);
                    },
                }
            }
            tokio::task::yield_now().await; // Allow other tasks to run
        }
    }
}