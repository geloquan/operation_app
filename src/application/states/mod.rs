use crate::component::design;

use super::field;

mod preoperative;

pub struct Login {
    pub field: field::Login,
    pub state: design::State,
}

#[derive(Default)]
pub enum Category {
    #[default]
    Uninitialize,
    Preoperative
}