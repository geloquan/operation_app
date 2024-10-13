use serde::{Deserialize, Serialize};

use crate::database::table::private;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum Actions {
    OperationToolOnSiteToggle(private::OperationToolOnSiteToggle)
}