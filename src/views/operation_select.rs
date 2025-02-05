use std::cell::RefCell;
use std::rc::Rc;

use egui::Context;

pub(crate) struct OperationSelect;

use crate::services::app;
use crate::widget::operation::operation_select::OperationSelect as WidgetOperationSelect;
use crate::components::operation_select::OperationSelect as ComponentOperationSelect;

impl OperationSelect {
    pub fn show(ctx: &Context, widget_state: &WidgetOperationSelect, app: Rc<RefCell<app::App>>) {
        let width = 500.0;
        let height = 250.0;
        egui::Window::new("SELECT OPERATION")
        .default_open(true)
        .resizable(true)
        .collapsible(false)
        .fixed_size([width, height])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ComponentOperationSelect::ui(ui, widget_state, app);
        });
    }
}