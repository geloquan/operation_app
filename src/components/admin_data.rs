enum State {
    Waiting,
    Error,
    Valid,
    Default
}
pub(crate) struct Login {
    state: State,
    email: String,
    password: String,
    closed: bool
}

impl Default for Login {
    fn default() -> Self {
        Self {
            state: State::Default,
            email: "".to_string(),
            password: "".to_string(),
            closed: true
        }
    }
}

impl Login {
    pub fn show(&mut self, ctx: &egui::Context) {
        let width = 500.0;
        let height = 250.0;
    
        egui::Window::new("STAFF LOGIN")
            .default_open(true)
            .resizable(true)
            .collapsible(self.closed)
            .fixed_size([width, height])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                let color: egui::Color32 = match self.state {
                    State::Waiting => {egui::Color32::from_hex("#FFA652").unwrap()},
                    State::Error => {egui::Color32::RED},
                    State::Valid => {egui::Color32::GREEN},
                    State::Default => {egui::Color32::TRANSPARENT},
                };
                ui.horizontal(|ui| {
                    ui.label("email ");
                });
                ui.horizontal(|ui| {
                    ui.label("password ");
                });
                if ui.button("login").clicked() {
                    println!("hello");
                    self.closed = !self.closed;
                }
            });
    }    
}