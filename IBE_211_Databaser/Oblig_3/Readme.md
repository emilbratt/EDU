<h1>Oppgave Beskrivelse</h1>
<pre>
Målet med denne øvingen er å trene på å lage script (program),
der du får trent på de nye SQL-kommandoene i kapitlet.

Jobb gjerne i grupper. Lever i så fall som gruppe.

Lag scriptet "oblig3.sql" som i rekkefølge utfører oppg. 1 (a-d) og oppg. 2 (a og e)
fra kap. 3 i læreboka. Se "Filer" for PDF av oppgavene.

Jeg skal fex kunne skrive mysql ... < oblig3.sql evt kjøre scriptet i phpmyadmin.

Programmet skal i følgende rekkefølge opprette tabeller (deloppgave 1a), legge inn en bil (1b),
endre hk (1c), fjerne bilen (1d).  Dernest (på de neste linjer i scriptet) følger det som trengs
som svar til 2 a og e, der du skal bruke ALTER for å endre Bilmodell og INSERT for å få inn
de tillatte verdiene som er listet i 2a.

Programmet skal ikke gi feilmelding.  Derfor må det starte med å slette databasen,
for så å lage databasen og velge den (DROP, CREATE og USE).

Som svar på oblig laster du opp filen oblig3.sql (ren tekstfil, ikke Word, PDF, e.l).

NB: Noen kan oppleve fremmednøkler som ignoreres



OPPGAVER FRA BOKA
1.)
Betrakt følgende database med aggregerte data om bilsalg:
Bilmodell(Bilmerke, Bilmodell, Gruppe, AntallHK)
Bilsalg(Bilmerke*, Bilmodell*, KommNr*, År, Måned, Antall)
Kommune(KommNr, KommNavn)
Én rad i tabellen Bilsalg inneholder antall solgte biler for en
bestemt bilmodell i en bestemt kommune en bestemt måned i et
bestemt år. Merk for øvrig den sammensatte fremmednøkkelen
fra Bilsalg til Bilmodell.
Utfør følgende oppgaver med SQL:

(a) Definer tabellene. Velg datatyper selv, og husk
primærnøkler og fremmednøkler. AntallHK, Antall og
KommNavn skal ikke ha nullmerker. Sistnevnte skal heller ikke
inneholde repetisjoner.

(b) Registrer en ny bilmodell.

(c) Endre antall hestekrefter for bilmodellen du registrerte i 1b.

(d) Slett bilmodellen du registrerte i 1b.

2.)
Definer følgende valideringsregler med SQL:
(a) Bilmodell.Gruppe skal være enten personbil, varebil,
lastebil eller buss.

(b) Bilmodell.AntallHK skal være over 0 og under 300.

(c) Bilsalg.År skal være mellom 1980 og 2100.

(d) Bilsalg.Måned skal være mellom 1 og 12.

(e) Kolonnen Bilmodell.Gruppe skal bare kunne inneholde
verdier fra en gitt liste med verdier. Hvordan kan dette gjøres
ved hjelp av fremmednøkler?
</pre>
