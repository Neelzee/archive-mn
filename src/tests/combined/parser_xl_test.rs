use scraper::Html;

use crate::{modules::webpage::Webpage, utils::{constants::ROOT_URL, funcs::{get_random_file_and_contents, get_random_webpage}}, parser::wp::get_sok_collection, xl::save_sok};

#[tokio::test]
async fn test_parsing_saving() {
    if let Some(wp) = get_random_webpage() {
        let res = get_sok_collection(wp).await;

        assert!(res.is_ok());

        let (sc, _) = res.unwrap();

        let res2 = save_sok(sc, "src\\tests");
        eprintln!("{:?}", &res2);
        assert!(res2.is_ok());
    } else {
        panic!("Could not get a random webpage");
    }
}