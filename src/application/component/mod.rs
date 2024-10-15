use egui::{Align2, Color32, Window};
use ewebsock::WsSender;

use crate::{component::design, SendMessage};

use super::authenticate::StaffCredential;

pub mod format;

pub fn login(ctx: &egui::Context, credential_panel: &mut crate::application::states::Login, sender: &mut WsSender, staff: &Option<StaffCredential>) {
    let width = 500.0;
    let height = 250.0;

    Window::new("STAFF LOGIN")
        .default_open(true)
        .resizable(true)
        .collapsible(false)
        .fixed_size([width, height])
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            let color: Color32 = match credential_panel.state {
                design::State::Waiting => {Color32::from_hex("#FFA652").unwrap()},
                design::State::Error => {Color32::RED},
                design::State::Valid => {Color32::GREEN},
                design::State::Default => {Color32::TRANSPARENT},
            };
            ui.horizontal(|ui| {
                ui.label("email ");
                design::input(ui, &mut credential_panel.field.email, color, design::Category::Frame, "email");
            });
            ui.horizontal(|ui| {
                ui.label("password ");
                design::input(ui, &mut credential_panel.field.password, color, design::Category::Frame, "password");
            });
            if ui.button("login").clicked() {
                credential_panel.state = design::State::Waiting;
                let request_json = serde_json::to_string(&SendMessage {
                    level: "Operation".to_string(),
                    method: "Authenticate".to_string(),
                    data: Some(serde_json::to_value(&credential_panel.field).unwrap()),
                    staff_credential: staff.clone(),
                    action: None
                }).unwrap();
                sender.send(ewebsock::WsMessage::Text(request_json.to_string()));
                
                credential_panel.field.password = "".to_string();
                credential_panel.field.email = "".to_string();
                
            }
        });
}
