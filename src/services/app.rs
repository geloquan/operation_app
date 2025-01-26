
pub(crate) struct Comms {
    receiver: ewebsock::WsReceiver,
    sender: ewebsock::WsSender,
}

impl super::Init for Comms {
    fn init() -> Result<Self, &'static str> {
        let (sender, receiver) = ewebsock::connect("ws://127.0.0.15:8080", ewebsock::Options::default()).unwrap();
        
        Ok(Self { receiver, sender })
    }
}