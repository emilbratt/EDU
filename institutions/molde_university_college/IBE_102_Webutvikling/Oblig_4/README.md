<h1>Oppgave Beskrivelse</h1>
<pre>
Lag "visvaluta.php" der all valutainfo ligger i en matrise som "v2",
identisk med "v2" som du laget i forrige oblig.

visvaluta.php skal enten lage skjema der bruker får velge valuta,
eller behandle skjema ved å vise kurser for valgt valuta.

I skjemaet velger bruker fra en ettvalgs nedtrekksliste med alle
valutaene som finnes i v2,
der hvert valg viser ticker og navn, f.eks. "ETH Ethereum".
Når knapp "Vis enkeltvaluta" trykkes, kalles samme programmet og
viser nyeste kurs ved å lese fra v2.
Resultatet (hvis jeg velger ETH) blir: "Ethereum (ETH): 47.2"
(trenger ikke ha samme verdier).

OBS: Nedtrekkslisten skal ikke være statisk, den skal være dynamisk.
D.v.s at innholdet i listen lages ut fra hva som ligger i v2.
Programmet må derfor lese v2 hver gang den skal lage nedtrekkslisten.
Av erfaring (fra tidligere år) er det flere som lager denne listen statisk
(og må levere om igjen).

Programmet blir da altså:
Hvis "Vis enkeltvaluta" valgt, behandle skjema, ellers lag skjema.


Utvidelse (FRIVILLIG): I samme skjema, legg inn sjekkbokser
(en for hver valuta og egen knapp "Vis alle valgte valutaer".
Bruker får da listet siste kurs for alle valgte.
Hvis ingen ble valgt (som jo er mulig med sjekkboks vises
"ingen valutaer valgt".
Programmet (med utvidelsen) blir da:  Hvis "Vis enkeltvaluta",
vis enkeltvaluta, ellers hvis "Vis valutaer",
vis valgte valutaer, ellers, lag skjema.
</pre>


<h1>Oppgave Besvarelse</h1>
<pre>
Filen "visvaluta.php" åpnes i din php-tolker og avhenger av å ha "data.inc.php" i samme mappe
</pre>
