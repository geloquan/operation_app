mod database;
use database::table::{
    builder::BuildTable, data::TableData, join::structure::OperationSelect, query, private::StaffAuthGrant
};

pub mod application;
use application::{authenticate::StaffCredential, field};
use application::{states, RunningApp};

pub mod ws;
use egui::{Frame, epaint, Color32};
use ws::receive::{
    Handle
};

pub mod temporary;
use temporary::*;

pub mod cipher;
use cipher::{decrypt_message, generate_fixed_key, EncryptedText};

pub mod component;
use component::design;

use chrono::{Local};
use eframe::{egui, App};
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
    staff: Option<StaffCredential>,
    //central_window: OperationWindow,
    state: Option<RunningApp>,
    temp: Option<Temporary>,
    credential_panel: states::Login
}

impl OperationApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(32);
        
        let options = ewebsock::Options::default();
        let (mut sender, receiver) = ewebsock::connect("ws://127.0.0.15:8080", options).unwrap();

        let request_json = serde_json::to_string(&SendMessage {
            level: "Operation".to_string(),
            method: "Initial".to_string(),
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
            staff: None,
            state: None,
            temp: None,
            credential_panel: states::Login {
                field: field::Login {
                    email: "".to_string(),
                    password: "".to_string(),
                    session_token: "".to_string()
                },
                state: design::State::Default,
            }
        }
    }
}

impl App for OperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_incoming();

        egui::SidePanel::left("left").show(ctx, |ui| {
            let margin = 20.0;
            ui.set_min_width(250.0);
            if self.staff.is_some() {
                ui.add_space(margin);
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    if let Some(operation) = self.get_operation() {
                        ui.heading("OPERATION: "); 
                        ui.label(operation.operation_label);
                        ui.heading("STATUS: "); 
                        ui.label(operation.operation_status);
                        ui.heading("ROOM: ");
                        ui.label(operation.room);
                        ui.heading("ROOM ALIAS: ");
                        ui.label(operation.room_code);
                    }
                    ui.label("🔎 SEARCH OPERATION");
                    if ui.text_edit_singleline(&mut self.search.search_operation).changed() {
                        &self.filter_operation();
                    }
    
                    ui.separator();
    
                    if self.search.search_operation_result.is_empty() && self.search.search_operation != "" {
                        ui.label("💤 No results found");
                    } else {
                        if let Some(data) = &mut self.data { 
                            if !self.search.search_operation_result.is_empty() {
                                TableData::build_table(ui, database::table::window::WindowTable::OperationSelect(Some(self.search.search_operation_result.clone())), data);
                            }
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
            }
            // Bottom section
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(margin);
                ui.heading("system by geloquan ");
                ui.separator();
                
                let current_time = Local::now(); 
                let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();

                ui.label(format!("Current Time: {}", formatted_time));
                if let Some(staff_credential) = &self.staff {
                    if ui.button("logout").clicked() {
                    }
                    ui.horizontal(|ui| {
                        ui.label("name");
                    });
                    ui.horizontal(|ui| {
                        ui.label("email");
                    });
                } else {
                    if ui.button("login").clicked() {
                        self.credential_panel.state = design::State::Waiting;
                        let request_json = serde_json::to_string(&SendMessage {
                            level: "Operation".to_string(),
                            method: "Authenticate".to_string(),
                            data: Some(serde_json::to_value(&self.credential_panel.field).unwrap())
                        }).unwrap();
                        self.sender.send(ewebsock::WsMessage::Text(request_json.to_string()));
                    }
                    let color: Color32 = match &self.credential_panel.state {
                        design::State::Waiting => {Color32::from_hex("#FFA652").unwrap()},
                        design::State::Error => {Color32::RED},
                        design::State::Valid => {Color32::GREEN},
                        design::State::Default => {Color32::TRANSPARENT},
                    };
                    ui.horizontal(|ui| {
                        ui.label("password ");
                        design::input(ui, &mut self.credential_panel.field.password, color, design::Category::Frame);
                    });
                    ui.horizontal(|ui| {
                        ui.label("email ");
                        design::input(ui, &mut self.credential_panel.field.email, color, design::Category::Frame);
                    });
                }
            });
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