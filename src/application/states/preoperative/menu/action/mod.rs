use std::sync::mpsc::Sender;

use egui::{
    Context, 
    Frame, 
    Margin, 
    RichText, 
    Style, 
    Ui
};
use egui_extras::{Column, TableBuilder};

use crate::{
    action::{Actions, Preoperation}, application::operation::menu::preoperative::{
            self, 
            action::{NewEquipmentRequirement, RemoveEquipmentRequirement}, 
            Action
        }, database::{
        self, 
        table::{
            data::TableData, 
            ui_builder::BuildTable
        }
    }, OperationApp, FORM_BACKGROUND, FORM_TEXT_SIZE
};

mod options;
mod area;

pub use options::*;
pub use area::*;