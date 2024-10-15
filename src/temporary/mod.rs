use serde::{Deserialize, Serialize};

use crate::ws::receive::TableTarget;
#[derive(Deserialize, Debug, Serialize, Clone)]
struct Row {
    id: i32,
    table: TableTarget,
}
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Temporary {
    database: Vec<Row>
}