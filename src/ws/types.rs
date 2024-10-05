
use serde::{Deserialize, Serialize};

use crate::database;

#[derive(Deserialize, Serialize)]
pub struct Update {
    pub id: i32,
    pub new_row_data: String
}