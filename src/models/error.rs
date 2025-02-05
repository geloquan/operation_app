use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub(crate) enum Error {
    ServerError,
    Empty
}