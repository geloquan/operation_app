
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};
use components::login;
use eframe::{egui, App};
use egui::{mutex::Mutex, Color32, Id, Label, RichText, Sense};
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


mod services;

mod components;

mod views;

struct OperationApp {
    service: services::Service,
    view: std::rc::Rc<std::cell::RefCell<views::View>>,
    login: components::login::Login,
}

impl OperationApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> OperationApp {
        //let request_json = serde_json::to_string(&SendMessage {
        //    level: "operation".to_string(),
        //    method: "initial".to_string(),
        //    data: Some(json!({"content": "Hello from button('Send Message')!"})),
        //}).unwrap();
        //sender.send(ewebsock::WsMessage::Text(request_json));

        let service = services::Service::init().expect(&"ggs");
        let view =  std::rc::Rc::new(std::cell::RefCell::new(views::View::default()));
        let login = components::login::Login::new(std::rc::Rc::clone(&view));

        OperationApp {
            service,
            view,
            login
        }
    }
}

impl App for OperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let view: views::View = self.login.get_view();
        match view {
            views::View::Login => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.login.show(ctx);
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
//#[derive(Debug)]
//pub struct CodeExample {
//    name: String,
//    age: u32,
//}
//impl Demo for CodeExample {
//    fn name(&self) -> &'static str {
//        "🖮 Code Example"
//    }
//
//    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
//        use crate::View;
//        egui::Window::new(self.name())
//            .open(open)
//            .min_width(375.0)
//            .default_size([390.0, 500.0])
//            .scroll(false)
//            .resizable([true, false]) // resizable so we can shrink if the text edit grows
//            .show(ctx, |ui| self.ui(ui));
//    }
//}

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("OPERATION APP", native_options, Box::new(|cc| {
        let app = OperationApp::new(cc);
        Ok(Box::new(app))
    }));
}