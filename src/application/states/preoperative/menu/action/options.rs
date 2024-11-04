
use std::sync::mpsc::Sender;

use egui::{
    Context, 
    Frame, 
    Margin, 
    RichText, 
    Style, 
    Ui
};
use egui_extras::{Column, TableBuilder};

use crate::{
    action::{Actions, Preoperation}, application::operation::menu::preoperative::{
            self, 
            action::{NewEquipmentRequirement, RemoveEquipmentRequirement}, 
            Action
        }, database::{
        self, 
        table::{
            data::TableData, 
            ui_builder::BuildTable
        }
    }, OperationApp, FORM_BACKGROUND, FORM_TEXT_SIZE
};

pub fn tool_ready_action_options(
    ui: &mut Ui,
    selected_action: &mut Option<Action>
) {
    let _ = Frame::none()
    .inner_margin(Margin::same(20.0))
    .show(ui, |ui| {
        let mut tool_response = Vec::new();
        let first_tool = ui.horizontal(|ui| {
            tool_response.push(ui.label(RichText::new("⊞").size(40.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
            tool_response.push(ui.heading(RichText::new("add new requirement").size(20.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
        }).response;
        
        let tool = first_tool.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
        tool_response.push(tool);
        tool_response.iter().for_each(|v: &egui::Response| {
            if v.clicked() && !matches!(selected_action, Some(_)) {
                *selected_action = Some(preoperative::Action::AddRequirement(Some(NewEquipmentRequirement::default())));
            } else if v.clicked() {
                *selected_action = None;
            };
        });
    });
    let _ = Frame::none()
    .inner_margin(Margin::same(20.0))
    .show(ui, |ui| {
        let mut tool_response = Vec::new();
        let first_tool = ui.horizontal(|ui| {
            tool_response.push(ui.label(RichText::new("⊞").size(40.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
            tool_response.push(ui.heading(RichText::new("remove requirement").size(20.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
        }).response;
        
        let tool = first_tool.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
        tool_response.push(tool);
        tool_response.iter().for_each(|v: &egui::Response| {
            if v.clicked() && !matches!(selected_action, Some(_)) {
                *selected_action = Some(preoperative::Action::RemoveRequirement(Some(RemoveEquipmentRequirement { id: 0, name: "".to_string(), status: database::table::public::EquipmentStatus::Borrowed })));
            } else if v.clicked() {
                *selected_action = None;
            };
        });
    });
}
pub fn staff_list_action_options(
    ui: &mut Ui,
    selected_action: &mut Option<Action>
) {
    let _ = Frame::none()
    .inner_margin(Margin::same(20.0))
    .show(ui, |ui| {
        let mut tool_response = Vec::new();
        let first_tool = ui.horizontal(|ui| {
            tool_response.push(ui.label(RichText::new("⊞").size(40.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
            tool_response.push(ui.heading(RichText::new("add new role requirement").size(20.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
        }).response;
        
        let tool = first_tool.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
        tool_response.push(tool);
        tool_response.iter().for_each(|v: &egui::Response| {
            if v.clicked() && !matches!(selected_action, Some(_)) {
                *selected_action = Some(preoperative::Action::AddStaffRole);
            } else if v.clicked() {
                *selected_action = None;
            };
        });
    });
}

pub fn equipment_requested_options(
    ui: &mut Ui,
    selected_action: &mut Option<Action>
) {
    let _ = Frame::none()
    .inner_margin(Margin::same(20.0))
    .show(ui, |ui| {
        let mut tool_response = Vec::new();
        let first_tool = ui.horizontal(|ui| {
            tool_response.push(ui.label(RichText::new("⊞").size(40.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
            tool_response.push(ui.heading(RichText::new("ADD NEW REQUIREMENT").size(20.0)).interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand));
        }).response;
        
        let tool = first_tool.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
        tool_response.push(tool);
        tool_response.iter().for_each(|v: &egui::Response| {
            if v.clicked() && !matches!(selected_action, Some(_)) {
                *selected_action = Some(preoperative::Action::AddRequirement(Some(NewEquipmentRequirement::default())));
            } else if v.clicked() {
                *selected_action = None;
            };
        });
    });
}