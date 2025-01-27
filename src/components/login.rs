use std::{cell::RefCell, rc::Rc};

use crate::views;

use super::View;

pub(crate) enum State {
    Waiting,
    Error,
    Valid,
    Default
}
pub(crate) struct Login {
    state: State,
    email: String,
    password: String,
    views: Rc<RefCell<views::View>>
}

impl Login {
    pub fn new(views: Rc<RefCell<views::View>>) -> Login {
        Self {
            state: State::Default,
            email: String::new(),
            password: String::new(),
            views
        }
    }
    pub fn get_view(&self) -> views::View {
        *self.views.borrow()
    }
    pub fn show(&mut self, ctx: &egui::Context) {
        self.ui(ctx);
    }
}
impl super::View for Login {
    fn ui(&mut self, ctx: &egui::Context) {
        let width = 500.0;
        let height = 250.0;
        
        egui::Window::new("STAFF LOGIN")
        .default_open(true)
        .resizable(true)
        .collapsible(false)
        .fixed_size([width, height])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            let mut visuals = ui.visuals().clone();
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(200, 100, 100); 
            
            let color: egui::Color32 = match self.state {
                State::Waiting => {egui::Color32::from_hex("#FFA652").unwrap()},
                State::Error => {egui::Color32::RED},
                State::Valid => {egui::Color32::GREEN},
                State::Default => {egui::Color32::TRANSPARENT},
            };
            
            let mut visuals = ui.visuals().clone();
            visuals.widgets.hovered.bg_fill = color;
            visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, color);
            
            ui.horizontal(|ui| {
                egui::Frame::none()
                .fill(color)
                .rounding(5.0)     
                .inner_margin(egui::Margin::same(1.0))
                .show(ui, |ui| {
                    ui.label("email ");
                    ui.ctx().set_visuals(visuals.clone());
                    ui.text_edit_singleline(&mut self.email);
                });
            });
            
            ui.horizontal(|ui| {
                egui::Frame::none()
                .fill(color)
                .rounding(5.0)     
                .inner_margin(egui::Margin::same(1.0))
                .show(ui, |ui| {
                    ui.label("password ");
                    ui.ctx().set_visuals(visuals.clone());
                    ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                });
            });
            
            if ui.button("login").clicked() {
                println!("hello");
                let mut views = self.views.borrow_mut();
                *views = views::View::OperationSelect;
            }
        });
    }
}
