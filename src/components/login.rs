use std::{cell::RefCell, rc::Rc, thread};

use egui::Ui;

use crate::{services::{app::{self, App}, middleman}, views, widget::login::{self, State}};

use crate::widget::login as WidgetLogin;

pub(crate) struct Login;

impl Login {
    pub fn ui(ui: &mut Ui, widget_login: &mut WidgetLogin::Login, app: Rc<RefCell<app::App>>) {
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
            app.borrow().send(crate::services::UiToMiddleman::LoginAuthentication(widget_login.clone()));
            //thread.borrow().send(crate::services::middleman::Get::Operation);
        }
    }
}