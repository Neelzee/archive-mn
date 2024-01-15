use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
    path::Path,
};

use reqwest::Client;
use scraper::Html;

use crate::{
    error::ArchiveError,
    modules::webpage::{Link, Webpage},
    scraper::get_html_content,
    utils::constants::ROOT_URL,
};

use super::offline_app::visit_dirs;

enum Cmd {
    DisplayForm,
    LoadFromUrl(Link),
    LoadFromIndex(usize),
    CreateRequest,
    DisplayHtml,
    Quit,
}

pub async fn interactive() {
    let mut buffer = String::new();
    let mut wp: Option<Webpage> = None;
    let mut ins: Vec<String> = Vec::new();
    loop {
        let stdin = io::stdin();
        if let Err(err) = stdin.read_line(&mut buffer) {
            eprintln!("Failed reading input: {}", err);
        }

        if let Some(cmd) = parse_input(&buffer) {
            match cmd {
                Cmd::DisplayForm => {
                    if wp.is_none() {
                        println!("Can't display form, load html first");
                        continue;
                    } else {
                        let w = wp.clone().unwrap();
                        let res = w.get_forms();
                        if res.is_err() {
                            eprintln!("Failed loading form: {}", res.unwrap_err());
                        } else {
                            let form = res.unwrap();
                            for op in form.options() {
                                op.show();
                            }
                        }
                    }
                }
                Cmd::LoadFromUrl(link) => match Webpage::from_link(link).await {
                    Ok(w) => {
                        if let Ok(title) = w.get_title() {
                            println!("Loaded: {}", title);
                        }

                        wp = Some(w);
                    }
                    Err(err) => eprintln!("Failed getting webpage from url: {}", err),
                },
                Cmd::DisplayHtml => {
                    let res = visit_dirs(Path::new("in"));

                    if res.is_err() {
                        eprintln!("Failed loading html from: in {}", res.unwrap_err());
                        continue;
                    }
                    let mut index = 0;
                    for path in res.unwrap() {
                        match load_html(path.clone()) {
                            Ok(w) => {
                                ins.push(path);
                                println!(
                                    "{} Medium: {}, Sok ID: {}, URL: {}",
                                    index,
                                    w.get_medium(),
                                    w.get_id(),
                                    w.get_url()
                                );
                                index += 1;
                            }
                            Err(_) => todo!(),
                        }
                    }
                    println!("/display");
                }
                Cmd::LoadFromIndex(id) => match ins.get(id) {
                    Some(path) => match load_html(path.to_string()) {
                        Ok(w) => {
                            println!("Loaded html");
                            wp = Some(w);
                        }
                        Err(err) => eprintln!("Error loading file from index: {}", err),
                    },
                    None => eprintln!("Invalid index: {}", id),
                },
                Cmd::CreateRequest => {
                    // TODO: Add way to create request
                }
                Cmd::Quit => return,
            }
        } else {
            println!("Didn't understand: '{:?}'", &buffer);
        }
        buffer.clear();
    }
}

fn parse_input(str: &str) -> Option<Cmd> {
    let s = str.replace("\r\n", "");
    let mut its = s.split(" ");
    if let Some(el) = its.next() {
        match el {
            "form" => {
                return Some(Cmd::DisplayForm);
            }
            "load" => {
                if let Some(e) = its.next() {
                    match e.parse::<usize>() {
                        Ok(index) => {
                            return Some(Cmd::LoadFromIndex(index));
                        }
                        Err(_) => {
                            let url = its.collect::<String>();
                            let mut link = Link::new(url);
                            if link.is_external() {
                                eprintln!("URL is external: {:?}", link);
                            }
                            if link.is_partial() {
                                link = link.create_full();
                                eprintln!("URL is partial, changed it too: {:?}", link);
                            }
                            return Some(Cmd::LoadFromUrl(link));
                        }
                    }
                }
            }
            "display" => {
                return Some(Cmd::DisplayHtml);
            }
            "q" => {
                return Some(Cmd::Quit);
            }
            _ => {
                return None;
            }
        }
    }
    None
}

fn load_html(path: String) -> Result<Webpage, ArchiveError> {
    let mut file = File::open(path.clone())?;

    let mut raw_html = String::new();
    file.read_to_string(&mut raw_html)?;

    let mut id = 0;
    let mut medium = "unknown".to_string();

    let mut str = path.split("\\").collect::<Vec<_>>().into_iter().rev();

    if let Some(i) = str.next()
        && let Some(m) = str.next()
    {
        id = i.parse::<usize>()?;
        medium = m.to_owned();
    }

    if medium == "in".to_string() {
        medium = "unknown".to_string();
    }

    let url = format!("{}/{}/{}", ROOT_URL, &medium, &id);
    let content = Html::parse_document(&raw_html);

    Ok(Webpage::from_html(id, url, content, medium))
}
