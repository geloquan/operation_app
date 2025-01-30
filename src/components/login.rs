use std::{cell::RefCell, rc::Rc, thread};

use egui::Ui;

use crate::{services::app::App, views, widget::login::{self, State}};

use crate::widget::login as WidgetLogin;

pub(crate) struct Login;

impl Login {
    //pub fn show(&mut self, ctx: &egui::Context, thread: &mut Rc<RefCell<App>>) {
    //    self.ui(ctx, thread);
    //}
    pub fn ui(ui: &mut Ui, widget_login: &mut WidgetLogin::Login) {
        let mut visuals = ui.visuals().clone();
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(200, 100, 100); 
        
        let color: egui::Color32 = match widget_login.state {
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
                ui.text_edit_singleline(&mut widget_login.email);
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
                ui.add(egui::TextEdit::singleline(&mut widget_login.password).password(true));
            });
        });
        
        if ui.button("login").clicked() {
            println!("login clicked!");
            //thread.borrow().send(crate::services::middleman::Get::Operation);
        }
    }
}
//impl super::View for Login {
//    fn ui(&mut self, ctx: &egui::Context, thread: &mut Rc<RefCell<App>>) {
//        let width = 500.0;
//        let height = 250.0;
//        
//        egui::Window::new("STAFF LOGIN")
//        .default_open(true)
//        .resizable(true)
//        .collapsible(false)
//        .fixed_size([width, height])
//        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
//        .show(ctx, |ui| {
//
//        });
//    }
//}
