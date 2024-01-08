use std::{env, io};
use error::ArchiveError;
use modules::webpage::Link;
use std::fs::File;
use std::time::Instant;
use crate::app::{main_app::run_app, single_app::get_soks, offline_app::get_soks_offline, interactive_app::interactive, mf_app::mf_app};

mod error;
mod modules;
mod parser;
mod utils;
mod xl;
mod app;
mod scraper;
mod tests;
/// # Setup
/// Ensures that all the *.log files are available, before exec.
/// Can throw an io::Error
fn setup() -> io::Result<()> {
    File::create("sok.log")?;
    File::create("log.log")?;
    File::create("failed_sok.log")?;
    Ok(())
}

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), ArchiveError> {
    if let Err(e) = setup() {
        eprintln!("Error during setup: {}", e);
    }
    let mut args: Vec<String> = env::args().collect();

    if args.contains(&"-cli".to_string()) {
        interactive().await;
        return Ok(());
    }

    args.remove(0); // First argument is path to the exe
    if args.len() == 0 {
        args.append(
            &mut vec![
                "https://medienorge.uib.no/statistikk/medium/avis".to_string(),
                "https://medienorge.uib.no/statistikk/medium/fagpresse".to_string(),
                "https://medienorge.uib.no/statistikk/medium/ukepresse".to_string(),
                "https://medienorge.uib.no/statistikk/medium/boker".to_string(),
                "https://medienorge.uib.no/statistikk/medium/radio".to_string(),
                "https://medienorge.uib.no/statistikk/medium/fonogram".to_string(),
                "https://medienorge.uib.no/statistikk/medium/tv".to_string(),
                "https://medienorge.uib.no/statistikk/medium/kino".to_string(),
                "https://medienorge.uib.no/statistikk/medium/video".to_string(),
                "https://medienorge.uib.no/statistikk/medium/ikt".to_string()
            ]
        );
    }

    if args.contains(&"-err".to_string()) {
        // TODO: Add stop on first error code
    }

    if args.contains(&"-mf".to_string()) {
        println!("Archiving many-form-soks");
        let time_start = Instant::now();
        let r = mf_app(vec![
            "https://medienorge.uib.no/statistikk/medium/avis".to_string(),
            "https://medienorge.uib.no/statistikk/medium/fagpresse".to_string(),
            "https://medienorge.uib.no/statistikk/medium/ukepresse".to_string(),
            "https://medienorge.uib.no/statistikk/medium/boker".to_string(),
            "https://medienorge.uib.no/statistikk/medium/radio".to_string(),
            "https://medienorge.uib.no/statistikk/medium/fonogram".to_string(),
            "https://medienorge.uib.no/statistikk/medium/tv".to_string(),
            "https://medienorge.uib.no/statistikk/medium/kino".to_string(),
            "https://medienorge.uib.no/statistikk/medium/video".to_string(),
            "https://medienorge.uib.no/statistikk/medium/ikt".to_string()
        ]).await;
        let time_end = Instant::now();
        let duration = time_end - time_start;
        println!("That took: {} seconds", duration.as_secs());
        return r;
    }

    if args.contains(&"-offline".to_string()) {
        let time_start = Instant::now();
        let r = get_soks_offline().await;
        let time_end = Instant::now();
        let duration = time_end - time_start;
        println!("That took: {} seconds", duration.as_secs());
        return r;
    }

    if args.contains(&"-sok".to_string()) {
        let mut j = 0;
        for i in 0..args.len() {
            if args.get(i).unwrap() == &"-sok".to_string() {
                j = i;
                break;
            }
        }

        args.swap_remove(j);

        match parse_args(args) {
            Ok(links) => get_soks(links).await?,
            Err(err) => println!("Failed parsing args: {}", err),
        }
        
        return Ok(());
    }


    let time_start = Instant::now();
    let r = run_app(args).await;
    let time_end = Instant::now();
    let duration = time_end - time_start;
    println!("That took: {} seconds", duration.as_secs());

    println!("Press any key to continue");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    r
}

fn parse_args(args: Vec<String>) -> Result<Vec<Link>, ArchiveError> {
    let mut links = Vec::new();
    for arg in args {
        let link = Link::new(arg.clone());
        if link.is_external() || link.is_partial() {
            println!("Invalid URL: {}", arg);
            return Err(ArchiveError::InvalidURL);
        }
        links.push(link);
    }

    Ok(links)
}
