use std::{io::Write, fs::OpenOptions};
use tokio::sync::mpsc::Receiver;

/// Logger
pub async fn logger_conc(
    mut logger: Receiver<String>
) {
    println!("Logger Conc Start");
    while let Some(l) = logger.recv().await {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.log")
            .expect("log.log File should exist");

        if let Err(err) = writeln!(file, "{}", &l) {
            eprintln!("Failed logging: {}, due to Error: {}", l, err);
        }
    }
    println!("Logger Conc End");
}