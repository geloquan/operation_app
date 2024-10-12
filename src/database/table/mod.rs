use std::fmt;
use serde::{Serialize, Deserialize};

pub mod join;
pub mod query;
pub mod data;
pub mod ui_builder;
pub mod window;
pub mod tree;
pub mod private;
pub mod public;
pub mod properties;

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum Tables {
    Equipment,
    Room,
    Tool,
    Staff,
    ToolReservation,
    ToolDesignatedRoom,
    ToolInspector,
    Patient,
    Operation,
    PatientWardRoom,
    PatientWardAssistant,
    OperationStaff,
    OperationTool,
    Alert,
    Frontdesk,
    AlertFrontdesk,
    AlertStaff,
    ActionLog
}