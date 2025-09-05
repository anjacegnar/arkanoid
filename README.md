# Arkanoid

Arkanoid je klasična računalniška igra, v kateri igralec nadzira ploščico, ki se premika levo in desno, da odbija žogico, ki uničuje bloke na zaslonu. Ko žogica trči v blok, le-ta izgine, igralec pa dobi točke. Cilj igre je uničiti vse bloke, ne da bi žogica padla na tla in zbrati čim več točk. Igra ima več nivojev, vsak z različnimi postavitvami blokov.

## Navodila za uporabo

Za igranje je potrebno imeti Rust in kloniran ta repozitorij. Uporablja se knjižnica macroquad, ki je ni treba dodatno naložiti. V terminalu se premaknite v mapo, v kateri se nahaja repozitorij in vpišite ukaz `cargo run`. Pojavilo se bo okno, v katerem se požene igra.

## Navodila za igranje

Ploščico premikamo z levo in desno puščico na tipkovnici. Na začetku ima igralec 3 življenja, vsakič, ko žogica pade na tla, pa enega izgubi. Ob izgubi vseh življenj se igra začne od začetka. Med igro je možno tudi pobrati powerupe za dodatno življenje, daljšo ploščico, več žogic in počasnejše žogice.
