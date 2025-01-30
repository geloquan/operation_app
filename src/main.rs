
use std::{cell::RefCell, rc::Rc, sync::{Arc, RwLock}, time::Duration};

use eframe::{egui, App};
use egui::Id;
use models::{operation::OperationModel, StreamDatabase};
use services::Service;
use tokio::time::sleep;
use widget::Widget;

mod services;

mod components;

mod views;

mod models;

mod widget;

struct OperationApp {
    view: Rc<RefCell<views::View>>,
    service: Service,
    widget: Widget
}
impl OperationApp {
    pub fn new(cc: &eframe::CreationContext<'_>, service: Service) -> OperationApp {

        let view =  Rc::new(RefCell::new(views::View::default()));
        let widget = Widget::default();
        
        OperationApp {
            view,
            service,
            widget,
            
        }
    }
}

impl App for OperationApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let view: views::View = self.view.borrow().clone();
        match view {
            views::View::Login => {
                egui::CentralPanel::default().show(ctx, |_ui| {
                    views::login::Login::show(ctx, &mut self.widget.login);
                });
            },
            views::View::OperationSelect => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    //let mut login_form = components::operation::select::LoginForm::LoginForm::new(components::operation::select::LoginForm::State::Default, "email".to_string(), "password".to_string());
                });
            },
            views::View::Operation(state) => {
                match state {
                    views::State::Preoperation => {
                        egui::TopBottomPanel::top(Id::new("side-top")).show(ctx, |ui| {
                            
                        });
                        egui::TopBottomPanel::bottom(Id::new("side-bottom")).show(ctx, |ui| {
                            
                        });
                        egui::SidePanel::left(Id::new("side-left")).show(ctx, |ui| {
                            
                        });
                        egui::CentralPanel::default().show(ctx, |ui| {

                        });
                        egui::SidePanel::right(Id::new("side-right")).show(ctx, |ui| {
                            
                        });
                        egui::TopBottomPanel::bottom(Id::new("side-bottom-bottom")).show(ctx, |ui| {
                            
                        });
                        egui::TopBottomPanel::top(Id::new("side-top-top")).show(ctx, |ui| {
                            
                        });
                    },
                    views::State::Intraoperation => {
                        
                    },
                    views::State::Postoperation => {
                        
                    },
                }
            },
        }
    }
    
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.service.middleman.borrow_mut().abort();
        self.service.server.borrow_mut().abort();
    }
    
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
    
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()
    
        // _visuals.window_fill() would also be a natural choice
    }
    
    fn persist_egui_memory(&self) -> bool {
        true
    }
    
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}
}

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();

    let stream_database: Arc<RwLock<models::StreamDatabase>> = Arc::new(
        RwLock::new(
            StreamDatabase::init(
                OperationModel::new(None))
            )
        );
    
    let service = services::Service::init(stream_database).await.unwrap();

    let _ = eframe::run_native("Operation", native_options, Box::new(|cc| {
        let app = OperationApp::new(cc,  service);
        Ok(Box::new(app))
    }));

    //service.middleman.borrow_mut().abort();
    //service.server.borrow_mut().abort();
    //
    //// Wait for cancellation to complete
    //sleep(Duration::from_secs_f32(1.0)).await;
//
    //let middleman = service.middleman.borrow_mut().is_finished();
    //let server = service.server.borrow_mut().is_finished();
    
    println!("last exit");

}