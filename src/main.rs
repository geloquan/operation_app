mod database;

mod action;

use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::thread;

use application::menu::selected;
use cipher::{decrypt_message, generate_fixed_key, EncryptedText};
use database::table::{
    ui_builder::BuildTable, data::TableData, join::structure::OperationSelect
};

pub mod application;
use application::{authenticate::StaffCredential, field};
use application::{states, component as app_component};

pub mod ws;
use egui::{Align, Id, LayerId, Margin, Order, Rounding, ScrollArea, Style, Vec2};
use egui::{Color32, FontId, Frame, Pos2, RichText, TextEdit};
use egui_extras::TableBuilder;
use futures::channel::mpsc::Receiver;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use ws::receive::{Handle, Operation, ReceiveMessage};

pub mod temporary;

pub mod cipher;

pub mod component;
use component::design;


use chrono::Local;
use eframe::{egui, App};
use ewebsock::{self, WsReceiver, WsSender};
use serde::{Deserialize, Serialize};

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
#[allow(dead_code)]
const DEBUGCOLOR: Color32 = Color32::GOLD;
const SIDEPANELSIZE: f32 = 250.0;
const FORM_BACKGROUND: Color32 = Color32::from_rgb(39, 45, 45);
const FORM_TEXT_SIZE: f32 = 30.0;

pub struct OperationApp {
    data: Option<TableData>,
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
    search: PreRunning,
    staff: Option<StaffCredential>,
    credential_panel: states::Login,
    operation_id: Option<i32>,
    require_update: bool,
    selected_menu: Option<application::menu::selected::Menu>,
    selected_action: Option<application::menu::selected::Action>,
}

impl OperationApp {
    fn new(_: &eframe::CreationContext<'_>, app_rx: mpsc::Receiver<String>, backend_tx: mpsc::Sender<String>) -> Self {
        
        OperationApp {
            data: None,
            sender: backend_tx,
            receiver: app_rx,
            search: PreRunning::default(),
            staff: None,
            credential_panel: states::Login {
                field: field::Login {
                    email: "".to_string(),
                    password: "".to_string(),
                    session_token: "".to_string()
                },
                state: design::State::Default,
            },
            operation_id: None,
            require_update: false,
            selected_menu: None,
            selected_action: None,
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
            app_component::login(&ctx, &mut self.credential_panel, &mut self.sender, &self.staff);
        } 

        if self.staff.is_some() {
            egui::TopBottomPanel::top("top").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(staff_credential) = self.staff.clone() {
                        ui.horizontal(|ui| {
                            ui.label("name");
                            ui.label(staff_credential.full_name.clone());
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("email");
                            ui.label(staff_credential.email.clone());
                        });
                        if ui.button("logout").clicked() {
                            self.credential_panel.state = design::State::Default;
                            self.staff = None;
                            self.operation_id = None;
                            self.data = None;
                        }
                    }
                    let current_time = Local::now(); 
                    let formatted_time = format!("Current Time: {}", current_time.format("%Y-%m-%d %H:%M:%S").to_string());
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
            egui::SidePanel::right("right").show(ctx, |ui: &mut egui::Ui| {
                ui.set_max_width(SIDEPANELSIZE);
                
                Frame::none()
                .fill(Color32::default())
                .show(ui, |ui| {
                    ui.set_height(ui.available_height());

                    if self.get_selected_operation().is_some() {
                        Frame::none()
                        .inner_margin(Margin::same(20.0))
                        .show(ui, |ui| {
                            ui.heading(RichText::new("Action log"));
                        });
                    };
                    
                    ScrollArea::vertical()
                    .id_salt("action_log")
                    .auto_shrink([false; 2]) 
                    .show(ui, |ui| {
                        if let Some(action_log) = self.get_action_log_operation() {
                            for row in &action_log {
                                ui.horizontal(|ui| {
                                    ui.label("target: ");
                                    ui.label(row.label_reference.clone());
                                });
                                ui.label(row.label.clone());
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
                if self.staff.is_some() {
                    ui.add_space(50.0);
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        if self.get_selected_operation().is_none() {
                            ui.label("🔎 SEARCH OPERATION");
                            if ui.text_edit_singleline(&mut self.search.search_operation).changed() || self.require_update == true {
                                let _ = &self.filter_operation();

                                self.require_update = false;
                            }
            
                            ui.separator();
            
                            if self.search.search_operation_result.is_empty() && self.search.search_operation != "" {
                                ui.label("💤 No results found");
                            } else if self.data.is_some() {
                                if !self.search.search_operation_result.is_empty() {
                                    Frame::none()
                                    .fill(FORM_BACKGROUND)
                                    .rounding(20.0)
                                    .inner_margin(20.0)
                                    .show(ui, |ui| {
                                        ui.columns(1, |columns| {
                                            columns[0].vertical_centered(|ui| {
                                                ui.add_space(20.0);
                                                ui.horizontal_centered(|ui| {
                                                    self.build_table(ui, database::table::window::WindowTable::OperationSelect(Some(self.search.search_operation_result.clone())));
                                                });
                                            });
                                        });
                                    });
                                }
                            }
                        } else if let (Some(selected_action), Some(_), Some(data)) = (&mut self.selected_action, &self.operation_id, &self.data) {
                            ui.heading("New Requirement Form");
                            ui.add_space(20.0);
                            match selected_action {
                                selected::Action::AddRequirement(s) => {
                                    if let Some(s) = s {
                                        match data.equipment.read() {
                                            Ok(equipments) => {
                                                Frame::none()
                                                .fill(FORM_BACKGROUND)
                                                .rounding(20.0)
                                                .inner_margin(20.0)
                                                .show(ui, |ui| {
                                                    ui.columns(1, |columns| {
                                                        columns[0].vertical_centered(|ui| {
                                                            ui.set_width(150.0);
                                                            ui.horizontal_wrapped(|ui| {
                                                                ui.push_id("select", |ui| {
                                                                    ui.heading(RichText::new("Select: ").size(FORM_TEXT_SIZE));
                                                                    ui.separator();
                                                                    egui::ComboBox::from_label("")
                                                                    .selected_text(&s.name) 
                                                                    .show_ui(ui, |ui| {
                                                                        for equipment in equipments.iter() {
                                                                            if let Some(name) = &equipment.name {
                                                                                ui.selectable_value(&mut s.name, name.clone(), name.clone());
                                                                            }
                                                                        }
                                                                    });
                                                                });
                                                            });

                                                            ui.horizontal_wrapped(|ui| {
                                                                ui.label(RichText::new("On Site: ").size(FORM_TEXT_SIZE));
                                                                ui.separator();
                                                                let mut style: Style = (*ctx.style()).clone();
                                                                style.spacing.icon_width = 32.0;
                                                                style.spacing.icon_spacing = 16.0;
                                                                ctx.set_style(style);
                                                                ui.checkbox(&mut s.on_site, "");
                                                            });
                                                            
                                                            ui.horizontal_wrapped(|ui| {
                                                                ui.push_id("qty", |ui| {
                                                                    ui.label(RichText::new("Quantity: ").size(FORM_TEXT_SIZE));
                                                                    ui.separator();
                                                                    egui::ComboBox::from_label("")
                                                                    .selected_text(s.quantity.to_string())
                                                                    .show_ui(ui, |ui| {
                                                                        for i in 1..=99 {
                                                                            ui.selectable_value(&mut s.quantity, i, i.to_string());
                                                                        }
                                                                    });
                                                                });
                                                            });
                                                            ui.horizontal_wrapped(|ui| {
                                                                if ui.button(RichText::new("SUBMIT").size(FORM_TEXT_SIZE)).clicked() {
                                                                    println!("{:?}", s);
                                                                }
                                                            });
                                                        });
                                                    });
                                                });
                                            
                                            },
                                            Err(_) => todo!(),
                                        }
                                    }
                                },
                            }              
                        } else if let (Some(selected_menu), Some(_)) = (&self.selected_menu, self.operation_id) {
                            match selected_menu {
                                application::menu::selected::Menu::PreOperativeToolReady => {
                                    if let Some(preoperative_tool_ready) = self.get_preoperative_tool_ready() {
                                        if let Some(data) = &mut self.data { 
                                            ui.heading("Tool Checklist");
                                            ui.add_space(20.0);
                                            
                                            Frame::none()
                                            .fill(FORM_BACKGROUND)
                                            .rounding(20.0)
                                            .inner_margin(20.0)
                                            .show(ui, |ui| {
                                                ui.columns(1, |columns| {
                                                    columns[0].vertical_centered(|ui| {
                                                        self.build_table(ui, database::table::window::WindowTable::PreOperativeToolReady(Some(preoperative_tool_ready.clone())));
                                                    });
                                                });
                                            });
                                        }
                                    }
                                },
                            }
                        } 
                    });
                } 
            });
            egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
                if let Some(operation) = self.get_selected_operation() {
                    Frame::none()
                    .inner_margin(Margin::same(10.))
                    .show(ui, |ui| {
                            ui.horizontal(|ui| ui.heading("options:"));
                        });
                    ui.horizontal_centered(|ui| {
                        let staff_clr: Color32 = Color32::default();
                        let mut tool_clr: Color32 = Color32::default();
                        let ascend_clr: Color32 = Color32::default();
                        if let Some(selected_menu) = &self.selected_menu {
                            match selected_menu {
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
                                    self.selected_action = None;
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
                                    if v.clicked() && !matches!(self.selected_action, Some(application::menu::selected::Action::AddRequirement(_))) {
                                        self.selected_action = Some(application::menu::selected::Action::AddRequirement(Some(application::forms::NewEquipmentRequirement::default())));
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

        ctx.request_repaint();
    }
}

fn main() {
    let native_options: eframe::NativeOptions = eframe::NativeOptions::default();

    let (backend_tx, mut backend_rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(1_000_000);
    let (app_tx, mut app_rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(1_000_000);
        
    let (mut sender, receiver) = ewebsock::connect("ws://192.168.1.6:8080", ewebsock::Options::default()).unwrap();
    let _server_socket_thread = Runtime::new()
    .unwrap()
    .block_on(async {
        tokio::spawn(async move {
            while let Some(message) = receiver.try_recv() {
                match message {
                    ewebsock::WsEvent::Opened => {
                        
                    },
                    ewebsock::WsEvent::Message(text) => {
                        match text {
                            ewebsock::WsMessage::Binary(vec) => todo!(),
                            ewebsock::WsMessage::Text(text) => {
                                let text_len = text.len();
                                println!("text len: {:?}", text_len);
                                match serde_json::from_str::<EncryptedText>(&text) {
                                    Ok(encrypted_text) => {
                                        if let Ok(key) = &generate_fixed_key() {
                                            if let Ok(decrypted_text) = decrypt_message(key, &encrypted_text.nonce, &encrypted_text.cipher_text) {
                                                match serde_json::from_str::<ReceiveMessage>(&decrypted_text) {
                                                    Ok(message) => {
                                                        match message.operation {
                                                            Operation::Initialize => {
                                                                if let Some(data) = &mut self.data {
                                                                    data.initialize(message.data);
                                                                } else {
                                                                    let mut new_table_data = TableData::new();
                                                                    new_table_data.initialize(message.data);
                                                                    self.data = Some(new_table_data);
                                                                }
                                                            },
                                                            Operation::Update => {
                                                                println!("update: {:?}", message);
                                                                self.update(message);
                                                            },
                                                            Operation::AuthHandshake => {
                                                                println!("statuscode {:?}", message.status_code);
                                                                if let Ok(staff_credential) = serde_json::from_str::<StaffCredential>(&message.data) {
                                                                    self.staff = Some(staff_credential);
                                                                } else {
                                                                    self.staff = None;
                                                                }
                                                                if message.status_code == "success" { 
                                                                    self.credential_panel.state = design::State::Valid
                                                                }
                                                                else if message.status_code == "failed" { 
                                                                    self.credential_panel.state = design::State::Error 
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
                        let options = ewebsock::Options::default();
                        let (mut sender, receiver) = ewebsock::connect("ws://192.168.1.6:8080", options).unwrap();
                        
                        let request_json = serde_json::to_string(&SendMessage {
                            level: "Operation".to_string(),
                            method: "Initial".to_string(),
                            data: Some(json!({"content": "Hello from button('Send Message')!"})),
                            staff_credential: self.staff.clone(),
                            action: None
                        }).unwrap();
                        sender.send(ewebsock::WsMessage::Text(request_json));
    
                        self.sender = sender;
                        self.receiver = receiver;
                    },
                    ewebsock::WsEvent::Closed => {
    
                    },
                }
            }
        });
    });

    let _backend_thread = Runtime::new()
    .unwrap()
    .block_on(async {
        tokio::spawn(async move {
            println!("backend-thread");
            
            while let Ok(msg) = backend_rx.try_recv() {
                sender.send(ewebsock::WsMessage::Text("".to_string()));
                //sender
            }
        });
    });
    let _app_thread = run_app(native_options, app_rx, backend_tx);

}

fn run_app(native_options: eframe::NativeOptions, app_rx: mpsc::Receiver<String>, backend_tx: mpsc::Sender<String>) -> Result<(), eframe::Error> {
    eframe::run_native("OPERATION APP", native_options, Box::new(|cc| {
        let app = OperationApp::new(cc, app_rx, backend_tx);
        
        Ok(Box::new(app))
    }))
}