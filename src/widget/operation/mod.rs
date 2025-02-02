use super::patient::Patient;

pub(crate) mod operation_select;

pub(crate) struct Operation {
    id: i32,
    name: String,
    room: String,
    patient: Patient
}