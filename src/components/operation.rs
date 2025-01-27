

pub(crate) struct Operation;

enum Type {
    SelectTable
}

pub mod select {

    //pub mod LoginForm {
    //    use crate::components::{login::Login, View};
//
    //    pub(crate) enum State {
    //        Waiting,
    //        Error,
    //        Valid,
    //        Default
    //    }
    //    pub(crate) struct LoginForm {
    //        state: State,
    //        email: String,
    //        password: String,
    //        v
    //    }
    //    
    //    impl Default for LoginForm {
    //        fn default() -> Self {
    //            Self {
    //                state: State::Default,
    //                email: "".to_string(),
    //                password: "".to_string(),
    //            }
    //        }
    //    }
    //    impl LoginForm {
    //        pub fn new(state: State, email: String, password: String) -> Self {
    //            Self {
    //                state,
    //                email,
    //                password
    //            }
    //        }
    //    }
    //    impl crate::components::View for LoginForm {
    //        fn ui(&mut self, ctx: &egui::Context) {
    //            let width = 500.0;
    //            let height = 250.0;
    //        
    //            egui::Window::new("STAFF LOGIN")
    //            .default_open(true)
    //            .resizable(true)
    //            .collapsible(false)
    //            .fixed_size([width, height])
    //            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
    //            .show(ctx, |ui| {
    //                ui.columns(1, |columns| {
    //                    columns[0].vertical_centered(|ui| {
    //                        let tbl = egui_extras::TableBuilder::new(ui)
    //                        .column(egui_extras::Column::auto().resizable(true).at_least(150.0).at_most(200.0))
    //                        .column(egui_extras::Column::auto().resizable(true).at_least(150.0).at_most(200.0))
    //                        .column(egui_extras::Column::auto().resizable(true).at_least(150.0).at_most(200.0))
    //                        .auto_shrink(true)
    //                        .striped(true)
    //                        .max_scroll_height(500.0)
    //                        .header(20.0, |mut header| {
    //                            let headings = [
    //                                "EQUIPMENT",
    //                                "STATUS",
    //                                "",
    //                            ];                
    //                            for title in headings {
    //                                header.col(|ui| {
    //                                    ui.horizontal_centered(|ui|{
    //                                        ui.heading(title);
    //                                    });
    //                                });
    //                            }
    //                        })
    //                        .body(|mut body| {
    //            
    //                        });
    //                    });
    //                });
    //            });
    //        }
    //    }
    //}
}