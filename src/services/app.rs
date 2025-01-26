
pub(crate) struct Comms {
    receiver: ewebsock::WsReceiver,
    sender: ewebsock::WsSender,
}

impl super::Init for Comms {
    fn init(&self) -> Result<(), &'static str> {
        (self.sender, self.receiver) = ewebsock::connect("ws://127.0.0.15:8080", ewebsock::Options::default()).unwrap();
        
        Ok(())
    }
}