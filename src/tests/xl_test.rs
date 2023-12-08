use crate::{parser::sok::{Sok, Table}, xl::save_sok};

#[test]
fn test_save() {
    let mut sok = Sok::new(0, "avis".to_string());
    sok.id = 346;
    let mut table_1 = Table::new();
    table_1.name = "Andel med avisabonnement hjemme, fordelt på alle (prosent)".to_string();
    table_1.rows = vec![
        vec![
            "Alle".to_string(),
            "2000".to_string(),
            "2001".to_string(),
            "2002".to_string(),
            "2003".to_string(),
            "2004".to_string(),
            "2005".to_string(),
            "2006".to_string(),
            "2007".to_string(),
            "2008".to_string(),
            "2009".to_string(),
            ],
        vec![
            "Alle (9-79 år)".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "1,0".to_string(),
            "1,0".to_string(),
            "1,1".to_string(),
            "1,0".to_string(),
        ]
    ];
    assert_eq!(table_1.rows.get(0).unwrap().len(), table_1.rows.get(1).unwrap().len());

    let mut table_2 = Table::new();
    table_2.name = "Andel med avisabonnement hjemme, fordelt på alle (prosent)".to_string();
    table_2.rows = vec![
        vec![
            "Alle".to_string(),
            "2010".to_string(),
            "2011".to_string(),
            "2012".to_string(),
            "2013".to_string(),
            "2014".to_string(),
            "2015".to_string(),
            "2016".to_string(),
            "2017".to_string(),
            "2018".to_string(),
            "2019".to_string(),
            ],
        vec![
            "Alle (9-79 år)".to_string(),
            "1,0".to_string(),
            "1,0".to_string(),
            "0,9".to_string(),
            "0,9".to_string(),
            "0,8".to_string(),
            "0,7".to_string(),
            "0,6".to_string(),
            "0,6".to_string(),
            "0,6".to_string(),
            "0,5".to_string(),
        ]
    ];
    assert_eq!(table_2.rows.get(0).unwrap().len(), table_2.rows.get(1).unwrap().len());

    let mut table_3 = Table::new();
    table_3.name = "Andel med avisabonnement hjemme, fordelt på alle (prosent)".to_string();
    table_3.rows = vec![
        vec![
            "Alle".to_string(),
            "2020".to_string(),
            "2021".to_string(),
            "2022".to_string(),
            ],
        vec![
            "Alle (9år+/-79 år)".to_string(),
            "0,5".to_string(),
            "0,4".to_string(),
            "-".to_string(),
        ]
    ];
    assert_eq!(table_3.rows.get(0).unwrap().len(), table_3.rows.get(1).unwrap().len());

    sok.tables = vec![table_1, table_2, table_3];

    sok.title = "Andel med papiravisabonnement og antall abonnement".to_string();

    sok.text = vec![
        "Statistisk sentralbyrå har gjennomført mediebruksundersøkelser hvert år siden 1991 (med unntak av 1993). Undersøkelsene er i hovedsak finansiert av Kulturdepartementet og formålet er å kartlegge bruken av ulike medier i Norge. I 1995 fikk undersøkelsene navnet Norsk mediebarometer.".to_string(),
        "I 2022 ble undersøkelsen utvidet og endret, slik at dataene ikke er helt sammenlignbare med tidligere år. Utvalget som besvarte undersøkelsen er doblet, og det er lagt til en ny alderskategori: 80 år og eldre. Utvidelsen i alder innvirker på resultatene, siden eldre generelt bruker mer tradisjonelle medier enn yngre. I tillegg har spørreskjema på nett erstattet telefonintervju som hovedmetode for datainnsamling, og selve spørreskjemaet har gjennomgått flere endringer.".to_string(),
        "Her kan du finne tall for andel som har abonnement på papiravis hjemme, samt gjennomsnittlig antall abonnement i den norske befolkningen. Bruk menyen til høyre for å velge. I samme meny kan du også velge å få tallene fra 2006 og framover fordelt på ulike bakgrunnsvariabler, som kjønn, alder og utdanning. Det finnes egne tall for andel med nettavisabonnement hjemme.".to_string(),
        "Resultater fra andre deler av Norsk mediebarometer finner du i denne menyen. Rapport for undersøkelsen i sin helhet finner du på nettsidene hos Statistisk sentralbyrå.".to_string()
    ];

    sok.merknad = vec![
        "Fra og med 2022 er ikke gjennomsnittelig antall abonnement lenger inkludert i undersøkelsen.".to_string(),
        "Fra og med 2022 er aldersutvalget for undersøkelsen endret fra 9-79 år til 9 år pluss. Inkluderingen av personer fra 80 år og oppover har innvirkning på resultatene, siden eldre generelt bruker mer tradisjonelle medier enn yngre. Samtidig er utvalget for undersøkelsen doblet i størrelse og spørreskjema på nett har erstattet telefonintervju som hovedmetode for innsamling av data. Spørreskjemaet er også endret i forhold til tidligere år.".to_string(),
        "SSB har i 2020 gjort endringer i inndelingen av landsdeler.".to_string(),
        "Opplysningene gjelder kun abonnement på papiraviser.".to_string(),
        "Fordelinger på utdanning og yrke gjelder aldersgruppen 16 år og eldre.".to_string()
    ];

    sok.kilde = vec![
        "Statistisk sentralbyrå".to_string()
    ];

    sok.metode = vec![
        "Statistisk sentralbyrå gjennomførte kultur- og mediebruksundersøkelser i 1991, 1992 og 1994. F.o.m. 1995 ble mediedelen av undersøkelsene videreført årlig under navnet Norsk mediebarometer. Kulturdelen videreføres som Norsk kulturbarometer.".to_string(),
        "Undersøkelsene har fram til 2022 vært gjennomført via telefonintervju, med fordeling av intervjuene på alle ukedager og på fire perioder i året: mars, juni, september og desember. Dette er gjort for at svarene kunne gi et representativt bilde av mediebruk på årsbasis.".to_string(),
        "I 2022 ble undersøkelsen endret på flere måter: Utvalget ble utvidet fra rundt 3000 personer til 6000. Samtidig ble aldergrensen på 79 år fjernet, slik at også personer 80 år og eldre deltok i undersøkelsen. Endringen i alderssammensetning påvirker resultatene, da eldre gjerne har mer tradisjonell mediebruk enn yngre.".to_string(),
        "En viktig endring er også at SSB dette året gikk over fra telefonintervju til spørreskjema på nett som hovedmetode. Personer som ikke besvarte undersøkelsen ble fulgt opp med intervju på telefon.".to_string(),
        "Spørsmålene om mediebruk gjelder dagen før intervjuet/skjemautfylling fant sted. Spørreskjemaet ble mye endret i 2022, slik at sammenlignbarheten med tidligere år ble dårligere. I perioden 1999-2021 var spørsmålene om bruk av radio og TV mer detaljerte, noe som kan ha ført til en økning i brukertallene for disse årene.".to_string(),
        "Undersøkelsene er gjennomført blant utvalg som er representative for hele den norske befolkningen 9-79 år (1991-2021) og 9 år og eldre (fra 2022). Utvalgsstørrelsen har fram til 2022 ligget på på rundt 3000 personer, med en svarporsent som har vært langsomt synkende. I 2021 lå den på 54 prosent. I 2022 ble utvalget økt til rundt 6000 personer. Svarprosenten var den samme som i 2021.".to_string(),
        "Fordelinger på utdanning og yrke omfatter bare personer 16 år og eldre.".to_string()
    ];

    let res = sok.save("src\\tests\\sok_346.xlsx");

    let res_2 = save_sok(vec![sok], "src\\tests\\sok_346_new.xlsx");

    if res.is_err() {
        eprintln!("{}", res.as_ref().unwrap_err());
    }
    
    if res_2.is_err() {
        eprintln!("{}", res_2.as_ref().unwrap_err());
    }

    assert!(res.is_ok());
    assert!(res_2.is_ok());
}