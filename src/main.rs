mod database;
use database::table::{
    builder::BuildTable, data::TableData, join::structure::OperationSelect, query
};


pub mod application;
use application::RunningApp;

pub mod ws;
use ws::receive::*;

use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};
use eframe::{egui, App, Frame};
use egui::{mutex::Mutex, Color32, Label, RichText, Sense};
use egui_extras::{TableBuilder, Column};
use ewebsock::{self, WsReceiver, WsSender};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Debug, Serialize)]
struct SendMessage {
    level: String,
    method: String,
    data: Option<serde_json::Value>,
}
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum DatabaseTable {
    All,
    Equipment,
    Room,
    Tool,
    Staff,
    ToolReservation,
    ToolDesignatedRoom,
    ToolInspector,
    Patient,
    Operation,
    PatientWardRoom,
    PatientWardAssistant,
    OperationStaff,
    OperationTool
}
#[derive(Deserialize, Debug, Serialize, Default)]
struct PreRunning {
    search_operation: String,
    search_operation_result: Vec<OperationSelect>,
} 

struct OperationApp {
    data: Option<TableData>,
    rx: tokio::sync::mpsc::Receiver<String>,
    tx: tokio::sync::mpsc::Sender<String>,
    sender: WsSender,
    receiver: WsReceiver,
    search: PreRunning,
    //central_window: OperationWindow,
    state: Option<RunningApp>
}

impl OperationApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(32);
        
        let options = ewebsock::Options::default();
        let (mut sender, receiver) = ewebsock::connect("ws://127.0.0.15:8080", options).unwrap();

        let request_json = serde_json::to_string(&SendMessage {
            level: "operation".to_string(),
            method: "initial".to_string(),
            data: Some(json!({"content": "Hello from button('Send Message')!"})),
        }).unwrap();
        sender.send(ewebsock::WsMessage::Text(request_json));

        OperationApp {
            data: None,
            rx,
            tx,
            sender,
            receiver,
            search: PreRunning::default(),
            state: None
        }
    }
    pub fn search(&self) {

    }
}

impl App for OperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(msg) = self.receiver.try_recv() {
            match msg {
                ewebsock::WsEvent::Opened => {
                    
                },
                ewebsock::WsEvent::Message(text) => {
                    match text {
                        ewebsock::WsMessage::Binary(vec) => todo!(),
                        ewebsock::WsMessage::Text(text) => {
                            println!("text: {:?}", text);
                            match serde_json::from_str::<ReceiveMessage>(&text) {
                                Ok(message) => {
                                    println!("message: {:?}", message);
                                    match message.operation {
                                        Operation::Initialize => {
                                            if let Some(data) = &mut self.data {
                                                data.initialize(message.data);
                                            } else {
                                                let mut new_table_data = TableData::new();
                                                new_table_data.initialize(message.data);
                                                self.data = Some(new_table_data);
                                                println!("self.data: {:?}", self.data);
                                            }
                                        },
                                        Operation::Update => {},
                                    }
                                },
                                Err(_) => {
                                    println!("err parsing: ReceiveMessage");
                                },
                            }
                        },
                        ewebsock::WsMessage::Unknown(_) => todo!(),
                        ewebsock::WsMessage::Ping(vec) => todo!(),
                        ewebsock::WsMessage::Pong(vec) => todo!(),
                    }
                },
                ewebsock::WsEvent::Error(_) => {
                    let options = ewebsock::Options::default();
                    let (mut sender, receiver) = ewebsock::connect("ws://127.0.0.15:8080", options).unwrap();
                    
                    let request_json = serde_json::to_string(&SendMessage {
                        level: "operation".to_string(),
                        method: "initial".to_string(),
                        data: Some(json!({"content": "Hello from button('Send Message')!"})),
                    }).unwrap();
                    sender.send(ewebsock::WsMessage::Text(request_json));

                    self.sender = sender;
                    self.receiver = receiver;
                },
                ewebsock::WsEvent::Closed => {

                },
            }
        }

        egui::SidePanel::left("left").show(ctx, |ui| {
            if let Some(operation) = self.get_operation() {
                ui.heading("OPERATION: "); 
                ui.label(operation.operation_label);
                ui.heading("STATUS: "); 
                ui.label(operation.operation_status);
                ui.heading("ETA: "); 
                ui.heading("ROOM: ");
                ui.label(operation.room);
                ui.heading("ROOM ALIAS: ");
                ui.label(operation.room_code);
            }
            ui.label("ENTER OPERATION");
            if ui.text_edit_singleline(&mut self.search.search_operation).changed() {
                println!("trying to search: {:?}", self.search.search_operation);
                &self.filter_operation();
                //if let Some(operation_result) =  {
                //    &self.search.search_operation_result = operation_result.clone();
                //}
            }

            ui.separator();

            if self.search.search_operation_result.is_empty() {
                ui.label("No results found");
            } else {
                ui.label("Results:");
                if let Some(data) = &mut self.data { 
                    TableData::build_table(ui, database::table::window::WindowTable::OperationSelect(Some(self.search.search_operation_result.clone())), data);
                }
            }

            if ui.button("send alert").clicked() {
                let request_json = serde_json::to_string(&SendMessage {
                    level: "operation".to_string(),
                    method: "alert".to_string(),
                    data: Some(json!({"content": "Hello from button('Send Message')!"})),
                }).unwrap();
                self.sender.send(ewebsock::WsMessage::Text(request_json));
            }
        });
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.label("Hello World!");
        });
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("OPERATION APP", native_options, Box::new(|cc| {
        let app = OperationApp::new(cc);
        Ok(Box::new(app))
    }));
}