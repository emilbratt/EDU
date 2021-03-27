<h1>Oppgave Beskrivelse</h1>
<pre>
Vi skal her levere logisk design for forrige oppgave.
<br>
Forrige oppgave:
Konsulentene i en bedrift leverer timelister en gang i måneden.
En konsulent jobber for flere kunder i løpet av en måned.
For hver sammenhengende arbeidsperiode for samme kunde,
noterer konsulenten antall arbeidstimer.

Lag datamodell i form av ER-diagram basert på beskrivelsen over."
</pre>

<h1>Oppgave Besvarelse</h1>
<pre>
Jeg har tatt høyder for at det ikke skal forekomme redundans ved
å benytte 4 tabeller. Nøkler på hver av kolonnene som spesifiserer
sin id sørger for integritet mellom tabellene.
Det er bl.annet et mange-til-mange forhold fra tabellen "oppdrag_timeliste"
til oppdrag og konsulent tabellene som et resultat av normalisering.
Her vil vi da kunne fylle inn flere rader med arbeidsøkter og timer.
Jeg har også laget en komposittnøkkel på denne tabellen.
Denne primærnøkkelen tvinger minst en ulihet mellom id_konsulent,
id_oppdrag og dato slik at en konsulent ikke kan legge inn flere
rader for samme oppdrag på samme dato da dette er uhensiktsmessig.
Vi har ingen gjentagende data da oppdrag, oppdragstittel, dato
for start og dato for slutt er i sin egen tabell "oppdrag".
Konsulent-data er i tabellen "konsulent"og kunde-data er i sin
egen tabell som heter "kunde".

Jeg legger ved en mwb fil som kan åpnes i MySQL workbench som viser
det logiske designet visuelt. (Har ikke tatt med kompositnøkkel på denne..)

Filen run.sql er et script som lager denne databasen ut ifra
det logiske designet og legger inn noen få rader med data.
Den gjør noen SELECT spørringer for å vise hvordan den er
ment å skal fungere i praksis.
</pre>

<h1>Tabeller/Entiteter</h1>
<pre>
<strong>konsulent</strong>
id_konsulent (pk) (en-til-mange mot oppdrag_timeliste)
fornavn
etternavn
<br>
<strong>oppdrag_timeliste</strong>
id_oppdrag (pk) (mange-til-en mot oppdrag)
id_konsulent (pk) (mange-til-en mot konsulent)
dato (pk)
timer_fra
timer_til
beskrivelse
<br>
<strong>oppdrag</strong>
id_oppdrag (pk) (en-til-mange mot oppdrag_timeliste)
id_kunde (fk) (mange-til-en mot kunde)
navn
dato_start
dato_slutt
<br>
<strong>kunde</strong>
id_kunde (pk) (en-til-mange mot oppdrag)
fornavn
etternavn
</pre>
