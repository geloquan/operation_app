
use std::{cell::RefCell, rc::Rc, sync::Arc};

use components::{login, operation};
use eframe::{egui, App};
use egui::{Color32, Id, Label, RichText, Sense};
use egui_extras::{TableBuilder, Column};
use serde::{Deserialize, Serialize};
use services::middleman::{self, Get};

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


mod services;

mod components;

mod views;

mod models;

struct OperationApp {
    view: std::rc::Rc<std::cell::RefCell<views::View>>,
    login: components::login::Login,
    thread: Rc<RefCell<services::app::App>>,
}
impl OperationApp {
    pub fn new(cc: &eframe::CreationContext<'_>, thread: Rc<RefCell<services::app::App>>) -> OperationApp {

        let view =  std::rc::Rc::new(std::cell::RefCell::new(views::View::default()));
        let login = components::login::Login::new(std::rc::Rc::clone(&view));

        OperationApp {
            view,
            login,
            thread
        }
    }
}

impl App for OperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let view: views::View = self.login.get_view();
        match view {
            views::View::Login => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.login.show(ctx, &mut self.thread);
                });
            },
            views::View::OperationSelect => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    //let mut login_form = components::operation::select::LoginForm::LoginForm::new(components::operation::select::LoginForm::State::Default, "email".to_string(), "password".to_string());
                });
            },
            views::View::Operation(state) => {
                match state {
                    views::State::Preoperation => {
                        egui::TopBottomPanel::top(Id::new("side-top")).show(ctx, |ui| {
                            
                        });
                        egui::TopBottomPanel::bottom(Id::new("side-bottom")).show(ctx, |ui| {
                            
                        });
                        egui::SidePanel::left(Id::new("side-left")).show(ctx, |ui| {
                            
                        });
                        egui::CentralPanel::default().show(ctx, |ui| {

                        });
                        egui::SidePanel::right(Id::new("side-right")).show(ctx, |ui| {
                            
                        });
                        egui::TopBottomPanel::bottom(Id::new("side-bottom-bottom")).show(ctx, |ui| {
                            
                        });
                        egui::TopBottomPanel::top(Id::new("side-top-top")).show(ctx, |ui| {
                            
                        });
                    },
                    views::State::Intraoperation => {
                        
                    },
                    views::State::Postoperation => {
                        
                    },
                }
            },
        }
    }
}
#[derive(Debug)]
struct DataMessage {
    message: String
}
#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();

    let stream_database: Arc<std::sync::RwLock<models::StreamDatabase>> = std::sync::Arc::new(
        std::sync::RwLock::new(
            models::StreamDatabase::init(
                models::operation::OperationModel::new(
                    vec![models::operation::Operation::default()]
                ))
            )
        );
    
    let mut service = services::Service::init(stream_database).await.unwrap();
    let _ = eframe::run_native("OPERATION APP", native_options, Box::new(|cc| {
        let app = OperationApp::new(cc, service);
        Ok(Box::new(app))
    }));
}