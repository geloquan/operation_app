
use crate::widget::login as WidgetLogin;
use crate::components::login as ComponentLogin;

pub(crate) struct Login;

impl Login {
    pub fn show(ctx: &egui::Context, widget_state: &mut WidgetLogin::Login) {
        let width = 500.0;
        let height = 250.0;
    
        egui::Window::new("STAFF LOGIN")
        .default_open(true)
        .resizable(true)
        .collapsible(false)
        .fixed_size([width, height])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ComponentLogin::Login::ui(ui, widget_state);
        });
    }    
}