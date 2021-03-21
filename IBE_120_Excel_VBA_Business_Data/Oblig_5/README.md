<h1>Oppgave Beskrivelse</h1>
<pre>
IBE120 Obligatorisk øving 5
Du skal levere en zip-fil. Zip-filen skal inneholde flere regnearkfiler med løsning på oppgavene.

Oppgave 1
Overvåking av miljøet er mer og mer aktuelt i byer i Norge. Databasen du skal hente data fra i denne
øvelsen heter miljodataStavanger.accdb og inneholder noen miljødata som er samlet inn av sensorer
i Stavanger. Det er en tabell med miljødata her:
Tabellen nilu som inneholder data om luftkvalitet.
Lokalisering av sensorene er i tabellen lokalisering_sensorer. Tabellen komponenter inneholder
informasjon om de komponentene i luften som måles. Tabellen farenivaa inneholder en farge som
kan brukes til å vise de tre farenivåene.
Det fins dessuten en spørring i databasen. Spørringen luftkvalitet_uke viser data om luftkvalitet fra
og med 25.januar til og med 31.januar.

Oppgave 1 a
Importer spørringen luftkvaltitet_uke. Sett inn en pivottabell med utgangspunkt i de importerte
dataene.
Lag et beregnet element som heter partikkelforurensning. Dette elementet skal være summen av
verdiene for Små partikler og Støvpartikler.
Lag et beregnet felt som viser verdien av forurensingen som milligram. Verdien i feltet value er i
mikrogram. Du gjør om til milligram ved å dele på 1000.
Vis en oversikt som den under. Brukeren skal kunne velge dag.

Oppgave 1b
Importer filen sykkel_uke.xml til en ny arkfane i regnearket.
Bruk pivottabell, og lag en oversikt over hvor mange syklister som har passert de ulike målepunktene
de ulike dagene.Oppgave 2
a) Importer tabellene nilu, komponenter, lokalisering_sensorer og farenivaa til en ny
regnearkfil. Skal importeres slik at de havner i regnearkets datamodell med koblingene som i
databasen.
I deloppgavene som følger skal du bruke pivottabell som henter data fra regnearkets datamodell.
Ha en arkfane per deloppgave.
b) Lag en pivottabell der brukeren kan velge dag med tidslinje, og få se gjennomsnittlig
måleverdi for de tre komponentene på hver av de to målestasjonene for luftkvalitet.
c) Lag en pivottabell som viser antall ganger de ulike farenivåene er målt for de ulike
komponentene. Vis data for en målestasjon av gangen.

Oppgave 3
Filen passasjerer.xlsx inneholder data om passasjertrafikk ved noen norske lufthavner i september en
del år. Filen er ikke så stor, den kunne vært mye større. Du skal tilrettelegge filen for analyse med
pivottabell.
a) Lag en makro som fyller ut tomme celler i A-kolonnen.
b) Tilrettelegg dataene for analyse med pivottabell. Du kan enten gjøre dette med en makro
eller bruke muligheter i Power-pivot.
Hvis du lager makro er dette en mulig framgangsmåte:
    •Sett inn to nye kolonner til høyre for B-kolonnen. (Dette kan du gjøre før du lager
    makroen)
Lag en eller flere makroer som gjør følgende:
    • Kopiere navnet på lufthavnene og flytype (kolonne A og B) tilstrekkelig mange ganger
    • Sette inn årstall i den nye C-kolonnen
    • Sette inn passasjertall i den nye D-kolonnen
c) Bruk Pivottabell til å finne ut hvilken lufthavn som har mest mest trafikk i 2018.
d) Finn, ved hjelp av pivottabell, hvor stor andel av passasjerene på Sola som var
helikopterpassasjerer og hvor stor andel som var flypassasjerer.
</pre>

<h1>Læremål</h1>
<pre>
Importere data fra en Access-database
Importere data fra en XML-fil
Lage og bruke et beregnet felt i en pivottabell
Kunne lage og bruke et beregnet element i en pivottabell
Importere data til regnearkets datamodell
Kunne finne gjennomsnitt og antall med pivottabell
Vite hvordan data skal være organisert for at de skal kunne brukes i en pivottabell
Lage en makro som fyller ut tomme celler i regnearket
Bruke Power Query eller makroer til å tilrettelegge data for behandling med pivottabell
Kunne vise beregnenede verdier som prosent
</pre>

<h1>Oppgave Besvarelse</h1>
<pre>
Hvert regneark er navngitt med oppgavenummer.

Arkfaner er navngitt med tilsvarende bokstav for hvert delspørsmål.

Makro som er brukt i oppgave 3a for klargjøring av pivottabell er
gjort tilgjengelig i egen fil -> oppg3_makro.bas

Kommentering av makro er gjort på engelsk
</pre>
