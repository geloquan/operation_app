pub mod login;
pub mod operation_select;

#[derive(Clone, Copy)]
pub(crate) enum State {
    Preoperation,
    Intraoperation,
    Postoperation
}

#[derive(Clone, Copy)]
pub(crate) enum View {
    Login,
    OperationSelect,
    Operation(State)
}
impl Default for View {
    fn default() -> Self {
        View::Login
    }
}
impl View {
    fn next(&mut self) {
        *self = View::OperationSelect;
    }
}