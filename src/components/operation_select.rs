use std::{cell::RefCell, rc::Rc};

use egui::Ui;

use crate::{services::app::App, widget::operation::operation_select::OperationSelect as WidgetOperationSelect};

pub(crate) struct OperationSelect;

impl OperationSelect {
    pub fn ui(ui: &mut Ui, widget_state: &WidgetOperationSelect, app: Rc<RefCell<App>>) {
        ui.columns(1, |columns| {
            columns[0].vertical_centered(|ui| {
                let tbl = egui_extras::TableBuilder::new(ui)
                .column(egui_extras::Column::auto().resizable(true).at_least(150.0))
                .column(egui_extras::Column::auto().resizable(true).at_least(150.0))
                .column(egui_extras::Column::auto().resizable(true).at_least(150.0))
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
                });
            });
        });
    }
}