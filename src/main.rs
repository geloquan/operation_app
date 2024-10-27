mod database;

mod action;

use action::Actions;
use application::data::dispatch::Dispatch;
use database::table::{
    ui_builder::BuildTable, 
    data::TableData, 
    join::structure::OperationSelect
};

pub mod application;
use application::{
    authenticate::StaffCredential, 
    field
};
use application::{
    component as app_component, 
    operation, 
    states
};

pub mod ws;
use egui::{
    Margin, 
    ScrollArea
};
use egui::{
    Color32, 
    FontId, 
    Frame, 
    RichText, 
    TextEdit
};
use ws::receive::Handle;

use application::states::preoperative::menu::action as PreoperativeMenuAction;
use application::operation::menu::preoperative::Action as PreoperativeActions;
use application::operation::menu::preoperative::MenuOptions as PreoperativeMenuActionOptions;



use application::operation::State as OperationStates;
use application::states::preoperative::menu as PreoperativeMenu;

use application::states::intraoperative::menu as IntraoperativeMenu;

pub mod temporary;

pub mod cipher;

pub mod component;

use component::design;

use chrono::Local;
use eframe::{
    egui, 
    App
};
use ewebsock::{
    self, 
    WsReceiver, 
    WsSender
};
use serde::{
    Deserialize, 
    Serialize
};
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
    sender: WsSender,
    receiver: WsReceiver,
    search: PreRunning,
    staff: Option<StaffCredential>,
    credential_panel: states::Login,
    operation_id: Option<i32>,
    require_update: bool,
    operation_state: Option<application::operation::State>,
    pub app_tx: std::sync::mpsc::Sender<Actions>,
    pub app_rx: std::sync::mpsc::Receiver<Actions>
}

impl OperationApp {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        
        let options = ewebsock::Options::default();
        let (sender, receiver) = ewebsock::connect("ws://192.168.1.2:8080", options).unwrap();

        let (app_tx, app_rx) = std::sync::mpsc::channel();

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
            operation_state: None,
            app_tx,
            app_rx,
        }
    }
}

impl App for OperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_incoming();

        while let Ok(action) = &self.app_rx.try_recv() {
            let main_panel_reload: bool = match action {
                Actions::Preoperation(preoperation) => {
                    match preoperation {
                        action::Preoperation::ToolOnSiteToggle(operation_tool_on_site_toggle) => {
                            self.action(action.to_owned());
                            false
                        },
                        action::Preoperation::AddNewEquipmentRequirement(new_equipment_requirement) => {
                            true
                        },
                        action::Preoperation::RemoveEquipmentRequirement(remove_equipment_requirement) => {
                            true
                        },
                    }
                },
            };
            if main_panel_reload {
                if let Some(operation_state) = &mut self.operation_state {
                    match operation_state {
                        operation::State::Preoperation(menu) => {
                            menu.selected_action = None;
                            menu.selected_menu = None;
                        },
                        operation::State::Intraoperation(menu) => {
                            menu.selected_action = None;
                            menu.selected_menu = None;
                        },
                        operation::State::Postoperation => todo!(),
                    }
                }
            }
        }

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
                
                if let Some(operation_state) = &self.operation_state {
                    if let operation::State::Preoperation(_) = operation_state {
                        if let Some(operation) = self.get_selected_preoperation() {
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
                                ui.heading("APPROVED CONSENT: ");
                                ui.add_enabled(false, 
                                    TextEdit::singleline(&mut if operation.approved_consent { "YES".to_string() } else { "NO".to_string() })
                                );
                            });
                        }
                    }
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

                    if self.operation_state.is_some() {
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
                        if self.operation_state.is_none() {
                            ui.label("🔎 SEARCH OPERATION");
                            if ui.text_edit_singleline(&mut self.search.search_operation).changed() || 
                            self.require_update == true {
                                let _ = &self.filter_operation();

                                self.require_update = false;
                            }
            
                            ui.separator();
            
                            if self.search.search_operation_result.is_empty() && 
                            self.search.search_operation != "" {
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
                        } else if let (
                            Some(operation_state), 
                            Some(_), 
                            Some(data), 
                            app_tx
                        ) = (
                            &mut self.operation_state, 
                            &self.operation_id, 
                            &self.data, 
                            &self.app_tx
                        ) {
                            match operation_state {
                                OperationStates::Preoperation(menu) => {
                                    if let Some(selected_action) = &mut menu.selected_action {
                                        ui.add_space(20.0);
                                        match selected_action {
                                            PreoperativeActions::AddRequirement(s) => {
                                                ui.heading("New Requirement Form");
                                                PreoperativeMenuAction::add_requirement_area(s, data, ui, ctx, app_tx);
                                            },
                                            PreoperativeActions::RemoveRequirement(s) => {
                                                ui.heading("Remove Requirement");
                                                PreoperativeMenuAction::remove_requirement_area(s, data, ui, ctx, app_tx);
                                            },
                                            PreoperativeActions::AddStaffRole => {
                                                
                                            },
                                            PreoperativeActions::RemoveStaffRole => {
                                                
                                            },
                                        }
                                    } else if let Some(selected_menu) = &menu.selected_menu {
                                        match selected_menu {
                                            PreoperativeMenuActionOptions::ToolReady => {
                                                PreoperativeMenuAction::tool_checklist_area(self, ui);
                                            },
                                            PreoperativeMenuActionOptions::Staff => {
                                                PreoperativeMenuAction::staff_list_area(self, ui);
                                            },
                                        }
                                    
                                    }
                                },
                                OperationStates::Intraoperation(_) => {},
                                OperationStates::Postoperation => todo!(),
                            }           
                        } 
                    });
                } 
            });
            egui::TopBottomPanel::bottom("menu_option").show(ctx, |ui| {
                if let (
                    Some(operation), 
                    Some(operation_state),
                    operation_id, 
                    sender, 
                    staff_credential
                ) = (
                    &mut self.get_selected_preoperation(), 
                    &mut self.operation_state,
                    &self.operation_id,
                    &mut self.sender, 
                    &self.staff
                ) {
                    if let OperationStates::Preoperation(menu) = operation_state {
                        PreoperativeMenu::init(ui, menu, operation, operation_id, sender, staff_credential);
                    }
                } else if let (
                    Some(operation), 
                    Some(operation_state),
                    operation_id, 
                    sender, 
                    staff_credential
                ) = (
                    &mut self.get_selected_intraoperation(), 
                    &mut self.operation_state,
                    &self.operation_id,
                    &mut self.sender, 
                    &self.staff
                ) {
                    if let OperationStates::Intraoperation(menu) = operation_state {
                        IntraoperativeMenu::init(ui, menu, operation, operation_id, sender, staff_credential);
                    }
                }
                ui.add_space(40.0);
            });

            if let Some(operation_state) = &mut self.operation_state {
                egui::TopBottomPanel::bottom("action_option").show(ctx, |ui| {
                    match operation_state {
                        operation::State::Preoperation(menu) => {
                            if let (Some(selected_menu), selected_action) = (menu.selected_menu.as_mut(), &mut menu.selected_action) {
                                match selected_menu {
                                    PreoperativeMenuActionOptions::Staff => {
                                        PreoperativeMenuAction::staff_list_action_options(ui, selected_action);
                                    },
                                    PreoperativeMenuActionOptions::ToolReady => {
                                        PreoperativeMenuAction::tool_ready_action_options(ui, selected_action);
                                    },
                                }
                                if selected_action.is_none() {
                                } else {
                                    
                                }
                            } else {
                                
                            }
                        
                        },
                        operation::State::Intraoperation(_) => {},
                        operation::State::Postoperation => todo!(),
                    }
                });
            } 
        }

        ctx.request_repaint();
    }
}

fn main() {
    let native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    let _app_thread = run_app(native_options);

}

fn run_app(native_options: eframe::NativeOptions) -> Result<(), eframe::Error> {
    eframe::run_native("OPERATION APP", native_options, Box::new(|cc| {
        let app = OperationApp::new(cc);

        Ok(Box::new(app))
    }))
}