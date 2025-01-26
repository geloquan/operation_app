
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


mod services;

struct OperationApp {
    service: services::Service
}

impl OperationApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //let request_json = serde_json::to_string(&SendMessage {
        //    level: "operation".to_string(),
        //    method: "initial".to_string(),
        //    data: Some(json!({"content": "Hello from button('Send Message')!"})),
        //}).unwrap();
        //sender.send(ewebsock::WsMessage::Text(request_json));

        let service = services::Service::init();

        OperationApp {
            service
        }
    }
}

impl App for OperationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left").show(ctx, |ui| {});
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.label("Hello Worled!");
        });
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}
#[derive(Debug)]
pub struct CodeExample {
    name: String,
    age: u32,
}
impl Demo for CodeExample {
    fn name(&self) -> &'static str {
        "🖮 Code Example"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use crate::View;
        egui::Window::new(self.name())
            .open(open)
            .min_width(375.0)
            .default_size([390.0, 500.0])
            .scroll(false)
            .resizable([true, false]) // resizable so we can shrink if the text edit grows
            .show(ctx, |ui| self.ui(ui));
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