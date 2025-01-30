use egui::debug_text::print;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{models::Config, DataMessage};


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
}

pub(crate) enum Get {
    Operation
}

impl Middleman {
    pub fn new(
        receiver: Receiver<Get>, 
        ui_sender: Sender<super::app::Get>, 
        server_sender: Sender<super::server::Get>, 
        data: std::sync::Arc<std::sync::RwLock<crate::models::StreamDatabase>>
    ) -> Self {
        Self {
            receiver,
            ui_sender,
            server_sender,
            data
        }
    }
    pub async fn serve(&mut self) {
        println!("middleman_thread");
        loop {
            while let Ok(msg) = self.receiver.try_recv() {
                println!("middleman_thread got msg");
                match msg {
                    Get::Operation => {
                        let operation_model = crate::models::Model::get_operation(Config::default(), &self.data);
                        println!("operation model: {:?}", operation_model);
                    },
                }
            }
        }
    }
}