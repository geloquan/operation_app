use crate::OperationApp;

pub trait Local {
    fn dispatch();
}

impl Local for OperationApp {
    fn dispatch() {
        
    }
}