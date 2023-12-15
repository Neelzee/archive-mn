use std::fs::OpenOptions;
use std::io::prelude::*;
use tokio::sync::mpsc::Receiver;

use crate::error::ArchiveError;

pub async fn error_logger(mut error_channel: Receiver<ArchiveError>) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.log")
        .expect("log.log File should exist");

    while let Some(error) = error_channel.recv().await {
        let res = writeln!(file, "{}", error);
        if res.is_err() {
            println!("Failed logging error: {}", res.unwrap_err());
        }
    }
    
}