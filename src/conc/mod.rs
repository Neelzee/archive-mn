use tokio::spawn;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::channel;
use crate::conc::error_conc::error_conc;
use crate::conc::logger_conc::logger_conc;
use crate::conc::medium_conc::medium_conc;
use crate::conc::sok_conc::sok_conc;
use crate::modules::webpage::Link;
use crate::utils::constants::MEDIUM;
use futures::future::JoinAll;

mod sok_conc;
mod medium_conc;
mod logger_conc;
mod error_conc;

pub fn failed_sending<T>(err: Result<(), SendError<T>>, channel: String) {
    if err.is_err() {
        println!("Error: {} on channel: {}", err.unwrap_err(), channel);
    }
}

pub async fn main_conc() {
    let BUFFER = 32;

    let (log_send, log_recv) = channel::<String>(BUFFER);
    let (err_send, err_recv) = channel::<String>(BUFFER);

    let log_handler = spawn(
        logger_conc(log_recv)
    );

    let err_handler = spawn(
        error_conc(err_recv)
    );

    let mediums = MEDIUM.into_iter()
        .map(|e| Link::new(e.to_string()))
        .collect::<Vec<Link>>();

    let mut med_handlers = Vec::new();
    let mut sok_handlers = Vec::new();
    for l in mediums {
        let (link_send, link_recv) = channel::<String>(BUFFER);
        let s = l.clone().to_string();
        let mut itr = s.split("/").collect::<Vec<_>>().into_iter().rev();
        itr.next();
        let medium = itr.next().unwrap_or("unknown");

        med_handlers.push(
            spawn(
                medium_conc(
                    l,
                    link_send.clone(),
                    err_send.clone()
                )
            )
        );

        sok_handlers.push(
            spawn(
                sok_conc(
                    medium.to_string(),
                    link_recv,
                    log_send.clone(),
                    err_send.clone()
                )
            )
        )
    }
    match tokio::join!(
        med_handlers.into_iter().collect::<JoinAll<_>>(),
        sok_handlers.into_iter().collect::<JoinAll<_>>(),
        log_handler,
        err_handler,
    ) {
        (med, sok, Ok(()), Ok(())) => {
            for r in med {
                if r.is_err() {
                    eprintln!("Medium Error: {}", r.unwrap_err());
                }
            }

            for r in sok {
                if r.is_err() {
                    eprintln!("Sok Error: {}", r.unwrap_err());
                }
            }
        },
        (_, _, Err(e), _) => {
            eprintln!("Logger Error: {}", e);
        },
        (_, _, _, Err(e)) => {
            eprintln!("Error Error: {}", e);
        },
    }
}