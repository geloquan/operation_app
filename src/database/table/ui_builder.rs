use egui::{Label, Sense, Ui, Window};
use egui_extras::{TableBuilder, Column};

use super::private::OperationToolOnSiteToggle;
use super::window::WindowTable;

use crate::{action, application};
use crate::{application::data::dispatch::Dispatch, OperationApp};

pub trait BuildTable {
    fn build_table<'a>(&mut self, ui: &mut Ui, window_table: WindowTable);
}

impl BuildTable for OperationApp {
    fn build_table<'a>(&mut self, ui: &mut Ui, window_table: WindowTable) -> () {
        ui.vertical(|ui| {
            let table_return = if let WindowTable::OperationSelect(Some(s)) = &window_table {
                let tbl = TableBuilder::new(ui)
                .column(Column::auto().resizable(true).at_least(60.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(60.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(60.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(60.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(60.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(60.0).at_most(200.0))
                .auto_shrink(true)
                .striped(true)
                .header(20.0, |mut header| {
                    let headings = [
                        "OPERATION",
                        "STATUS",
                        "PATIENT NAME",
                        "ROOM",
                        "ROOM CODE",
                        ""
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
                        body.row(30.0, |mut row| {
                            row.col(|ui: &mut Ui| {
                                ui.add(Label::new(content.operation_label.clone()));
                            });
                            row.col(|ui| {
                                ui.add(Label::new(content.operation_status.clone()));
                            });
                            row.col(|ui| {
                                ui.add(Label::new(content.patient_full_name.clone()));
                            });
                            row.col(|ui| {
                                ui.add(Label::new(content.room.clone()));
                            });
                            row.col(|ui| {
                                ui.add(Label::new(content.room_code.clone()));
                            });
                            row.col(|ui| {
                                if ui.button("    ⮩    ").clicked() {
                                    self.operation_id = Some(content.operation_id);
                                    match content.operation_status.as_str() {
                                        "Pre-Operative" => {
                                            self.operation_state = Some(application::operation::State::Preoperation(application::operation::menu::preoperative::Menu::default()))
                                        },
                                        _ => {

                                        }
                                    }

                                }
                            });
                        });
                    }
                });
            } else if let WindowTable::PreOperativeToolReady(Some(s)) = &window_table {
                let tbl = TableBuilder::new(ui)
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .auto_shrink(true)
                .striped(true)
                .max_scroll_height(300.0)
                .header(20.0, |mut header| {
                    let headings = [
                        "EQUIPMENT",
                        "STATUS",
                        "ON SITE",
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
                        body.row(30.0, |mut row| {
                            row.col(|ui: &mut Ui| {
                                ui.add(Label::new(content.equipment_name.clone()));
                            });
                            row.col(|ui| {
                                ui.add(Label::new(content.tool_status.clone().to_string()));
                            });
                            row.col(|ui| {
                                match content.on_site {
                                    true => {
                                        if ui.checkbox(&mut true, "").interact(Sense::click()).clicked() {
                                            let toggle = OperationToolOnSiteToggle{tool_id: content.operation_tool_id, operation_id: self.operation_id.unwrap_or_else(|| 0), on_site_value: false, operation_tool_id: content.operation_tool_id};
                                            let _ = self.app_tx.send(
                                                action::Actions::Preoperation(
                                                    action::Preoperation::ToolOnSiteToggle(toggle)
                                                )
                                            );
                                        }
                                    },
                                    false => {
                                        if ui.checkbox(&mut false, "").interact(Sense::click()).clicked() { 
                                            let toggle = OperationToolOnSiteToggle{tool_id: content.operation_tool_id, operation_id: self.operation_id.unwrap_or_else(|| 0), on_site_value: true, operation_tool_id: content.operation_tool_id};
                                            let _ = self.app_tx.send(
                                                action::Actions::Preoperation(
                                                    action::Preoperation::ToolOnSiteToggle(toggle)
                                                )
                                            );
                                        }
                                    },
                                }
                            });
                        });
                    }
                });
            } else if let WindowTable::PreoperativeStaffList(Some(staff_list)) = &window_table {
                let tbl = TableBuilder::new(ui)
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .auto_shrink(true)
                .striped(true)
                .max_scroll_height(300.0)
                .header(20.0, |mut header| {
                    let headings = [
                        "Name",
                        "Email",
                        "Phone",
                        "Role",
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
                    for content in staff_list {
                        body.row(30.0, |mut row| {
                            row.col(|ui: &mut Ui| {
                                ui.add(Label::new(content.full_name.clone()));
                            });
                            row.col(|ui: &mut Ui| {
                                ui.add(Label::new(content.email.clone()));
                            });
                            row.col(|ui: &mut Ui| {
                                ui.add(Label::new(content.phone.clone()));
                            });
                            row.col(|ui: &mut Ui| {
                                ui.add(Label::new(content.role.clone().to_string()));
                            });
                        });
                    }
                });
            } else if let WindowTable::PreoperationEquipmentRequested(Some(s)) = &window_table {
                let _ = TableBuilder::new(ui)
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                .column(Column::auto().resizable(true).at_least(150.0).at_most(200.0))
                 .auto_shrink(true)
                .striped(true)
                .max_scroll_height(300.0)
                .header(20.0, |mut header| {
                    let headings = [
                        "Name",
                        "Requested by"
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
                        body.row(30.0, |mut row| {
                            row.col(|ui: &mut Ui| {
                                ui.add(Label::new(content.equipment_name.clone()));
                            });
                            row.col(|ui: &mut Ui| {
                                ui.add(Label::new(content.staff_name.clone()));
                            });
                        });
                    }
                });
            };
            table_return
        });
    }
}