# archive-mn

Code for archiving medienorge

```nushell
cargo run "https://medienorge.uib.no/statistikk/medium/avis" "https://medienorge.uib.no/statistikk/medium/fagpresse" "https://medienorge.uib.no/statistikk/medium/ukepresse" "https://medienorge.uib.no/statistikk/medium/boker" "https://medienorge.uib.no/statistikk/medium/radio" "https://medienorge.uib.no/statistikk/medium/fonogram" "https://medienorge.uib.no/statistikk/medium/tv" "https://medienorge.uib.no/statistikk/medium/kino" "https://medienorge.uib.no/statistikk/medium/video" "https://medienorge.uib.no/statistikk/medium/ikt"
```

Got this error:

```rs
thread 'main' panicked at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962\library\core\src\str\mod.rs:660:13:
byte index 31 is not a char boundary; it is inside 'å' (bytes 30..32) of `Skjønnlitterære bøker og småtrykk etter type 1991 - 2009 (antall titler)`
```

Because I used `len()`, instead of `chars().count()`
