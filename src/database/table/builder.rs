use egui::{Button, Color32, Label, RichText, Sense, Ui};
use egui_extras::{TableBuilder, Column};

use super::{data::TableData, window::WindowTable};

pub trait BuildTable {
    fn build_table<'a>(ui: &'a mut Ui, window_table: WindowTable, data: &mut TableData);
}

impl BuildTable for TableData {
    fn build_table<'a>(ui: &'a mut Ui, window_table: WindowTable, data: &mut TableData) -> () {
        let table_return = if let WindowTable::OperationSelect(Some(s)) = &window_table {
            let tbl = TableBuilder::new(ui)
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .header(20.0, |mut header| {
                let headings = [
                    "OPERATION",
                    "STATUS",
                    "ROOM",
                    "ROOM CODE"
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
                        row.col(|ui| {
                            ui.add(Label::new(content.operation_label.clone()));
                        });
                        row.col(|ui| {
                            ui.add(Label::new(content.operation_status.clone()));
                        });
                        row.col(|ui| {
                            ui.add(Label::new(content.room.clone()));
                        });
                        row.col(|ui| {
                            ui.add(Label::new(content.room_code.clone()));
                        });
                    });
                }
            });
        };
        table_return
    }
}