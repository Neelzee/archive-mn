use std::{env, fs::{self, OpenOptions}, io, collections::HashMap};
use error::ArchiveError;
use itertools::Itertools;
use logger::{error_logger::error_logger, sok_logger::sok_logger};
use modules::{sok::{SokCollection, self, Sok, Merknad}, webpage::{Webpage, Link}};
use parser::{wp::{get_sok_collection, get_kilde, get_metode}, parse_sok::parse_sok, get_text, get_merknad};
use reqwest::{Client, Request};
use ::scraper::Html;
use scraper::{get_html_content, html_scraper::scrape_html, request_scraper::execute_request};
use tokio::sync::mpsc::{self, Receiver, Sender, error::SendError};
use xl::save_sok;
use std::fs::File;
use std::io::prelude::*;

use crate::parser::medium::get_links_from_medium;

mod error;
mod logger;
mod modules;
mod parser;
mod scraper;
mod utils;
mod xl;
mod tests;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), ArchiveError> {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0); // First argument is path to the exe
    if args.len() == 0 {
        println!("Missing URL-argument.");
        return Err(ArchiveError::InvalidURL);
    }

    let buffer_size = 16;

    let medium_links: Vec <Link> = parse_args(args)?;

    let mut checked_soks: Vec<usize> = get_checked_soks(); 

    // Error channel
    let (err_chnl_snd, err_chnl_rec) = mpsc::channel::<ArchiveError>(buffer_size);
    let error_handle = tokio::spawn(error_logger(err_chnl_rec));

    // Sok logger
    let (sokl_chnl_snd, sokl_chnl_rec) = mpsc::channel::<usize>(buffer_size);
    let logger_handle = tokio::spawn(
        sok_logger(
            sokl_chnl_rec,
            err_chnl_snd.clone()
            )
        );

    // Links
    let (link_chnl_snd, link_chnl_rec) = mpsc::channel::<Link>(buffer_size);

    // OG Html channel
    let (html_chnl_snd, html_chnl_rec) = mpsc::channel::<String>(buffer_size);
    let scraper_handle = tokio::spawn(scrape_html(
        link_chnl_rec,
        html_chnl_snd,
        err_chnl_snd.clone()
    ));

    // Sok Html channel
    let (skht_chnl_snd, skht_chnl_rec) = mpsc::channel::<String>(buffer_size);

    // Request channel
    let (req_chnl_snd, req_chnl_rec) = mpsc::channel::<HashMap<String, String>>(buffer_size);
    let request_handle = tokio::spawn(execute_request(
        req_chnl_rec,
        skht_chnl_snd,
        err_chnl_snd.clone()
    ));

    // Sok saver
    let (soks_chnl_snd, soks_chnl_rec) = mpsc::channel::<(SokCollection, String)>(buffer_size);
    let request_handle = tokio::spawn(save_sokc(
        soks_chnl_rec,
        sokl_chnl_snd,
        err_chnl_snd.clone()
    ));


    let sok_handler = tokio::spawn(sok_parser(
        link_chnl_snd, // Sender<Link>
        html_chnl_rec, // Receiver<String>
        link_chnl_rec, // Receiver<Link>
        req_chnl_snd, // Sender<HashMap<String, String>>,
        skht_chnl_rec, // Receiver<String>,
        soks_chnl_snd, // Sender<(SokCollection, String)>,
        err_chnl_snd // Sender<ArchiveError>,
    ));

    Ok(())    
}



fn get_checked_soks() -> Vec<usize> {
    let file = File::open("sok.log").expect("sok.log File should exist");

    let reader = io::BufReader::new(file);

    return reader.lines()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.parse::<usize>().ok())
        .collect::<Vec<usize>>();
}


fn parse_args(args: Vec<String>) -> Result<Vec<Link>, ArchiveError> {
    let mut links = Vec::new();
    for arg in args {
        let link = Link::new(arg);
        if link.is_external() || link.is_partial() {
            return Err(ArchiveError::InvalidURL);
        }
        links.push(link);
    }

    Ok(links)
}

async fn sok_parser(
    lnk_chnl_snd: Sender<Link>,
    mut html_channel: Receiver<String>,
    mut sok_channel: Receiver<Link>,
    req_channel: Sender<HashMap<String, String>>,
    mut html_sok_channel: Receiver<String>,
    sok_saver: Sender<(SokCollection, String)>,
    error_channel: Sender<ArchiveError>,
) {
    let client = Client::default();
    while let Some(og_link) = sok_channel.recv().await {
        let url = og_link.to_string().clone();

        let mut id = 0;
        let mut medium = String::new();
        let url_c = url.clone();

        let mut str = url_c.split("/").collect::<Vec<_>>().into_iter().rev();

        if let Some(i) = str.next() {
            match i.parse::<usize>() {
                Ok(i) => {
                    id = i;
                }
                Err(err) => {
                    let r = error_channel.send(err.into()).await;
                    sending_error(r);
                },
            }
        }

        if let Some(m) = str.next() {
            medium = m.to_owned();
        }

        
        let mut sok_collection = SokCollection::new(id, medium.clone());
        let path = format!("src\\out\\{}", medium.clone());
        sending_error(lnk_chnl_snd.send(og_link).await);
        
        // Getting the mainpage
        if let Some(raw_content) = html_channel.recv().await {
            let content = Html::parse_document(&raw_content);
            let og_wp = Webpage::from_html(
                id,
                url.clone(),
                content.clone(),
                medium
            );

            // Get form
            let form = og_wp.get_forms();

            if form.is_err() {
                let err = form.unwrap_err();
                sending_error(error_channel.send(err).await);
                continue; // New link, this one did not work.
            }

            let form = form.unwrap();
            let request = client
                .post(url);

            // Get all sub-sok
            for args in form.combinations() {
                let mut form_data: HashMap<String, String> = HashMap::new();
                let mut title = String::new();
                for (k, (v, d)) in args {
                    title += &d;
                    title += " ";
                    form_data.insert(k, v);
                }
                form_data.insert("btnSubmit".to_string(), "Vis+tabell".to_string());
                
                title = title.split_whitespace().collect::<Vec<&str>>().join(" ");

                let req = request
                    .try_clone().expect("Should not be a stream")
                    .form(&form_data).build();

                if req.is_err() {
                    let err = req.unwrap_err();
                    sending_error(error_channel.send(err.into()).await);
                    continue; // New arg, this one did not work.
                }

                let req = req.unwrap();

                sending_error(req_channel.send(form_data).await);

                // Will only be None if something went wrong?
                if let Some(raw_html) = html_sok_channel.recv().await {
                    let html = Html::parse_document(&raw_html);
                    let res = parse_sok(html);
                    if res.is_err() {
                        let err = res.unwrap_err();
                        // TODO: Add info, specifying what sok this error belongs too
                        // TODO: Add info, specifying what sub-sok this error belongs too
                        sending_error(error_channel.send(err.into()).await);
                        continue; // New arg, this one did not work.
                    }

                    let mut sok = res.unwrap();

                    sok.header_title = title;
                    

                    sok_collection.add_sok(sok);
                }
            }

            // Kilde
            let res = get_kilde(&og_wp).await;

            if res.is_err() {
                let err = res.unwrap_err();
                let r = error_channel.send(err.into()).await;
                sending_error(r);
                continue; // New link, this one did not work.
            }

            for k in res.unwrap() {
                sok_collection.add_kilde(k.into());
            }

            // Text
            let res = get_text(&content);

            if res.is_err() {
                let err = res.unwrap_err();
                let r = error_channel.send(err.into()).await;
                sending_error(r);
                continue; // New link, this one did not work.
            }

            for t in res.unwrap() {
                sok_collection.add_text(t);
            }

            // Merkand
            let res = get_merknad(&content);

            if res.is_err() {
                let err = res.unwrap_err();
                let r = error_channel.send(err.into()).await;
                sending_error(r);
                continue; // New link, this one did not work.
            }
            sok_collection.add_merknad(Merknad { title: "Merknad".to_string(), content: res.unwrap() });

            // Metode
            let res = get_metode(&og_wp).await;

            if res.is_err() {
                let err = res.unwrap_err();
                let r = error_channel.send(err.into()).await;
                sending_error(r);
                continue; // New link, this one did not work.
            }
            
            for m in res.unwrap() {
                sok_collection.add_metode(m.into());
            }

            // Save
            sending_error(sok_saver.send((sok_collection, path)).await);
            
        }
    }
}

pub async fn save_sokc(
    mut sokc: Receiver<(SokCollection, String)>,
    // Sok Logger
    sok_logger_channel: Sender<usize>,
    error_channel: Sender<ArchiveError>
) {
    let mut save_count = 0;
    while let Some((sok, path)) = sokc.recv().await {
        let id = sok.id.clone();
        match save_sok(sok, &path) {
            Ok(_) => {
                save_count += 1;
                sending_error(sok_logger_channel.send(id).await);
            },
            Err(e) => {
                sending_error(error_channel.send(e).await);
            }, 
        }
    }
    println!("Save Sokc exited, saved: {} sok", save_count);
}

pub fn sending_error<T, E>(res: Result<T, SendError<E>>)
where
    T: std::fmt::Debug,
{
    if res.is_err() {
        println!("Failed sending error: {:?}", res.unwrap_err());
    }
}