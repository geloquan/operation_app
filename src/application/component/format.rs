use egui::Ui;

pub fn get_width_from_text(ui: &Ui, text: String) -> f32 {
    let text_size = ui.fonts(|fonts| fonts.font_image_size());
    println!("textsize {:?}", text_size);
    println!("text {:?}", text);
    let total = text_size[1] as f32 * (text.len() as f32 / 4.0);
    println!("total {:?}", total);
    total
}