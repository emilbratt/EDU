<h1>Oppgave Beskrivelse</h1>
<pre>
Målet her er å trene på modularisering til flere filer, og inklusjon (kap. 5, første del).
Senere vil vi se på siste del i kap 5 (om funksjoner).

Nettstedet for kryptovaluta ligger i mappen "kryptovaluta".
For å nå det skriver jeg "localhost/kryptovaluta" i nettleseren.

Her har du filene index.php som skal lenke til diverse innhold via en kulepunktliste
som med A lenker inn interessant innhold som oblig2.php, oblig3.php og oblig4.php.

Filen "valutaer.php" er en ren datafil som kun inneholder gjeldende valutakurser.
Den skal definere v2. Flytt derfor matrisen "v2" hit og sett "oblig4.php" til å inkludere denne.

Hvis du har ikonfiler (som ETH.png, hvis du gjorde den frivillige delen av forrige oblig)
og andre bilder i dine program, plasser de i egen undermappe.

For å få likt utseende, skal alle innholds-sidene inkludere "topp.php" og "bunn.php"
slik at de utgjør topp og bunn.  Dette er forklart i boken.  Selve innholdet kommer
da som et midtparti (under topp og over bunn).

Topp.php lager en tabell (1 linje, 2 kolonner) som viser velkomst og nyheter.

Programmets navn skal være "Kryptovaluta A/S" og lagres som konstanten PROGNAVN
i en fellesfil med navn "felles.php".  PROGNAVN skal også brukes for TITLE,
og overalt ellers der vi vil skrive ut navnet.  Alle program må inkludere denne,
så mulig require_once må brukes.

Nyhetene inkluderes fra "nyheter.html". De som skriver nyheter oppdaterer da denne filen,
som kun inneholder UL og et par LI med nyhetslenker (A) om kryptovaluta.

Vis også når nyhetsfilen ble sist oppdatert (http://php.net/manual/en/function.filemtime.php)

Bunntekst gir info om nettstedets innehaver som ikke påtar seg noe som helst ansvar
for bruk av nettstedet, og i tillegg har en knapp ("Vis hjelpeinfo") som når den
trykkes viser en hjelpetekst ("hjelp.php"), aktivert av Javascript, slik som i boken.

Her skal det stå "Hjelp til " (og PROGNAVN), samt stå et par setninger
eller mer om hvordan nettstedet virker.

For at det skal være tydelige visuelle grenser mellom topp, midtparti (hovedinnhold) og bunn,
ønskes en horisontal linje <HR> nederst i topp og øverst i bunn.
Denne linjen skal altså ikke stå i hoveddelen.

Lever en pakkefil med nettstedet, slik det nå fremstår
(gjøres vel enklest med å pakke mappa "kryptovaluta").
</pre>


<h1>Oppgave Besvarelse</h1>
<pre>
Jeg øvde litt på OOP da jeg gjorde denne oppgaven og lagde
en klasse i felles.php som de andre filene kan "instantiate".

Legg kildekoden (alle filene utenom denne, i webroot)
index.php åpnes automatisk
</pre>
