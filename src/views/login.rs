
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
}

impl Default for Login {
    fn default() -> Self {
        Self {
            state: State::Default,
            email: "".to_string(),
            password: "".to_string(),
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
        .collapsible(false)
        .fixed_size([width, height])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
    
        });
    }    
}