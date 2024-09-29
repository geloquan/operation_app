mod database;
use database::table::{
    data::TableData,
    query
};


pub mod application;
use application::RunningApp;


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
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum Operation {
    Initialize,
    Update
}
#[derive(Deserialize, Debug, Serialize)]
struct ReceiveMessage {
    table_name: DatabaseTable,
    operation: Operation,
    status_code: String,
    data: String,
}


struct OperationApp {
    rx: tokio::sync::mpsc::Receiver<String>,
    tx: tokio::sync::mpsc::Sender<String>,
    sender: WsSender,
    receiver: WsReceiver,
    search_operation: String,
    search_operation_result: Vec<String>,
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
            rx,
            tx,
            sender,
            receiver,
            search_operation: "".to_string(),
            search_operation_result: vec![],
            state: None
        }
    }
    
}

impl App for OperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(msg) = self.receiver.try_recv() {
            match msg {
                ewebsock::WsEvent::Opened => {
                    
                },
                ewebsock::WsEvent::Message(text) => {
                    println!("msg receieved: {:?}", text);
                    match text {
                        ewebsock::WsMessage::Binary(vec) => todo!(),
                        ewebsock::WsMessage::Text(text) => {
                            match serde_json::from_str::<ReceiveMessage>(&text) {
                                Ok(message) => {
                                    match message.operation {
                                        Operation::Initialize => {},
                                        Operation::Update => {},
                                    }
                                },
                                Err(_) => {
                                    
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
            if let Some(state) = &self.state {
                if let Some(operation) = state.get_operation() {
                    
                }
            } else {
                ui.label("ENTER OPERATION");
                if ui.text_edit_singleline(&mut self.search_operation).changed() {
                    println!("trying to search: {:?}", self.search_operation);
                }

                ui.separator();
                if self.search_operation_result.is_empty() {
                    ui.label("No results found");
                } else {
                    ui.label("Results:");
                    for row in &self.search_operation_result {
                        ui.label(row);  
                    }
                }
            }
            ui.label("OPERATION: "); //op.label
            ui.label("STATUS: "); //op.status
            ui.label("ETA: "); //op.start -> op.end
            ui.label("ROOM: ");
            ui.label("ROOM ALIAS: ");

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