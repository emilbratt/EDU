<h1>Oppgave Beskrivelse</h1>
<pre>
Konsulentene i en bedrift leverer timelister en gang i måneden.
En konsulent jobber for flere kunder i løpet av en måned.
For hver sammenhengende arbeidsperiode for samme kunde,
noterer konsulenten antall arbeidstimer.

Lag datamodell i form av ER-diagram basert på beskrivelsen over."
</pre>

<h1>Oppgave Besvarelse</h1>
<pre>
I denne oppgaven har jeg kun tatt med det mest grunnleggende
for å komme i gang med en database for et bruksområde som er
etterspurt i oppgaven.
Se: filen mysql_workbench.mwb kan åpnes i MySQL Workbench.
<br>
<strong>Oversikt</strong>
Tabellen "konsulent" inneholder kun data om konsulentene som er ansatt
og vil være nok til å identifisere hver konsulent.

Tabellen "kunde" inneholder kun data om selve kunden og vil også identifisere
hver kunde uten andre avhengigheter.

Tabellen oppdrag vil ha en rad for hvert oppdrag. Tabellen skal kun vise
når oppdrag starter og avsluttes og hvilke oppdrag som hører til hvilke kunder.
En verdi i feltet "dato_slutt" vil også være tilstrekkelig
for å kunne identifisere dette oppdraget som "ferdig".

Tabellen "oppdrag_timeliste" inneholder beskrivende data om hvert oppdrag.
Tabellen danner et mange-til-mange forhold som en naturlig følge
av at den knytter to ulike par av fremmednøkler til hver sine respektive primærnøkler
i "oppdrag" og "konsulent" tabellene.
Meningen er at denne skal fylles inn hver dag (hver gang oppdrag for bestemt dag
utføres).
Da fylles også antall timer som blir regnet ut ved å føye inn klokkeslett start og
klokkeslett slutt i tillegg til en datostempling fra konsulenten.
Det er også i denne sammenhengen mulig for en annen konsulent å fortsette på et
oppdrag da det ikke eksisterer noen begrensning mellom hvilke konsulenter som
er knyttet til hvilke oppdrag
Anngående månedsinnlevering..
Jeg kunne sikkert brukt egne kolonner for "år" og "måned". Dette ville
gjort spørringene enklere å konstruere (og kanskje raskere) samt gjort utvikling mot
et tredjeparts brukergrensesnitt enklere. Jeg føler at det er mere hensiktsmessig
å bruke dato sammen med dato-datatype slik at man heller da løseer den månedlige
biten gjennom applikasjonslogikken til et eventuelt brukergrensesnitt som blir
bygget for denne databasen. Dette viller vært noe å evaluere avhengig av
om skalering/tilpassing kontra utviklingstid er av interesse for oppdragsgiver.


Tabeller (Entiteter):
    konsulent
        id_konsulent (pk) (en-til-mange mot oppdrag_timeliste)
        fornavn
        etternavn

    oppdrag_timeliste
        id_oppdrag (fk) (mange-til-en mot oppgrad)
        id_konsulent (fk) (mange-til-en mot konsulent)
        dato
        timer_fra
        timer_til
        beskrivelse

    oppdrag
        id_oppdrag (pk) (en-til-mange mot oppdrag_timeliste)
        id_kunde (fk) (mange-til-en mot kunde)
        navn
        dato_start
        dato_slutt

    kunde
        id_kunde (pk) (en-til-mange mot oppdrag)
        fornavn
        etternavn
</pre>

<figure>
    <figcaption style="margin-left: 10px">ER-Diagram</figcaption>
    <img src="screen_shot.png" alt="Image not found" style="width:50%">
</figure>
