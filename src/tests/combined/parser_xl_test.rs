use crate::{parser::wp::get_sok_collection, utils::funcs::get_random_webpage, xl::save_sok};

#[tokio::test]
async fn test_parsing_saving() {
    if let Some(wp) = get_random_webpage() {
        let res = get_sok_collection(wp).await;

        assert!(res.is_ok());

        let (sc, _) = res.unwrap();

        println!("id: {}", &sc.id);

        for sk in sc.sok.clone() {
            println!("\n{:?}\n", sk);
        }

        let res2 = save_sok(&sc, "src\\tests");
        assert!(res2.is_ok());
    } else {
        eprintln!("Could not get a random webpage");
    }
}
