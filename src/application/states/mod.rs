use crate::component::design;

use super::field;

pub(crate) mod preoperative;

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