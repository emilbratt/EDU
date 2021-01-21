<pre>
Redigerer kap2_film.sql med teksteditor og legger til øverst i scriptet:
USE Hobbyhuset;
slik at tabellen Film blir laget i denne databasen.
Fjerner også kommentering på:
--DROP TABLE IF EXISTS Film;
slik at script kan kjøres om igjen for å "gjenoppbygge Film-tabellen"
hvis vi ødelegger/sletter data.
</pre>
