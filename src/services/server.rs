
pub(crate) struct Comms {
    receiver: tokio::sync::mpsc::Receiver<String>,
    sender: tokio::sync::mpsc::Sender<String>
}

impl super::Init for Comms {
    fn init(&self) -> Result<(), &'static str> {
        let (sender, receiver) = tokio::sync::mpsc::channel(32);
        
        self.receiver = receiver;
        self.sender = sender;

        Ok(())
    } 
}