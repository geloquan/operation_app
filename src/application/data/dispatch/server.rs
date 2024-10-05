use crate::OperationApp;

pub trait Server {
    fn dispatch();
}

impl Server for OperationApp {
    fn dispatch() {
        
    }
}