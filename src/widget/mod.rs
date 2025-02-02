
#[derive(Default)]
pub(crate) struct Widget {
    pub login: login::Login
}

pub(crate) mod login;

pub(crate) mod operation;

pub(crate) mod patient;