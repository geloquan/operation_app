use std::ops::DerefMut;

use egui::{Align2, Color32, Ui, Window};
use ewebsock::{WsMessage, WsSender};

use crate::{component::design, SafeCredentialPanel, SafeOutbox, SafeStaff, SendMessage};

use super::authenticate::StaffCredential;

pub mod format;

pub fn login(ctx: &egui::Context, credential_panel: &SafeCredentialPanel, outbox: &SafeOutbox, staff: &SafeStaff) {

    let width = 500.0;
    let height = 250.0;

    Window::new("STAFF LOGIN")
        .default_open(true)
        .resizable(true)
        .collapsible(false)
        .fixed_size([width, height])
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            let color: Color32 = match credential_panel.lock() {
                Ok(panel_option) => {
                    match panel_option.as_ref() {
                        Some(login) => {
                            match login.state {
                                design::State::Waiting => Color32::from_hex("#FFA652").unwrap(),
                                design::State::Error => Color32::RED,
                                design::State::Valid => Color32::GREEN,
                                design::State::Default => Color32::TRANSPARENT,
                            }
                        }
                        None => {
                            Color32::TRANSPARENT 
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to lock credential_panel: {}", err);
                    Color32::TRANSPARENT  // Return a default or error color if locking fails
                }
            };
            ui.horizontal(|ui| {
                ui.label("email ");
                if let Ok(credential_panel) = credential_panel.lock() {
                    let credential_panel = credential_panel.deref_mut();
                    if let Some(credential_panel) = credential_panel {
                        design::input(ui, &mut credential_panel.field.email, color, design::Category::Frame, "email");
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("password ");
                if let Ok(credential_panel) = credential_panel.lock() {
                    let credential_panel = credential_panel.deref_mut();
                    if let Some(credential_panel) = credential_panel {
                        design::input(ui, &mut credential_panel.field.password, color, design::Category::Frame, "password");
                    }
                }
            });
            if ui.button("login").clicked() {
                if let Ok(staff) = self.staff.lock() {
                    let staff = *staff;
                if let (Ok(credential_panel), Ok(staff)) = (credential_panel.lock(), staff.lock()) {
                    let credential_panel = credential_panel.deref_mut();
                    let staff = *staff;
                    if let Some(credential_panel) = credential_panel {
                        credential_panel.state = design::State::Waiting;
                        let request_json = serde_json::to_string(&SendMessage {
                            level: "Operation".to_string(),
                            method: "Authenticate".to_string(),
                            data: Some(serde_json::to_value(&credential_panel.field).unwrap()),
                            staff_credential: staff.clone(),
                            action: None
                        }).unwrap();
                    }
                }

                match outbox.lock() {
                    Ok(mut outbox) => {
                        outbox.push(ewebsock::WsMessage::Text(request_json.to_string()));
                    },
                    Err(_) => todo!(),
                }
                
                credential_panel.field.password = "".to_string();
                credential_panel.field.email = "".to_string();
                
            }
        }
    });
}
