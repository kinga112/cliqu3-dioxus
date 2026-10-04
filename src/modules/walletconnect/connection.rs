use reown_relay_client::{
    error::ClientError,
    websocket::{CloseFrame, ConnectionHandler, PublishedMessage},
};
use tokio::sync::mpsc::Sender;

pub struct Handler {
    name: &'static str,
    tx: Sender<PublishedMessage>,
}

impl Handler {
    pub fn new(name: &'static str, tx: Sender<PublishedMessage>) -> Self {
        Self { name, tx }
    }
}

impl ConnectionHandler for Handler {
    fn connected(&mut self) {
        println!("[{}] connection open", self.name);
    }

    fn disconnected(&mut self, frame: Option<CloseFrame>) {
        println!("[{}] connection closed: frame={frame:?}", self.name);
    }

    fn message_received(&mut self, message: PublishedMessage) {
        // println!(
        //     "[{}] message received from connection handler: tag={}",
        //     self.name, message.tag
        // );
        let tx = self.tx.clone();
        tokio::spawn(async move {
            if let Err(e) = tx.send(message).await {
                eprintln!("Failed to send message through channel: {}", e);
            }
        });
    }

    fn inbound_error(&mut self, error: ClientError) {
        println!("[{}] inbound error: {error}", self.name);
    }

    fn outbound_error(&mut self, error: ClientError) {
        println!("[{}] outbound error: {error}", self.name);
    }
}
