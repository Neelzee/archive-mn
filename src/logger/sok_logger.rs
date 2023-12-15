use std::fs::OpenOptions;
use std::io::prelude::*;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::error::ArchiveError;

pub async fn sok_logger(mut sok_channel: Receiver<usize>, error_channel: Sender<ArchiveError>) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("sok.log")
        .expect("sok.log File should exist");

    while let Some(id) = sok_channel.recv().await {
        let res = writeln!(file, "{}", id);
        if res.is_err() {
            let r = error_channel.send(res.unwrap_err().into()).await;
            if r.is_err() {
                println!("Failed sending error to Error Logger: {}", r.unwrap_err());
            }
        }
    }
    
}