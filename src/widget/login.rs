use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Default, Clone, Serialize)]
pub(crate) struct Login {
    pub state: State,
    pub email: String,
    pub password: String,
}

impl Login {
    pub fn process() {
        
    }
}
