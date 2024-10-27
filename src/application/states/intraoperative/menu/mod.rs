use ewebsock::WsSender;

use crate::application::{authenticate::StaffCredential, operation::menu::intraoperative::{Init, Menu}};


pub fn init(
    ui:&mut egui::Ui, 
    menu: &mut Menu, 
    operation: &mut Init, 
    operation_id: &Option<i32>, 
    sender: &mut WsSender, 
    staff_credential: &Option<StaffCredential>
) {
    todo!();
}