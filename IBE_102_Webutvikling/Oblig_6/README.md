<h1>Oppgave Beskrivelse</h1>
<pre>
I denne obligen skal du øve på å bruke ferdige funksjoner for tekst (kap 6.1-2).
Hvis du ønsker, kan du også øve på å skrive funksjoner selv (sist i kap. 5).

I en tidligere oblig laget du et skjema der bruker valgte valuta fra menyer.

Ta en kopi av denne (kall den oblig6.php) og legg til et søkefelt der bruker kan skrive
inn en letetekst og få listet alle valutaer som treffer enten i ticker eller valutanavnet.

I første omgang oppfører søket seg case sensitivt.

Se eksempelkjøring.
https://www.youtube.com/watch?v=DsmBTfQdpmk

Når bruker trykker "Let!" vises de valutaer som passer med leteteksten.

Hvis brukeren oppgir "TH" vises kun Ethereumkursen (fordi den har "TH" i sin ticker).
Hvis bruker skriver "e" vises fire av kursene (fordi de har "e" i navnet).

Bruk gjerne strpos, som er case sensitiv.  Se evt strstr og stristr.
Du må nok teste to ganger, først på ticker og så på valutanavnet.

Som svar laster du opp programfilen du har laget.

---

Del 2 (frivillig):  Lag en funksjon treff(t, m) som svarer True hvis valuta med ticker t er et treff,
altså at mønsteret m finnes. Da kan jeg skrive treff ("BTC", "C") og få True, evt.
treff ("ETH", "Ompapa") og få False.
Vis at funksjonen virker. OBS: For å bruke den globale v2 fra funksjonen må den nok deklareres lokalt
(som "global $v2"; les om global i boka s. 139).

Del 3 (frivillig) Lag et tillegg slik at hvis en skriver treff(t,m,cs=False) så vil treff
oppføre seg case insensitivt, altså at treff("BTC", "N", cs=False) gir True fordi navnet
("Bitcoin") har n i seg (liten n), mens treff("BTC", "N") gir False.
En måte å løse dette på er å kjøre strpos ( strupper(a), strupper(b) ) for å sammenligne a og b.
Parametret "cs" må også få en default verdi True, slik at en ikke trenger angi det.
</pre>


<h1>Oppgave Besvarelse</h1>
<pre>
Åpne oblig6.php i nettleseren. Sørg for at data.inc.php også er i samme mappe da oblig6.php avhenger av denne.
</pre>
