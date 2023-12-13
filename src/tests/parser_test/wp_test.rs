/*
Webpage test
*/

use std::{fs::File, io::Read, iter::zip};

use scraper::Html;

use crate::{modules::webpage::Webpage, parser::wp::get_metode};

fn get_webpage() -> Result<Webpage, std::io::Error> {
    let mut content = String::new();

    let mut file = File::open("src\\tests\\html\\sok_346.html")?;
    file.read_to_string(&mut content)?;

    Ok(Webpage::from_html(
        346,
        "https://medienorge.uib.no/statistikk/medium/avis/346".to_string(),
        Html::parse_fragment(&content),
        "avis".to_string()))
}


#[test]
fn test_get_links() {
    if let Ok(wp) = get_webpage() {
        let res = wp.get_links();

        assert!(res.is_ok());

        let links = res.unwrap();

        assert!(links.len() != 0);

        assert_eq!(
            "https://medienorge.uib.no/files/institusjon.cfm?institusjon_id=38".to_owned(),
            links.get(0).unwrap().create_full().to_string()
        );

        assert_eq!(
            "https://medienorge.uib.no/metode/346".to_owned(),
            links.get(1).unwrap().create_full().to_string()
        );

        println!("{:?}", links);
    } else {
        panic!("Could not get webpage to test");
    }
}

#[test]
fn test_get_title() {
    if let Ok(wp) = get_webpage() {
        let res = wp.get_title();

        assert!(res.is_ok());

        let title = res.unwrap();

        assert!(title.len() != 0);
        assert_eq!(
            "Andel med papiravisabonnement og antall abonnement",
            &title
        );
    } else {
        panic!("Could not get webpage to test");
    }
}


#[test]
fn test_get_text() {
    if let Ok(wp) = get_webpage() {
        let res = wp.get_text();

        assert!(res.is_ok());

        let text = res.unwrap();

        assert!(text.len() != 0);

        let expected_result = vec![
            "Statistisk sentralbyrå har gjennomført mediebruksundersøkelser hvert år siden 1991 (med unntak av 1993). Undersøkelsene er i hovedsak finansiert av Kulturdepartementet og formålet er å kartlegge bruken av ulike medier i Norge. I 1995 fikk undersøkelsene navnet Norsk mediebarometer.".to_owned(),

            "I 2022 ble undersøkelsen utvidet og endret, slik at dataene ikke er helt sammenlignbare med tidligere år. Utvalget som besvarte undersøkelsen er doblet, og det er lagt til en ny alderskategori: 80 år og eldre. Utvidelsen i alder innvirker på resultatene, siden eldre generelt bruker mer tradisjonelle medier enn yngre. I tillegg har spørreskjema på nett erstattet telefonintervju som hovedmetode for datainnsamling, og selve spørreskjemaet har gjennomgått flere endringer.".to_owned(),

            "Her kan du finne tall for andel som har abonnement på papiravis hjemme, samt gjennomsnittlig antall abonnement i den norske befolkningen. Bruk menyen til høyre for å velge. I samme meny kan du også velge å få tallene fra 2006 og framover fordelt på ulike bakgrunnsvariabler, som kjønn, alder og utdanning. Det finnes egne tall for andel med nettavisabonnement hjemme.".to_owned(),

            "Resultater fra andre deler av Norsk mediebarometer finner du i denne menyen. Rapport for undersøkelsen i sin helhet finner du på nettsidene hos Statistisk sentralbyrå.".to_owned()
        ];

        for (expected, result) in zip(expected_result, text) {
            assert_eq!(expected, result);
        }

    } else {
        panic!("Could not get webpage to test");
    }
}


#[test]
fn test_get_forms() {
    if let Ok(wp) = get_webpage() {
        let res = wp.get_forms();

        assert!(res.is_ok());

        let form = res.unwrap();

        println!("{:?}", form);
    } else {
        panic!("Could not get webpage to test");
    }
}

#[test]
fn test_get_sok() {
    if let Ok(wp) = get_webpage() {
        let res = wp.get_sok();

        assert!(res.is_ok());

        let sok = res.unwrap();

        for t in sok.clone().tables {
            t.show();
        }

        println!("{:?}", sok);
    } else {
        panic!("Could not get webpage to test");
    }
}


#[test]
fn test_get_merknad() {
    if let Ok(wp) = get_webpage() {
        let res = wp.get_merknad();

        assert!(res.is_ok());

        let merknad = res.unwrap();

        assert!(merknad.len() != 0);

        println!("{:?}", merknad);
    } else {
        panic!("Could not get webpage to test");
    }
}

#[tokio::test]
async fn test_get_metode() {
    if let Ok(wp) = get_webpage() {
        let res = get_metode(&wp).await;

        assert!(res.is_ok());

        let merknad = res.unwrap();

        assert!(merknad.len() == 1);

        println!("{:?}", merknad);
    } else {
        panic!("Could not get webpage to test");
    }
}