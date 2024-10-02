use egui::{Button, Color32, Frame, Label, Margin, RichText, Sense, Ui};
use egui_extras::{TableBuilder, Column};

use super::{data::TableData, window::WindowTable};

use crate::OperationApp;

pub trait BuildTable {
    fn build_table<'a>(ui: &'a mut Ui, window_table: WindowTable, data: &mut TableData, operation_id: &mut Option<i32>);
}

impl BuildTable for TableData {
    fn build_table<'a>(ui: &'a mut Ui, window_table: WindowTable, data: &mut TableData, operation_id: &mut Option<i32>) -> () {
        let table_return = if let WindowTable::OperationSelect(Some(s)) = &window_table {
            let tbl = TableBuilder::new(ui)
            .column(Column::auto().resizable(true).at_most(150.0))
            .column(Column::auto().resizable(true).at_most(150.0))
            .column(Column::auto().resizable(true).at_most(150.0))
            .column(Column::auto().resizable(true).at_most(150.0))
            .column(Column::auto().resizable(true).at_most(150.0))
            .column(Column::auto().resizable(false).at_most(150.0))
            .auto_shrink(false)
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
                        ui.horizontal(|ui|{
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
                                *operation_id = Some(content.operation_id);
                            }
                        });
                    });
                }
            });
        };
        table_return
    }
}