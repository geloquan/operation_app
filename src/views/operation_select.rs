use egui::Context;

use crate::components::operation_select::OperationSelect as OperationSelectComponent;

pub(crate) struct OperationSelect;

impl OperationSelect {
    pub fn show(ctx: &Context) {
        let width = 500.0;
        let height = 250.0;
        egui::Window::new("STAFF LOGIN")
        .default_open(true)
        .resizable(true)
        .collapsible(false)
        .fixed_size([width, height])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            OperationSelectComponent::ui(ui);
        });
    }
}