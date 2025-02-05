use operation::operation_select::OperationSelect;


#[derive(Default)]
pub(crate) struct Widget {
    pub login: login::Login,
    pub operation_select: OperationSelect
}

pub(crate) mod login;

pub(crate) mod operation;

pub(crate) mod patient;