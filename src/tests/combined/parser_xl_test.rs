use scraper::Html;

use crate::{modules::webpage::Webpage, utils::{constants::ROOT_URL, funcs::get_random_file_and_contents}, parser::wp::get_sok_collection, xl::save_sok};

fn get_random_webpage() -> Option<Webpage> {
    if let Ok((file_name, raw_content)) = get_random_file_and_contents("src\\tests\\html".to_string()) {

        let url = format!("{}/{}", ROOT_URL, file_name.clone());
        let content = Html::parse_document(&raw_content);

        let mut id = 0;
        let medium = String::from("MEDIUM");

        if let Ok(i) = file_name.parse::<usize>() {
            id = i;
        } else {
            return None;
        }

        Some(Webpage::from_html(id, url, content, medium))
    } else {
        None
    }
}

#[tokio::test]
async fn test_parsing_saving() {
    if let Some(wp) = get_random_webpage() {
        let res = get_sok_collection(wp).await;

        assert!(res.is_ok());

        let sc = res.unwrap();

        let res2 = save_sok(sc, "src\\tests");
        eprintln!("{:?}", &res2);
        assert!(res2.is_ok());
    } else {
        panic!("Could not get a random webpage");
    }
}