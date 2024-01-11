use itertools::Itertools;

use crate::{
    app::main_app::{write_failed_sok, write_log},
    error::ArchiveError,
    modules::webpage::Link,
};

use super::{main_fn, try_save_sok};

pub async fn get_soks(links: Vec<Link>) -> Result<(), ArchiveError> {
    for link in links {
        match main_fn(&link).await {
            Ok((sokc, mut sok_log)) => {
                let path = format!("arkiv\\");
                match try_save_sok(&sokc, &path, 2) {
                    Ok(mut log) => {
                        sok_log.append(&mut log);
                        if let Err(e) = write_log(
                            sok_log
                                .clone()
                                .into_iter()
                                .map(|e| e.to_string())
                                .collect_vec(),
                            sokc.id,
                        ) {
                            eprintln!("Error writing too logs: {}, dumping log to terminal", e);
                            for l in sok_log {
                                eprintln!("{}", l);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!(
                            "Failed saving sok: {};{}, due too errors: {}, dumping it to root",
                            &sokc.id,
                            &sokc.title,
                            err.into_iter().map(|e| e.to_string()).join("\n")
                        );
                    }
                }
            }
            Err(err) => {
                let mut id = 0;
                let url_c = link.to_string().clone();

                let mut str = url_c.split("/").collect::<Vec<_>>().into_iter().rev();

                if let Some(i) = str.next() {
                    id = i.parse::<usize>()?;
                }

                if let Err(e) = write_failed_sok(
                    err.clone().into_iter().map(|e| e.to_string()).join("\n"),
                    &id,
                    "UNKNOWN".to_string(),
                ) {
                    eprintln!(
                        "Error writing too error logs: {}, dumping log to terminal",
                        e
                    );
                    for l in err {
                        eprintln!("{}", l);
                    }
                }
            }
        }
    }
    Ok(())
}
