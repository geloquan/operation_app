use egui::{Ui, Frame, epaint, Color32, Margin};

pub enum State {
    Waiting,
    Error,
    Valid,
    Default
}
pub enum Category {
    Frame,
    
}

pub(crate)  fn input(ui: &mut Ui, data: &mut String, color: Color32, category: Category) {
    match category {
        Category::Frame => {
            Frame::none()
            .fill(color) // Red background
            .rounding(5.0)      // Optional: Rounded corners
            .inner_margin(egui::Margin::same(1.0)) // Optional: Inner padding
            .show(ui, |ui| {
                ui.text_edit_singleline(data);
            });
            
        },
    }
}

