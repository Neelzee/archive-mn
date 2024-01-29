use tokio::sync::broadcast::{Receiver, Sender};

pub async fn requester(rec: Receiver<String>, snd: Sender<String>) {
    
}