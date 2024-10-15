mod database;

mod action;

use application::menu::selected;
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
    data: Option<TableData>,
    sender: WsSender,
    receiver: WsReceiver,
    search: PreRunning,
    staff: Option<StaffCredential>,
    credential_panel: states::Login,
    operation_id: Option<i32>,
    require_update: bool,
    selected_menu: Option<application::menu::selected::Menu>,
    selected_action: Option<application::menu::selected::Action>,
}

impl OperationApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        let options = ewebsock::Options::default();
        let (sender, receiver) = ewebsock::connect("ws://192.168.1.7:8080", options).unwrap();

        OperationApp {
            data: None,
            sender,
            receiver,
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
            let left_panel_rect: Pos2;
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
                if self.staff.is_some() {
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
                                if let Some(data) = &mut self.data { 
                                    if !self.search.search_operation_result.is_empty() {
                                        ui.horizontal_centered(|ui| {
                                            self.build_table(ui, database::table::window::WindowTable::OperationSelect(Some(self.search.search_operation_result.clone())));
                                        });
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
                                            if let Some(data) = &mut self.data { 
                                                self.build_table(ui, database::table::window::WindowTable::PreOperativeToolReady(Some(preoperative_tool_ready.clone())));
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    });
                } 
                
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

        ctx.request_repaint();
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