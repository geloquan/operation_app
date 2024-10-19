pub mod menu;

#[derive(Clone)]
pub enum State {
    Preoperation(menu::preoperative::Menu),
    Intraoperation,
    Postoperation
}