use std::{cell::RefCell, rc::Rc};

use crate::services::app::App;

pub(crate) mod login;

pub(crate) mod operation;

pub(crate) mod operation_select;

enum Components {
    Login,
    Operation(operation::Operation),

}

struct WindowConfig {

}