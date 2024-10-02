use egui::{epaint, Color32, Frame, Margin, Stroke, Ui};

pub enum State {
    Waiting,
    Error,
    Valid,
    Default
}
pub enum Category {
    Frame,
    
}

pub(crate) fn input(ui: &mut Ui, data: &mut String, color: Color32, category: Category) {

    let mut visuals = ui.visuals().clone();
    
    // Customize the hover color
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(200, 100, 100); 

    match category {
        Category::Frame => {
            Frame::none()
            .fill(color) // Red background
            .rounding(5.0)      // Optional: Rounded corners
            .inner_margin(egui::Margin::same(1.0)) // Optional: Inner padding
            .show(ui, |ui| {
                let mut visuals = ui.visuals().clone();
                visuals.widgets.hovered.bg_fill = color;
                visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, color);
                ui.text_edit_singleline(data);
                ui.ctx().set_visuals(visuals);
            });
            
        },
    }
}

