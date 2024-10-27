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

pub fn add_requirement_area(
    s: &mut Option<NewEquipmentRequirement>, 
    data: &TableData, 
    ui: &mut Ui, 
    ctx: &Context, 
    app_tx: &Sender<Actions>
) {
    if let Some(s) = s {
        match data.equipment.read() {
            Ok(equipments) => {
                Frame::none()
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
                                if ui.button(RichText::new("SUBMIT").size(FORM_TEXT_SIZE)).clicked() &&
                                s.name != "" &&
                                s.quantity != 0
                                {
                                    let _ = app_tx.send(
                                        Actions::Preoperation(
                                            Preoperation::AddNewEquipmentRequirement(
                                                s.to_owned()
                                            )
                                        )
                                    );
                                }
                            });
                        });
                    });
                });
            },
            Err(_) => todo!(),
        }
    }
}
pub fn remove_requirement_area(
    s: &mut Option<RemoveEquipmentRequirement>, 
    data: &TableData, 
    ui: &mut Ui, 
    ctx: &Context, 
    app_tx: &Sender<Actions>
) {
    Frame::none()
    .rounding(20.0)
    .inner_margin(20.0)
    .show(ui, |ui| {
        ui.columns(1, |columns| {
            columns[0].vertical_centered(|ui| {
                let tbl = TableBuilder::new(ui)
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .auto_shrink(true)
                .striped(true)
                .max_scroll_height(500.0)
                .header(20.0, |mut header| {
                    let headings = [
                        "EQUIPMENT",
                        "STATUS",
                        "",
                    ];                
                    for title in headings {
                        header.col(|ui| {
                            ui.horizontal_centered(|ui|{
                                ui.heading(title);
                            });
                        });
                    }
                })
                .body(|mut body| {
                    for content in s {
                        
                    }
                });
            });
        });
    });
}

pub fn tool_checklist_area(
    app: &mut OperationApp,
    ui: &mut Ui,
) {
    if let Some(preoperative_tool_ready) = app.get_preoperative_tool_ready() {
        if let Some(_) = &mut app.data { 
            ui.heading("Tool Checklist");
            ui.add_space(20.0);
            
            Frame::none()
            .fill(FORM_BACKGROUND)
            .rounding(20.0)
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.columns(1, |columns| {
                    columns[0].vertical_centered(|ui| {
                        app.build_table(ui, database::table::window::WindowTable::PreOperativeToolReady(Some(preoperative_tool_ready.clone())));
                    });
                });
            });
        }
    }
}
pub fn staff_list_area(
    app: &mut OperationApp,
    ui: &mut Ui,
) {
    if let Some(operation_staff_property) = app.get_staff_list() {
        Frame::none()
        .fill(FORM_BACKGROUND)
        .rounding(20.0)
        .inner_margin(20.0)
        .show(ui, |ui| {
            ui.columns(1, |columns| {
                columns[0].vertical_centered(|ui| {
                    app.build_table(ui, database::table::window::WindowTable::PreoperativeStaffList(Some(operation_staff_property.clone())));
                });
            });
        });
    }
}

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