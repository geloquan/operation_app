use crate::application::forms::NewEquipmentRequirement;

#[derive(PartialEq)]
pub enum Menu {
    PreOperativeToolReady
}
#[derive(PartialEq)]
pub enum Action {
    AddRequirement(Option<NewEquipmentRequirement>)
}