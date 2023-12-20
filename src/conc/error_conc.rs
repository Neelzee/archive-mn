use std::{io::Write, fs::OpenOptions};
use tokio::sync::mpsc::Receiver;

/// Logger
pub async fn error_conc(
    mut error: Receiver<String>
) {
    println!("Error Conc Start");
    while let Some(l) = error.recv().await {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("failed_sok.log")
            .expect("failed_sok.log File should exist");

        if let Err(err) = writeln!(file, "{}", &l) {
            eprintln!("Failed logging: {}, due to Error: {}", l, err);
        }

    }
    println!("Error Conc End");
}