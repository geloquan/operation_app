use std::default;

#[derive(Default)]
pub(crate) struct Widget {
    pub login: login::Login
}

pub(crate) mod login {
    use std::{cell::RefCell, rc::Rc};

    use crate::views;

    pub(crate) enum State {
        Waiting,
        Error,
        Valid,
        Default
    }
    impl Default for State {
        fn default() -> Self {
            State::Default
        }
    }

    #[derive(Default)]
    pub(crate) struct Login {
        pub state: State,
        pub email: String,
        pub password: String,
    }
}