pub(crate) mod login;

pub(crate) mod operation;

enum Components {
    Login,
    Operation(operation::Operation),

}

trait View {
    fn ui(&mut self, ctx: &egui::Context);
}

struct WindowConfig {

}