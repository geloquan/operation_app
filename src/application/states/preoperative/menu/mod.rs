use egui::{Color32, Frame, Margin, RichText, Rounding};
use ewebsock::WsSender;

use crate::{application::{authenticate::StaffCredential, operation::{self, menu::preoperative::{Init, Menu}}}, database::table::{private::OperationAscend, public::OperationStatus}, SendMessage};

pub mod action;

pub fn init(
    ui:&mut egui::Ui, 
    menu: &mut Menu, 
    operation: &mut Init, 
    operation_id: &Option<i32>, 
    sender: &mut WsSender, 
    staff_credential: &Option<StaffCredential>
) {
    Frame::none()
    .inner_margin(Margin::same(10.))
    .show(ui, |ui| {
            ui.horizontal(|ui| ui.heading("options:"));
        });
    ui.horizontal_centered(|ui| {
        let staff_clr: Color32 = Color32::default();
        let mut tool_clr: Color32 = Color32::default();
        let ascend_clr: Color32 = Color32::default();
        
        let mut staff_response = Vec::new();
        let staff = ui.horizontal(|ui| {
            staff_response.push(ui.label(RichText::new("👷").size(60.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
            ui.vertical(|ui| {
                let rich_text = RichText::new("STAFF");
                staff_response.push(ui.heading(rich_text.size(30.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                staff_response.push(ui.label(RichText::new(operation.staff_count.to_string()).size(30.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
            });
        }).response;
        let staff = staff.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
        staff_response.push(staff);
    
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
                if v.clicked() && menu.selected_menu != Some(operation::menu::preoperative::MenuOptions::ToolReady) {
                    menu.selected_menu = Some(operation::menu::preoperative::MenuOptions::ToolReady);
                } else if v.clicked() {
                    menu.selected_menu = None;
                    menu.selected_action = None;
                };
            });
        });
        
        Frame::none()
        .rounding(Rounding::same(20.0))
        .fill(tool_clr)
        .inner_margin(Margin::same(20.0))
        .show(ui, |ui| {
            let mut ascend_response = Vec::new();
            let ascend = ui.horizontal(|ui| {
                ascend_response.push(ui.label(RichText::new("⏭").size(60.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                ui.vertical(|ui| {
                    ascend_response.push(ui.heading(RichText::new("ASCEND").size(30.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
                });
            }).response;
            let ascend = ascend.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
            ascend_response.push(ascend);

            ascend_response.iter().for_each(|v| {
                if v.clicked() {
                    if let Some(operation_id) = &operation_id {
                        let operation_ascend = OperationAscend {
                            operation_id: *operation_id,
                            operation_status: OperationStatus::PreOperative,
                            staff_credential: staff_credential.clone().unwrap(),
                        };
                        let request_json = serde_json::to_string(&SendMessage {
                            level: "Operation".to_string(),
                            method: "Ascend".to_string(),
                            data: Some(serde_json::to_value(&operation_ascend).unwrap()),
                            staff_credential: staff_credential.clone(),
                            action: None
                        }).unwrap();
                        sender.send(ewebsock::WsMessage::Text(request_json.to_string()));
                    };
                };
            });
        });

        staff_response.iter().for_each(|v| {
            if v.clicked() {
                println!("hello staff!");
            };
        });
    });
}
