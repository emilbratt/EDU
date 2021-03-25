<h1>Oppgave Beskrivelse</h1>
<pre>
IBE120 Obligatorisk øving 6
Du skal levere en zip-fil. Zip-filen skal inneholde en regnearkfil og en word-fil.

I en slakteribedrift med filialer flere steder får de hvert år inn rapport over
antall kg levert slakt fra de ulike slakteriene i kjeden.
Dataene kommer som en tekstfil eller som en tabell i en Access-database.
Vedlagt finner du to eksempel på slike filer. Slakteriet tar imot slakt av 15 ulike dyreslag.
Vi skal lage en applikasjon som leser data og deretter gjør noen beregninger og viser informasjon
som fins i de importerte dataene.

a) På en arkfane som du kaller oversikt, skriver du inn teksten du ser nedenfor, og lager en
knapp som skal kjøre makroene vi skal lage i oppgavene som følger. Brukeren skal taste inn
data i de oransje cellene.

b) Lag en makro som leser inn tekstfilen eller databasetabellen til en arkfane i regnearket.
Du velger om du vil importere data fra Access eller fra csv-filen,
det er ikke nødvendig å gjøre begge deler.
Plasserer dataene på en ny arkfane som du kaller data.

Du velger om du vil bruke regnearkformler eller løkker og summering.
Dyreslag og sum skal skrives i kolonne A og B på arkfanen oversikt under de overskriftene du har laget.

c) Lag en makro som lager en oversikt over antall kg som er levert til slakt totalt for hvert av
dyreslagene. Du velger om du vil bruke regnearkformler eller løkker og summering. Dyreslag
og sum skal skrives i kolonne A og B på arkfanen oversikt under de overskriftene du har laget.

d) Utvid makroen fra oppgave c slik at den også viser antall kunder som har levert slakt og
gjennomsnittlig antall kg per kunde for hvert av dyreslagene. Du skal ikke ta med leveranser
der det er registrert 0 kg.

e) Lag ett eller flere diagram som viser informasjonen som fins i tabellen du har laget med
makroer.

f) Beskriv informasjonen dataene gir deg med en tekst på ca. 100 ord.
</pre>

<h1>Læremål</h1>
<pre>
Sette inn knapp i regnearket og koble makro til denne

Hvis databasefilen er valgt: skrive kode som henter data fra
en tabell eller spørring i en database

Hvis csv-fil er valgt: skrive kode som leser data fra en csv-fil

Hvis csv-fil er valgt: splitte tekst som er lest inn som en linje
fra en csv-fil, og sette de enkelte dataene inn i regnearket

Hvis regnearkformler er valgt: Bruke regnearkformler til
beregninger i en makro

Hvis løkker er valgt: Kunne skrive kode som bruker løkke
til å summere og telle, og deretter regne ut gjennomsnitt.

Kunne håndtere ulike regnearkfaner med kode.

Kunne lage informative og ryddige diagram

Kunne velge fornyftig diagramtype til å presentere data

Kunne beskrive hvilken informasjon tabell og diagram gir oss.
</pre>

<h1>Oppgave Besvarelse</h1>
<pre>
Svarene ligger i regnearket som heter: oppg6.xlsm
Jeg valgte å importere fra Access og dermed så utelater jeg csv-filen.
Besvarelse på oppgave f ligger i oppg_f.txt
Sørg for at access databasen ligger i samme sti som excel-fila.
Makro som er brukt i oppgavene er også skrevet i egen fil: makro.bas
Kommentering av makro er gjort på engelsk
</pre>
