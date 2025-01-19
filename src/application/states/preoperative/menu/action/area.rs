use std::sync::mpsc::Sender;

use egui::{
    Context, Frame, Margin, RichText, Style, TextEdit, Ui
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
            data::TableData, public::Staff, ui_builder::BuildTable
        }
    }, OperationApp, FORM_BACKGROUND, FORM_TEXT_SIZE
};

use regex::Regex;

pub fn add_requirement_area(
    s: &mut Option<NewEquipmentRequirement>, 
    data: &TableData, 
    ui: &mut egui::Ui, 
    ctx: &Context, 
    app_tx: &Sender<Actions>,
    operation_id: &i32
) {
    if let Some(s) = s {
        match data.equipment.read() {
            Ok(equipments) => {
                Frame::none()
                .rounding(20.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.columns(1, |columns| {
                        columns[0].vertical_centered_justified(|ui| {
                            ui.set_width(700.0);
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
                                ui.label(RichText::new("Qty: ").size(FORM_TEXT_SIZE));
                                ui.separator();
                                let mut style: Style = (*ctx.style()).clone();
                                style.spacing.icon_width = 32.0;
                                style.spacing.icon_spacing = 16.0;
                                ctx.set_style(style);

                                egui::ComboBox::from_label("")
                                .selected_text(&s.quantity.to_string()) 
                                .show_ui(ui, |ui| {
                                    for i in 1..10 {
                                        ui.selectable_value(&mut s.quantity, i.clone(), i.clone().to_string());
                                    }
                                });
                            });

                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new("To Claim: ").size(FORM_TEXT_SIZE));
                                ui.separator();
                                let mut style: Style = (*ctx.style()).clone();
                                style.spacing.icon_width = 32.0;
                                style.spacing.icon_spacing = 16.0;
                                ctx.set_style(style);
                                ui.checkbox(&mut s.to_claim, "");
                            });

                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new("Staff to claim: ").size(FORM_TEXT_SIZE));
                                ui.text_edit_singleline(&mut s.staff_search);
                                let mut style: Style = (*ctx.style()).clone();
                                style.spacing.icon_width = 32.0;
                                style.spacing.icon_spacing = 16.0;
                                ctx.set_style(style);

                                let staff: Vec<Staff> = data.staff.read().unwrap().clone();
        
                                let filtered_items: Vec<&Staff> = if s.staff_search.is_empty() {
                                    vec![]
                                } else if let Some(regex) = Regex::new(&mut s.staff_search).ok() {
                                    staff.iter().filter(|item| regex.is_match(&item.first_name.clone().unwrap_or("".to_string())) || regex.is_match(&item.last_name.clone().unwrap_or("".to_string())) || regex.is_match(&item.email.clone().unwrap_or("".to_string()))).collect()
                                } else {
                                    vec![]
                                };
                            
                                if filtered_items.is_empty() && !s.staff_search.is_empty() && s.to_claim_staff_id.is_none() {
                                    ui.label("Staff not found.");
                                } else {
                                    for staff in filtered_items {
                                        if ui.selectable_label(s.to_claim_staff_id.unwrap_or(0) == staff.id.unwrap_or(-1), format!("{} {} ({})", staff.first_name.clone().unwrap(), staff.last_name.clone().unwrap(), staff.email.clone().unwrap())).clicked() {
                                            s.to_claim_staff_id = staff.id.clone();
                                            s.staff_search = format!("{} {} ({})", staff.first_name.clone().unwrap(), staff.last_name.clone().unwrap(), staff.email.clone().unwrap());
                                        }
                                    }
                                }
                            });

                            ui.horizontal_wrapped(|ui| {
                                if ui.button(RichText::new("SUBMIT").size(FORM_TEXT_SIZE)).clicked() &&
                                s.name != "" {
                                    for equipment in equipments.iter() {
                                        if let Some(equipment_name) = &equipment.name {
                                            if *equipment_name == s.name {
                                                s.tool_id = equipment.id;

                                                s.operation_id = *operation_id;
                                                
                                                let _ = app_tx.send(
                                                    Actions::Preoperation(
                                                        Preoperation::AddNewEquipmentRequirement(
                                                            s.to_owned()
                                                        )
                                                    )
                                                );
                                            }
                                        };
                                    };
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
    ui: &mut egui::Ui, 
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

pub fn equipment_requested_area(
    app: &mut OperationApp, 
    ui: &mut Ui
) {
    if let Some(operation_staff_property) = app.equipment_requested_options() {
        Frame::none()
        .fill(FORM_BACKGROUND)
        .rounding(20.0)
        .inner_margin(20.0)
        .show(ui, |ui| {
            ui.columns(1, |columns| {
                columns[0].vertical_centered(|ui| {
                    app.build_table(ui, database::table::window::WindowTable::PreoperationEquipmentRequested(Some(operation_staff_property.clone())));
                });
            });
        });
    }
}