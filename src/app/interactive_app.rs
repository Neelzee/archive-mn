use std::{
    collections::HashMap,
    fmt::format,
    fs::File,
    io::{self, Read},
    path::Path,
};

use itertools::Itertools;
use reqwest::Client;
use scraper::Html;

use crate::{
    app::{
        main_app::{write_failed_sok, write_log},
        try_save_sok,
    },
    error::ArchiveError,
    modules::{
        form::{Form, FormOption},
        webpage::{Link, Webpage},
    },
    parser::{get_title, wp::get_sok_collection_form},
    scraper::get_html_content,
    utils::constants::ROOT_URL,
};

use strum::IntoEnumIterator;

use super::offline_app::visit_dirs;

#[derive(Debug)]
enum Cmd {
    DisplayForm(usize),
    Load(Vec<Link>),
    List,
    CreateRequest(usize),
    Display(usize),
    Help,
    Quit,
}

impl Cmd {
    fn iter() -> std::vec::IntoIter<Cmd> {
        vec![
            Cmd::DisplayForm(0),
            Cmd::Load(vec![Link::new(ROOT_URL.to_string())]),
            Cmd::CreateRequest(0),
            Cmd::Display(0),
            Cmd::Help,
            Cmd::Quit,
        ]
        .into_iter()
    }
}

pub async fn interactive() {
    let mut buffer = String::new();

    let mut wps: Vec<Webpage> = Vec::new();

    loop {
        let stdin = io::stdin();
        if let Err(err) = stdin.read_line(&mut buffer) {
            eprintln!("Failed reading input: {}", err);
        }

        if let Some(cmd) = parse_input(&buffer) {
            match cmd {
                Cmd::DisplayForm(i) => {
                    if i >= wps.len() {
                        eprintln!("Invalid index, {} >= {}", i, wps.len());
                        continue;
                    }
                    let w = wps.get(i).unwrap();
                    let res = w.get_forms();
                    if res.is_err() {
                        eprintln!("Failed loading form: {}", res.unwrap_err());
                    } else {
                        let form = res.unwrap();
                        for op in form.options() {
                            op.show();
                        }
                    }
                    println!("/form");
                }
                Cmd::Load(link) => {
                    for l in link {
                        if let Ok(wp) = Webpage::from_link(l.clone()).await {
                            println!(
                                "Loaded webpage: {};{:?} from link: {}",
                                wp.get_id(),
                                wp.get_title(),
                                l.to_string()
                            );
                            wps.push(wp);
                        }
                    }
                    println!("/load");
                }
                Cmd::Display(i) => {
                    if i >= wps.len() {
                        eprintln!("Invalid index, {} >= {}", i, wps.len());
                        continue;
                    }

                    println!("\n{}\n", "=".repeat(30));
                    println!("{:?}", wps.get(i as usize));
                    println!("\n{}\n", "=".repeat(30));

                    println!("/display");
                }
                Cmd::CreateRequest(i) => {
                    if i >= wps.len() {
                        eprintln!("Invalid index, {} >= {}", i, wps.len());
                        continue;
                    }

                    create_req(wps.get(i)).await;

                    println!("/request");
                }
                Cmd::Quit => return,
                Cmd::Help => {
                    for c in Cmd::iter() {
                        println!("{:?}", c);
                    }
                    println!("/help");
                }
                Cmd::List => {
                    for i in 0..wps.len() {
                        if let Some(wp) = wps.get(i) {
                            println!("{} | {};{:?}", i, wp.get_id(), wp.get_title());
                        }
                    }
                }
            }
        } else {
            println!("Didn't understand: '{:?}'", &buffer);
        }
        buffer.clear();
    }
}

async fn create_req(wp: Option<&Webpage>) {
    if let None = wp {
        eprintln!("Expected webpage, got None");
        return;
    }

    let mut buffer = String::new();

    let mut new_form = Form::new();
    let wp = wp.expect("Should contain a webpage");
    match wp.get_forms() {
        Ok(forms) => loop {
            buffer.clear();
            println!("{:?}", &new_form);
            let stdin = io::stdin();
            if let Err(err) = stdin.read_line(&mut buffer) {
                eprintln!("Failed reading input: {}", err);
                buffer.clear();
                continue;
            }

            match buffer.replace("\r\n", "").split(" ").collect::<Vec<&str>>()[..] {
                ["list"] => {
                    for op in forms.options() {
                        if !new_form.options().contains(&op) {
                            op.show();
                        }
                    }
                }
                ["show"] => {
                    println!("Old Form:");
                    for op in forms.options() {
                        op.show();
                    }
                }
                ["fill"] => {
                    if forms.missing_options(&new_form) {
                        new_form = forms.fill_form_data(&new_form);
                    }
                }
                ["clear"] => new_form.clear(),
                ["send"] => {
                    if forms.missing_options(&new_form) {
                        new_form = forms.fill_form_data(&new_form);
                    }
                    match get_sok_collection_form(wp.clone(), new_form.clone()).await {
                        Ok((sokc, mut errs)) => {
                            let path = format!("arkiv\\{}", sokc.medium);
                            match try_save_sok(&sokc, &path, 2) {
                                Ok(mut logs) => {
                                    logs.append(&mut errs);
                                    let _ = write_log(
                                        logs.into_iter().map(|e| e.to_string()).collect_vec(),
                                        sokc.id,
                                    );
                                    println!("Saved to path: {}", path);
                                }
                                Err(mut logs) => {
                                    logs.append(&mut errs);
                                    let _ = write_failed_sok(
                                        logs.into_iter().map(|e| e.to_string()).join("\n"),
                                        &sokc.id,
                                        sokc.title,
                                    );
                                }
                            }
                        }
                        Err(err) => {
                            let _ = write_failed_sok(
                                err.to_string(),
                                &wp.get_id(),
                                wp.get_title().unwrap_or("MISSING TITLE".to_string()),
                            );
                        }
                    }
                    return;
                }
                ["all", option] => {
                    if forms.contains_option(option.to_string()) {
                        println!("Adding all {}", &option);
                        let mut op: FormOption = FormOption::new(option.to_string(), Vec::new());
                        if let Some(_op) = new_form.get_option(option.to_string()) {
                            op = _op;
                        }
                        for (r, d) in forms
                            .get_option(option.to_string())
                            .expect("Should be option")
                            .options()
                        {
                            if !op.contains_req(r.clone()) {
                                op.add_options((r, d));
                            }
                        }
                        new_form.add_options(op);
                    }
                }
                [option, ref choices @ ..] => {
                    if forms.contains_option(option.to_string())
                        && choices
                            .into_iter()
                            .all(|e| forms.contains_choice(e.to_string()))
                    {
                        new_form.add_options(FormOption::new(
                            option.to_string(),
                            choices
                                .into_iter()
                                .map(|e| (e.to_string(), forms.get_display(e.to_string())))
                                .collect_vec(),
                        ));
                        println!("Updated forms");
                    } else {
                        println!("could not understand: '{:?}', '{:?}'", option, choices);
                    }
                }

                _ => eprintln!("Failed parsing buffer: {}", buffer),
            }
        },
        Err(err) => eprintln!("Failed getting forms: {}", err),
    }
}

fn parse_input(str: &str) -> Option<Cmd> {
    let binding = str.replace("\r\n", "");
    let input = binding.split(" ").collect::<Vec<&str>>();
    match input.clone()[..] {
        ["form", index] => match index.parse::<usize>() {
            Ok(i) => Some(Cmd::DisplayForm(i)),
            Err(_) => {
                eprintln!("Expected u32, got {}", index);
                None
            }
        },
        ["load", ref links @ ..] => Some(Cmd::Load(
            links
                .into_iter()
                .map(|e| match e.parse::<u32>() {
                    Ok(i) => Link::new(format!("{}/statistikk/medium/cli/{}", ROOT_URL, i)),
                    Err(_) => Link::new(e.to_string()),
                })
                .filter(|l| !l.is_external())
                .collect_vec(),
        )),
        ["req", index] | ["request", index] => match index.parse::<usize>() {
            Ok(i) => Some(Cmd::CreateRequest(i)),
            Err(_) => {
                eprintln!("Expected usize, got {}", index);
                None
            }
        },
        ["list"] => Some(Cmd::List),
        ["display", index] => match index.parse::<usize>() {
            Ok(i) => Some(Cmd::Display(i)),
            Err(_) => {
                eprintln!("Expected u32, got {}", index);
                None
            }
        },
        ["h"] | ["help"] => Some(Cmd::Help),
        ["q"] | ["quit"] | [":q"] => Some(Cmd::Quit),
        _ => None,
    }
}
