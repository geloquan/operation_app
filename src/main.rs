mod database;
mod error;
mod action;

use std::{ops::{Deref, DerefMut}, sync::{Arc, RwLock}, time::Duration};

use application::menu::selected;
use data::dispatch::server::Server;
use database::table::{
    ui_builder::BuildTable, data::TableData, join::structure::OperationSelect, query::{self}, private::StaffAuthGrant
};

pub mod application;
use application::{authenticate::StaffCredential, field};
use application::{*, states, RunningApp, component as app_component};

pub mod ws;
use egui::{frame, CursorIcon, Id, LayerId, Margin, Order, Rounding, ScrollArea, WidgetText};
use egui::text::Fonts;
use egui::{menu, epaint, Align, Align2, Area, Color32, Direction, FontId, Frame, Layout, Pos2, RichText, Stroke, TextEdit, Window};
use futures::channel::mpsc::{Receiver, Sender};
use ws::receive::{
    self, Handle, Operation, ReceiveMessage
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
use ewebsock::{self, WsMessage, WsReceiver, WsSender};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};

type SafeOutbox = Arc<RwLock<Vec<WsMessage>>>;
type SafeMailbox = Arc<RwLock<Vec<WsMessage>>>;
type SafeServerConnection = Arc<RwLock<bool>>;
    
type SafeDataTable = Arc<RwLock<Option<TableData>>>;
type SafeStaff = Arc<RwLock<Option<StaffCredential>>>;
type SafeCredentialPanel = Arc<RwLock<states::Login>>;

#[derive(Deserialize, Debug, Serialize)]
struct SendMessage {
    level: String,
    method: String,
    data: Option<serde_json::Value>,
    staff_credential: Option<StaffCredential>,
    action: Option<action::Actions>
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

const DARKMODE_RED_HIGHLIGHT: Color32 = Color32::from_rgb(45, 8, 10);
const DEBUGCOLOR: Color32 = Color32::GOLD;
const SIDEPANELSIZE: f32 = 250.0;

pub struct OperationApp {
    outbox: Arc<RwLock<Vec<WsMessage>>>,
    server_connected: SafeServerConnection,
    
    data: SafeDataTable,
    staff: SafeStaff,
    credential_panel: SafeCredentialPanel,
    search: PreRunning,
    //central_window: OperationWindow,
    state: Option<RunningApp>,
    temp: Option<Temporary>,
    category: states::Category,
    operation_id: Option<i32>,
    require_update: bool,
    selected_menu: Option<application::menu::selected::Menu>,
    selected_action: Option<application::menu::selected::Action>,
    shared_value: Arc<RwLock<i32>>
}

impl OperationApp {
    fn new(cc: &eframe::CreationContext<'_>, shared_value: Arc<RwLock<i32>>, outbox: SafeOutbox,
    server_connected: SafeServerConnection,
    staff: SafeStaff,
    data_table: SafeDataTable,
    credential_panel: SafeCredentialPanel
    ) -> Self {

        OperationApp {
            outbox: outbox,
            server_connected: server_connected,
            data: data_table,
            staff: staff,
            credential_panel: credential_panel,
            search: PreRunning::default(),
            state: None,
            temp: None,
            category: states::Category::default(),
            operation_id: None,
            require_update: false,
            selected_menu: None,
            selected_action: None,
            shared_value
        }
    }
}

impl App for OperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_incoming();

        if let Some(id) = self.operation_id {
            self.select_operation(&id);
        }

        let staff: Option<StaffCredential> = if let Ok(staff_guard) = self.staff.clone().try_read() {
            let staff = staff_guard.clone();
            if staff.is_none() {
                app_component::login(&ctx, &mut self.credential_panel, &self.outbox, &self.staff);
            }
            staff
        } else {
            println!("staff lock issue");
            None
        };
        
        if staff.is_some() {
            let left_panel_rect: Pos2;
            egui::TopBottomPanel::top("top").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Ok(staff) = self.staff.read() {
                        let staff = staff.deref();
                        if let Some(staff) = staff {
                            ui.horizontal(|ui| {
                                ui.label("name");
                                ui.label(staff.full_name.clone());
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("email");
                                ui.label(staff.email.clone());
                            });
                        }
                    }
                    if ui.button("logout").clicked() {
                        if let (Ok(mut staff), Ok(mut credential_panel)) = (self.staff.write(), self.credential_panel.write()) {
                            credential_panel.state = design::State::Default;
                            
                            if let Some(staff) = staff.deref_mut() {
                                staff.email = "".to_owned();
                                staff.full_name = "".to_owned();
                                staff.id = 0;
                                staff.session_token = "".to_owned();
                            }
                        }
                    }
                    let current_time = Local::now(); 
                    let formatted_time = format!("Current Time: {} : {}", current_time.format("%Y-%m-%d %H:%M:%S").to_string(), self.shared_value.read().unwrap());
                    let font_id: FontId = FontId::default(); 

                    let text_size = ui.fonts(|font| {
                        let mut calculated_width = 0.0;
                        for char in formatted_time.chars() {
                            calculated_width += font.glyph_width(&font_id, char);
                        }
                        calculated_width
                    });

                    ui.add_space(ui.available_width() - text_size);
                    
                    ui.label(formatted_time);

                });
            });
            egui::SidePanel::left("left").show(ctx, |ui| {
                let margin = 20.0;
                let rect = ui.min_rect();
                let left_panel_rect = rect.center();
                ui.set_max_width(SIDEPANELSIZE);
                
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
                }
            
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(margin);
                    ui.heading("system by geloquan ");

                    ui.separator();
                    
                });
            });
            egui::SidePanel::right("right").show(ctx, |ui| {
                ui.set_max_width(SIDEPANELSIZE);
                
                Frame::none()
                .fill(Color32::default())
                .show(ui, |ui| {
                    ui.set_height(ui.available_height());
                    
                    Frame::none()
                    .inner_margin(Margin::same(20.0))
                    .show(ui, |ui| {
                        ui.heading(RichText::new("Action log"));
                    });
                    
                    ScrollArea::vertical()
                    .id_salt("action_log")
                    .auto_shrink([false; 2]) // Disable auto-shrink if needed
                    .show(ui, |ui| {
                        if let Some(action_log) = self.get_action_log_operation() {
                            for row in &action_log {
                                ui.horizontal(|ui| {
                                    ui.label("label: ");
                                    ui.label(row.label.clone());
                                });
                                ui.horizontal(|ui| {
                                    ui.label("staff: ");
                                    ui.label(row.staff.clone());
                                });
                                ui.horizontal(|ui| {
                                    ui.label("before: ");
                                    ui.label(row.before_val.clone());
                                    ui.label("after: ");
                                    ui.label(row.after_val.clone());
                                    
                                });
                                ui.horizontal(|ui| {
                                    ui.label("date: ");
                                    ui.label(row.date.clone());
                                });
                                ui.separator();
                            }
                        }
                    
                    });
                });
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(50.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    if self.get_selected_operation().is_none() {
                        ui.label("🔎 SEARCH OPERATION");
                        if ui.text_edit_singleline(&mut self.search.search_operation).changed() || self.require_update == true {
                            &self.filter_operation();

                            self.require_update = false;
                        }
        
                        ui.separator();
        
                        if self.search.search_operation_result.is_empty() && self.search.search_operation != "" {
                            ui.label("💤 No results found");
                        } else {
                            if let Ok(data) = self.data.clone().read() {
                                if let Some(_) = data.deref() { 
                                    if !self.search.search_operation_result.is_empty() {
                                        ui.horizontal_centered(|ui| {
                                            self.build_table(ui, database::table::window::WindowTable::OperationSelect(Some(self.search.search_operation_result.clone())));
                                        });
                                    }
                                }
                            }
                        }
                    } else {
                        if let (Some(selected_menu), Some(operation_id)) = (&self.selected_menu, self.operation_id) {
                            match selected_menu {
                                application::menu::selected::Menu::PreOperativeDefault => {
                                    println!("select options below");
                                },
                                application::menu::selected::Menu::PreOperativeToolReady => {
                                    if let Some(preoperative_tool_ready) = self.get_preoperative_tool_ready() {
                                        if let Ok(data) = self.data.clone().read().as_deref() {
                                            if let Some(_) = data { 
                                                self.build_table(ui, database::table::window::WindowTable::PreOperativeToolReady(Some(preoperative_tool_ready.clone())));
                                            }
                                        }
                                    }
                                },
                            }
                        }
                    }
                });
            });
            egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
                Frame::none()
                .inner_margin(Margin::same(10.))
                .show(ui, |ui| {
                        ui.horizontal(|ui| ui.heading("options:"));
                    });
                if let Some(operation) = self.get_selected_operation() {
                    ui.horizontal_centered(|ui| {
                        let mut staff_clr: Color32 = Color32::default();
                        let mut tool_clr: Color32 = Color32::default();
                        let mut ascend_clr: Color32 = Color32::default();
                        if let Some(selected_menu) = &self.selected_menu {
                            match selected_menu {
                                selected::Menu::PreOperativeDefault => {Color32::default();},
                                selected::Menu::PreOperativeToolReady => {tool_clr = DARKMODE_RED_HIGHLIGHT},
                            }
                        }

                        
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
                    
                    
                        let mut ascend_response = Vec::new();
                        let ascend = ui.horizontal(|ui| {
                            ascend_response.push(ui.label(RichText::new("⏭").size(60.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                            ui.vertical(|ui| {
                                ascend_response.push(ui.heading(RichText::new("ASCEND").size(30.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                            });
                        }).response;
                        let ascend = ascend.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
                        ascend_response.push(ascend);
                        
                        Frame::none()
                        .rounding(Rounding::same(20.0))
                        .fill(tool_clr)
                        .inner_margin(Margin::same(20.0))
                        .show(ui, |ui| {
                            let mut tool_response = Vec::new();
                            let tool = ui.horizontal(|ui| {
                                tool_response.push(ui.label(RichText::new("⚒").size(60.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                ui.vertical(|ui| {
                                    tool_response.push(ui.heading(RichText::new("TOOLS").size(30.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                    tool_response.push(ui.label(RichText::new(format!("{:?} / {:?}", operation.on_site_tools, operation.total_tools)).size(30.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                });
                            }).response;
                            let tool = tool.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
                            tool_response.push(tool);
        
                            tool_response.iter().for_each(|v| {
                                if v.clicked() && self.selected_menu != Some(application::menu::selected::Menu::PreOperativeToolReady) {
                                    self.selected_menu = Some(application::menu::selected::Menu::PreOperativeToolReady);
                                } else if v.clicked() {
                                    self.selected_menu = None;
                                };
                            });
                        });
                        staff_response.iter().for_each(|v| {
                            if v.clicked() {
                                println!("hello staff!");
                            };
                        });
                        ascend_response.iter().for_each(|v| {
                            if v.clicked() {
                                println!("hello ascend!");
                            };
                        });
                        
                    });
                }
                ui.add_space(40.0);
            });
            if let Some(menu) = &self.selected_menu {
                egui::TopBottomPanel::bottom("bottome").show(ctx, |ui| {
                    Frame::none()
                    .inner_margin(Margin::same(10.))
                    .show(ui, |ui| {
                        ui.heading("actions")
                    });
                    match menu {
                        selected::Menu::PreOperativeDefault => todo!(),
                        selected::Menu::PreOperativeToolReady => {
                            Frame::none()
                            .rounding(Rounding::same(20.0))
                            .fill(Color32::TRANSPARENT)
                            .inner_margin(Margin::same(20.0))
                            .show(ui, |ui| {
                                let mut tool_response = Vec::new();
                                let tool = ui.horizontal(|ui| {
                                    tool_response.push(ui.label(RichText::new("⊞").size(40.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                    tool_response.push(ui.heading(RichText::new("add new requirement").size(20.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                                }).response;
                                let tool = tool.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
                                tool_response.push(tool);
            
                                tool_response.iter().for_each(|v: &egui::Response| {
                                    if v.clicked() && self.selected_action != Some(application::menu::selected::Action::AddRequirement) {
                                        self.selected_action = Some(application::menu::selected::Action::AddRequirement);
                                    } else if v.clicked() {
                                        self.selected_action = None;
                                    };
                                });
                            });
                        },
                    }
                });
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let shared_value = Arc::new(RwLock::new(0));
    let timer = shared_value.clone(); 
    
    let outbox: SafeOutbox = Arc::new(RwLock::new(Vec::new()));
    let outbox_clone: SafeOutbox = outbox.clone();
        
    let mailbox: SafeMailbox = Arc::new(RwLock::new(Vec::new()));
    let mailbox_clone: SafeMailbox = outbox.clone();
    
    
    let data_table: SafeDataTable = Arc::new(RwLock::new(None));
    let data_table_clone: SafeDataTable = data_table.clone();

    let staff: SafeStaff = Arc::new(RwLock::new(None));
    let staff_clone: SafeStaff = staff.clone();

    let login_state = states::Login { 
        field: field::Login {
            email: "".to_string(),
            password: "".to_string(),
            session_token: "".to_string()
        }, 
        state: design::State::Default
    };

    let safe_credential_panel: SafeCredentialPanel = Arc::new(RwLock::new(login_state));
    let credential_panel_clone: SafeCredentialPanel = safe_credential_panel.clone();
    
    let safe_server_connected: SafeServerConnection = Arc::new(RwLock::new(false));
    let server_connected_clone: SafeServerConnection = safe_server_connected.clone();
    tokio::spawn(async move {
        websocket(outbox_clone, mailbox_clone, server_connected_clone, data_table_clone, staff_clone, credential_panel_clone).await;
    });
    run_egui_app(shared_value, outbox, safe_server_connected, data_table, staff, safe_credential_panel);
}

fn run_egui_app(shared_value: Arc<RwLock<i32>>, outbox: SafeOutbox, server_connected: SafeServerConnection, data_table: SafeDataTable, 
    staff: SafeStaff, credential_panel: SafeCredentialPanel) -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();


    eframe::run_native("OPERATION APP", native_options, Box::new(|cc| {
        let app = OperationApp::new(cc, shared_value, outbox, server_connected, staff, data_table, credential_panel);
        Ok(Box::new(app))
    }))
}

async fn websocket(outbox: SafeOutbox, mailbox: SafeMailbox, server_connected: SafeServerConnection, data_table: SafeDataTable, staff: SafeStaff, credential_panel: SafeCredentialPanel) {
    let (mut s, mut r) = ewebsock::connect("ws://localhost:8080", ewebsock::Options::default()).unwrap();
    loop {
        if let Ok(mut server_connected) = server_connected.write() {
            if *server_connected {
                if let Ok(outbox) = outbox.read() {
                    for message in outbox.deref() {
                        println!("sending message...");
                        s.send(message.to_owned());
                    }
                } 
                if let Some(msg) = r.try_recv() {
                    match msg {
                        ewebsock::WsEvent::Opened => {
                        },
                        ewebsock::WsEvent::Message(text) => {
                            match text {
                                ewebsock::WsMessage::Binary(vec) => todo!(),
                                ewebsock::WsMessage::Text(text) => {
                                    match serde_json::from_str::<EncryptedText>(&text) {
                                        Ok(encrypted_text) => {
                                            if let Ok(key) = &generate_fixed_key() {
                                                if let Ok(decrypted_text) = decrypt_message(key, &encrypted_text.nonce, &encrypted_text.cipher_text) {
                                                    match serde_json::from_str::<ReceiveMessage>(&decrypted_text) {
                                                        Ok(message) => {
                                                            match message.operation {
                                                                Operation::Initialize => {
                                                                    if let Ok(mut data_table) = data_table.write() {
                                                                        let datae = data_table.as_mut();
                                                                        if let Some(data) = datae {
                                                                            data.initialize(message.data);
                                                                        } else {
                                                                            let mut new_table_data = TableData::new();
                                                                            new_table_data.initialize(message.data);
                                                                            *data_table = Some(new_table_data);
                                                                        }
                                                                    }
                                                                },
                                                                Operation::Update => {
                                                                    println!("update: {:?}", message);
                                                                    //self.update(message);
                                                                },
                                                                Operation::AuthHandshake => {
                                                                    println!("statuscode {:?}", message.status_code);
                                                                    if let Ok(mut staff) = staff.write() {
                                                                        match serde_json::from_str::<StaffCredential>(&message.data) {
                                                                            Ok(staff_credential) => {
                                                                                *staff = Some(staff_credential.to_owned());
                                                                            }
                                                                            Err(err) => {
                                                                                eprintln!("Failed to deserialize StaffCredential: {}", err);
                                                                    
                                                                                *staff = None;
                                                                            }
                                                                        }
                                                                    }
                                                                    if let Ok(mut credential_panel) = credential_panel.write() {
                                                                        if message.status_code == "success" { 
                                                                            credential_panel.state = design::State::Valid;
                                                                        } else {
                                                                            credential_panel.state = design::State::Error;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        Err(_) => {
                                                            println!("err parsing: ReceiveMessage");
                                                        },
                                                    }
                                                }
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
                            
                        },
                        ewebsock::WsEvent::Closed => {
            
                        },
                    }
                }
            } else {
                match ewebsock::connect("ws://localhost:8080", ewebsock::Options::default()) {
                    Ok(ws_sr) => {
                        s = ws_sr.0;
                        r = ws_sr.1;
                        *server_connected = true;
                    },
                    Err(_) => {
                        println!("cannot connect to server...");
                    },
                }
            }
        } else {
            //poisoned
        }
    }
    

}
