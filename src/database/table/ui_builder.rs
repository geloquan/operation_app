use egui::{Button, Color32, Frame, Label, Margin, RichText, Sense, Ui};
use egui_extras::{TableBuilder, Column};

use super::private::OperationToolOnSiteToggle;
use super::public::OperationTool;
use super::{data::TableData, window::WindowTable};

use crate::action;
use crate::ws::receive::TableTarget;
use crate::application::data::{self, dispatch};
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
                                            let toggle = OperationToolOnSiteToggle{tool_id: content.operation_tool_id, operation_id: self.operation_id.unwrap_or_else(|| 0), on_site_value: false};
                                            println!("to false {:?}", toggle);
                                            self.action(action::Actions::OperationToolOnSiteToggle(toggle));
                                        }
                                    },
                                    false => {
                                        if ui.checkbox(&mut false, "").interact(Sense::click()).clicked() { 
                                            let toggle = OperationToolOnSiteToggle{tool_id: content.operation_tool_id, operation_id: self.operation_id.unwrap_or_else(|| 0), on_site_value: true};
                                            println!("to true {:?}", toggle);
                                            self.action(action::Actions::OperationToolOnSiteToggle(toggle));
                                        }
                                    },
                                }
                            });
                        });
                    }
                });
            
            };
            table_return
        });
    }
}