# Archive-MN

This script was used too archive [medienorge](https://web.archive.org/web/20231201215514/https://medienorge.uib.no/).
In this content, `archive`, means getting all the `sok`s and converting them into excel files, for long-term offline storage.

To achive this in a satisfactory manner, there were several `MVP`s.

- Get all information from a `sok`
  - Title
  - Text
  - Tables
  - _sub_-`sok`
  - `merknad`
  - `kilde`
  - `metode`
- List all successes
- Give satisfactory error messages

Which, for web-scraping, was easy. Most of the wanted information from any `sok`, is in id-specifc html-tags, so using the crate [scraper](https://docs.rs/scraper/latest/scraper/), getting `merknad`, title, text and tables was piece of cake.

Getting the title, was as simple as looking for the one `h1`-tag, and getting the text-content.

The text, is as simple as iterating over all `p`-tags inside this div: `<div id="forklaringTxt">`.

For tables, its iterating over all `table`-tags inside this div: `<div id="sokResult">`.

And for `merknad`, its just getting the text from all `<p class=merknadTekst>`.

The remainding information is harder to get, as they either have information behind other URLs (`kilder` and `metode`), or they are retrived by a `POST`-request (`sub-sok`).

`kilder` and `metode`, is a list of URLs, each corresponding to a webpage with a title (`h1`), and text (`p`). So, to get the URLs, it's as simple as getting `a.bold-text[href][onclick]`, and getting the `a`-tags, with parent elements with `.merknadHeader`. Then, using their `href`, just scrape their content.

Finally, `sub-sok`. All `sok`s have different values their tables are showing. An example of a `sok` be "News paper subscription". The `sok` at that URL, will have tables showing the data from that study, sorted on `alle` (everyone). So, each `sub-sok` is just a different variables, like `alder` (age), `yrke` (occupation), or `utdanning` (education). It was important having all variations of a `sok` in the same excel workbook, on different sheets.
Too achive this, the `form`-tag was analyzed. Using the different arguments in the `form`, a POST-request was made using the [reqwest](https://docs.rs/reqwest/latest/reqwest/) crate. To construct each POST-request for every variation of a `sok`, a cartesian product was made on the arguments. This worked for most `sok`, as most had a count of less than 50, but some `sok` were worse, and had a total count of **25536**. To solve this, a hardcap of 50 was set, and the few (around 10/265) were archived _manually_ (aka, downloaded the html-page, since the `sok` with a count higher than 100, had such a high count due to being able to differentiate between years, (you could choose any combination of years between 1991 to 2020), so the _correct_ variation of the `sok`, was the one with all the years; just one "valid" way to combine the arguments).

This worked, but a lot of time was wasted, due to _not_ reading the documentation. The webpage, medienorge, is a norwegian website, so there were non-ascii charecters. During testing, this error occured:

```
thread 'main' panicked at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962\library\core\src\str\mod.rs:660:13:
byte index 31 is not a char boundary; it is inside 'å' (bytes 30..32) of `Skjønnlitterære bøker og småtrykk etter type 1991 - 2009 (antall titler)`
```

Which was caused by the use of `split_at()`, on a non-ascii string.

```ps
cargo run "https://medienorge.uib.no/statistikk/medium/avis" "https://medienorge.uib.no/statistikk/medium/fagpresse" "https://medienorge.uib.no/statistikk/medium/ukepresse" "https://medienorge.uib.no/statistikk/medium/boker" "https://medienorge.uib.no/statistikk/medium/radio" "https://medienorge.uib.no/statistikk/medium/fonogram" "https://medienorge.uib.no/statistikk/medium/tv" "https://medienorge.uib.no/statistikk/medium/kino" "https://medienorge.uib.no/statistikk/medium/video" "https://medienorge.uib.no/statistikk/medium/ikt"
```
