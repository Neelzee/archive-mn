#![feature(let_chains)]
use crate::app::json_app::{self, jsonify_soks};
use crate::app::{
    interactive_app::interactive, main_app::run_app, mf_app::mf_app, offline_app::get_soks_offline,
    single_app::get_soks,
};
use crate::utils::constants::{MEDIUM, VALID_SOKS};

use error::ArchiveError;
use itertools::Itertools;
use lazy_static::lazy_static;
use modules::webpage::Link;
use rand::Rng;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::sync::Mutex;
use std::time::Instant;
use std::{env, io};

mod app;
mod error;
mod modules;
mod parser;
mod scraper;
mod tests;
mod utils;
mod xl;

lazy_static! {
    pub static ref CHECKED_SOK_ID: Mutex<HashMap<usize, bool>> = Mutex::new(HashMap::new());
    pub static ref CHECKED_SOK_TITLE: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
    pub static ref ALLOW_DUPS: Mutex<bool> = Mutex::new(true);
}

/// # Setup
/// Ensures that all the *.log files are available, before exec.
/// Can throw an io::Error
fn setup() -> io::Result<()> {
    File::create("sok.log")?;
    File::create("log.log")?;
    File::create("failed_sok.log")?;

    let file = OpenOptions::new().read(true).open("skip.log")?;

    let reader = BufReader::new(file);

    if let Ok(mut map_title) = CHECKED_SOK_TITLE.lock()
        && let Ok(mut map_id) = CHECKED_SOK_ID.lock()
    {
        for line in reader.lines() {
            let line = line?;

            let mut parts = line.split(";").collect::<Vec<&str>>();

            if parts.len() == 2 {
                let raw_id = parts.pop().unwrap();
                let title = parts.pop().unwrap();
                map_title.insert(title.to_owned(), true);

                if let Ok(id) = raw_id.parse::<usize>() {
                    map_id.insert(id, true);
                }
            } else if parts.len() == 1 {
                let r = parts.pop().unwrap();
                match r.parse::<usize>() {
                    Ok(id) => {
                        map_id.insert(id, true);
                        continue;
                    }
                    Err(_) => {
                        map_title.insert(r.to_owned(), true);
                        continue;
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ArchiveError> {
    if let Err(e) = setup() {
        eprintln!("Error during setup: {}", e);
    }
    let mut args: Vec<String> = env::args().collect();

    if args.contains(&"-dup".to_string()) {
        let mut j = 0;
        for i in 0..args.len() {
            if args.get(i).unwrap() == &"-dup".to_string() {
                j = i;
                break;
            }
        }

        args.swap_remove(j);

        if let Ok(mut dups) = ALLOW_DUPS.lock() {
            *dups = false;
        }
    }

    if args.contains(&"-json".to_string()) {
        println!("Jsoning");
        return jsonify_soks(MEDIUM.into_iter().map(|l| Link::new(l.to_string())).collect_vec()).await;
    }

    if args.contains(&"-cli".to_string()) {
        interactive().await;
        return Ok(());
    }

    if args.contains(&"-rand".to_string()) {
        println!("Fetching random soks...");
        while let Some(n) = args.pop() {
            match n.parse::<u32>() {
                Ok(n) => {
                    let mut random_sok: Vec<Link> = Vec::new();
                    let mut rng = rand::thread_rng();
                    for _ in 0..n {
                        let random_index = rng.gen_range(0..VALID_SOKS.len());
                        random_sok.push(
                            Link::new(format!(
                                "/statistikk/medium/test/{}",
                                VALID_SOKS[random_index]
                            ))
                            .create_full(),
                        )
                    }
                    match get_soks(random_sok).await {
                        Ok(_) => continue,
                        Err(err) => {
                            eprintln!("{}", err);
                            continue;
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        println!("Finished fetching random sok, exiting...");
        return Ok(());
    }

    args.remove(0); // First argument is path to the exe
    if args.len() == 0 {
        args.append(&mut MEDIUM.into_iter().map(|e| e.to_string()).collect_vec());
    }

    if args.contains(&"-err".to_string()) {
        // TODO: Add stop on first error code
    }

    if args.contains(&"-mf".to_string()) {
        println!("Archiving many-form-soks");
        let time_start = Instant::now();
        let r = mf_app(MEDIUM.into_iter().map(|e| e.to_string()).collect_vec())
        .await;
        let time_end = Instant::now();
        let duration = time_end - time_start;
        println!("That took: {} seconds", duration.as_secs());
        return r;
    }

    if args.contains(&"-offline".to_string()) {
        let time_start = Instant::now();
        let r = get_soks_offline().await;
        if r.is_err() {
            eprintln!("{}", r.unwrap_err());
        }
        let time_end = Instant::now();
        let duration = time_end - time_start;
        println!("That took: {} seconds", duration.as_secs());
        return Ok(());
    }

    if args.contains(&"-sok".to_string()) {
        match parse_args(args.into_iter().filter(|e| e != "-sok").collect_vec()) {
            Ok(links) => get_soks(links).await?,
            Err(err) => println!("Failed parsing args: {}", err),
        }

        return Ok(());
    }

    if args.contains(&"-tmf".to_string()) {
        match parse_args(args.into_iter().filter(|e| e != "-tmf").collect_vec()) {
            Ok(links) => app::tmf::run_app(links).await?,
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
