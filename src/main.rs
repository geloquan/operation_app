mod database;

use database::table::{
    ui_builder::BuildTable, data::TableData, join::structure::OperationSelect, query, private::StaffAuthGrant
};

pub mod application;
use application::{authenticate::StaffCredential, field};
use application::{states, RunningApp, component as app_component};

pub mod ws;
use egui::CursorIcon;
use egui::text::Fonts;
use egui::{epaint, Align, Align2, Area, Color32, Direction, FontId, Frame, Layout, Pos2, RichText, Stroke, TextEdit, Window};
use ws::receive::{
    Handle
};

pub mod temporary;
use temporary::*;

pub mod cipher;
use cipher::{decrypt_message, generate_fixed_key, EncryptedText};

pub mod component;
use component::design;

use application::component::format::get_width_from_text;

use chrono::{Local};
use eframe::{egui, App};
use egui_extras::{TableBuilder, Column};
use ewebsock::{self, WsReceiver, WsSender};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};

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

pub struct OperationApp {
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
    credential_panel: states::Login,
    category: states::Category,
    operation_id: Option<i32>,
    require_update: bool
}

impl OperationApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(32);
        
        let options = ewebsock::Options::default();
        let (sender, receiver) = ewebsock::connect("ws://127.0.0.15:8080", options).unwrap();

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
            },
            category: states::Category::default(),
            operation_id: None,
            require_update: false
        }
    }
}

impl App for OperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_incoming();

        if let Some(id) = self.operation_id {
            self.select_operation(&id);
        }

        if self.staff.is_none() {
            app_component::login(&ctx, &mut self.credential_panel, &mut self.sender);
        } else {
            let left_panel_rect: Pos2;
            egui::SidePanel::left("left").show(ctx, |ui| {
                let margin = 20.0;
                let rect = ui.min_rect();
                let left_panel_rect = rect.center();
                ui.set_max_width(250.0);

                if self.staff.is_some() {
                    ui.add_space(margin);
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        if let Some(operation) = self.get_selected_operation() {
                            ui.horizontal_wrapped(|ui| {
                                ui.heading("OPERATION: ");       
                                ui.add_enabled(false, 
                                TextEdit::singleline(&mut operation.op_label.to_string())
                                );
                                ui.heading("STATUS: "); 
                                ui.add_enabled(false, 
                                    TextEdit::singleline(&mut operation.op_status.to_string())
                                );
                                ui.heading("ROOM: ");
                                ui.add_enabled(false, 
                                    TextEdit::singleline(&mut operation.room_name.to_string())
                                );
                                ui.heading("PATIENT: ");
                                ui.add_enabled(false, 
                                    TextEdit::singleline(&mut operation.patient_full_name.to_string())
                                );
                            });
                            let mut tool_response = Vec::new();
                            let patient = ui.horizontal(|ui| {
                                tool_response.push(ui.label(RichText::new("⚒").size(60.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                ui.vertical(|ui| {
                                    tool_response.push(ui.heading(RichText::new("TOOLS").size(30.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                    tool_response.push(ui.label(RichText::new(format!("{:?} / {:?}", operation.on_site_tools, operation.total_tools)).size(30.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                });
                            }).response;
                            let patient = patient.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
                            tool_response.push(patient);
                            
                            let mut staff_response = Vec::new();
                            let staff = ui.horizontal(|ui| {
                                staff_response.push(ui.label(RichText::new("👷").size(60.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                ui.vertical(|ui| {
                                    staff_response.push(ui.heading(RichText::new("STAFF").size(30.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                    staff_response.push(ui.label(RichText::new(operation.staff_count.to_string()).size(30.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                });
                            }).response;
                            let staff = staff.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
                            staff_response.push(staff);
                        

                            tool_response.iter().for_each(|v| {
                                if v.clicked() {
                                    println!("hello patient!");
                                };
                            });
                            staff_response.iter().for_each(|v| {
                                if v.clicked() {
                                    println!("hello patient!");
                                };
                            });

                        } else {
                            ui.label("🔎 SEARCH OPERATION");
                            if ui.text_edit_singleline(&mut self.search.search_operation).changed() || self.require_update == true {
                                &self.filter_operation();

                                self.require_update = false;
                            }
            
                            ui.separator();
            
                            if self.search.search_operation_result.is_empty() && self.search.search_operation != "" {
                                ui.label("💤 No results found");
                            } else {
                                if let Some(data) = &mut self.data { 
                                    if !self.search.search_operation_result.is_empty() {
                                        TableData::build_table(ui, database::table::window::WindowTable::OperationSelect(Some(self.search.search_operation_result.clone())), data, &mut self.operation_id);
                                    }
                                }
                            }
                        }
                    });
                }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(margin);
                    ui.heading("system by geloquan ");

                    ui.separator();
                    
                    let current_time = Local::now(); 
                    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
                    
                    ui.label(format!("Current Time: {}", formatted_time));
                    if let Some(staff_credential) = self.staff.clone() {
                        if ui.button("logout").clicked() {
                            self.credential_panel.state = design::State::Default;
                            self.staff = None;
                        }
                        ui.horizontal(|ui| {
                            ui.label("name");
                            ui.label(staff_credential.full_name.clone());
                        });
                        ui.horizontal(|ui| {
                            ui.label("email");
                            ui.label(staff_credential.email.clone());
                        });
                    } else {}
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            
        });
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