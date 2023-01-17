-- oblig5.sql. IBE211. Dato: 05.03.2021 Student: Emil Bratt Børsting

USE Friidrett;

-- (a)
-- Vis alle Medlemmer med navn og klasse, der klasse er
-- sammensatt av kjønn og alder for de under 18 år (K14 for
-- jenter som er 14 år gamle), og lik kjønn for resten (K og M).
-- NB Siden oppgaven og datasettet er noen år gammel så setter vi alderen høyere..


-- Vi benytter CASE/WHEN slik at vi kan behandle rader hvor alderen er under 19

-- Vi bruker CONCAT for å redigere resultatet på kolonnen Kjønn for å legge til
-- riktig aldersklasse for medlemmer under 19

SELECT
    Fornavn,
    Etternavn,
    CASE
        WHEN
            Fødselsår >= YEAR(CURDATE())-19
        THEN
            CONCAT(Medlem.Kjønn, YEAR(CURDATE())-Medlem.Fødselsår)
        ELSE
            Kjønn
    END
        AS Klasse

FROM
    Medlem
ORDER BY
    Fødselsår DESC
;


-- Her har vi en litt mer detaljert oversikt

SELECT
    Medlem.Fornavn,
    Medlem.Etternavn,
    Medlem.Kjønn,
    CASE
        WHEN
            Fødselsår >= YEAR(CURDATE())-19
        THEN
            CONCAT(Medlem.Kjønn, YEAR(CURDATE())-Medlem.Fødselsår)
        ELSE
            Kjønn
    END
        AS Klasse,
    Resultat.LøpsNr,
    Resultat.Tid
FROM
    Medlem,
    Resultat
WHERE
    Medlem.MedlemsNr = Resultat.MedlemsNr
ORDER BY
    Medlem.MedlemsNr
;




-- (b)
-- Vis dem under 18 år som løp raskere enn gjennomsnittet i
-- sin klasse i et bestemt løp.
-- NB jeg setter under 24 år for å få med flere deltagere


-- Denne består av en SELECT + en DELSPØRRING i WHERE klausulen
-- Dette må til for å først regne ut snitt på utøvere som faktisk har
-- deltatt og som også oppfyller krav om å være under 24 år

-- Hovedspørringen består av den samme oppbygningen som delspørringen
-- fordi vi ønsker å hente ut de samme utøvereme fra de samme løpene
-- med utgangspunkt i de samme resultatene hvor forskjellen er at i
-- hovedspørringen skal vi filtrere ut de utøverene som har bedre
-- tid enn resultatet som spyttes ut av delspørringen viser
SELECT
    Medlem.Fornavn,
    Medlem.Etternavn,
    CASE -- her setter vi inn litt mere informasjon om utøveren og klassen
        WHEN
            Fødselsår >= YEAR(CURDATE())-19
        THEN
            CONCAT(Medlem.Kjønn, YEAR(CURDATE())-Medlem.Fødselsår)
        ELSE
            Kjønn
    END
        AS Klasse,
    MIN(Resultat.Tid) AS Beste_Tid
FROM
    Medlem
INNER JOIN
    Resultat
ON
    Medlem.MedlemsNr = Resultat.MedlemsNr
INNER JOIN
    Løp
ON
    Resultat.LøpsNr = Løp.LøpsNr
WHERE
    Fødselsår >= YEAR(CURDATE())-24 AND
    Løp.Distanse = '800' AND
    Resultat.Tid < (
        -- spørringen her kobler sammen samme tabeller
        -- fordi vi MÅ bruke de korrekte tidene, løpene og utøverene
        -- få et korrekt gjennomsnitt som vi kan basere den øvre
        -- spørringen på
        SELECT
            AVG(Resultat.Tid)
        FROM
            Medlem
        INNER JOIN
            Resultat
        ON
            Medlem.MedlemsNr = Resultat.MedlemsNr
        INNER JOIN
            Løp
        ON
            Resultat.LøpsNr = Løp.LøpsNr
        WHERE
            Fødselsår >= YEAR(CURDATE())-24 AND
            Løp.Distanse = '800'
    )
GROUP BY
    Medlem.MedlemsNr
ORDER BY
    Beste_Tid ASC
;




-- (c)
-- Vis navnet til alle som har vunnet minst ett løp.


-- For å løse denne bruker vi venstre ytterkobling fra to spørringer

-- Vi finner beste resultater fra en tabell ved å bruke MIN() funksjonen
-- uavhengig hvem som har denne tiden

-- Nå har vi altså en tabell som viser løpsnummer og den tiden som er best
-- på hver løp

-- Vi slår sammen denne tabellen med resultatet av spørringen mot samme
-- tabeller og sammenfletter resultatet av denne spørringen
-- med å koble sammen den forrige spørringen i ytre venstrekobling

-- Vi benytter alias fordi vi må spørre mot samme tabeller i begge koblingen

SELECT
    Beste_tider.MedlemsNr,
    Medlemmer.Fornavn,
    Medlemmer.Etternavn,
    Beste_tider.LøpsNr,
    Medlemmer.Distanse,
    Beste_tider.Bestetid
FROM (
    SELECT
        Resultat.LøpsNr,
        Beste.Bestetid,
        Resultat.MedlemsNr
    FROM
        Resultat, (
            SELECT
                Resultat.LøpsNr,
                MIN(Resultat.Tid) AS Bestetid
            FROM
                Resultat
            GROUP BY
                LøpsNr
        ) AS Beste
    WHERE
        Beste.LøpsNr = Resultat.LøpsNr AND
        Beste.Bestetid = Resultat.Tid

    ) Beste_tider
LEFT JOIN (
    SELECT
        Medlem.MedlemsNr,
        Medlem.Fornavn,
        Medlem.Etternavn,
        Løp.Distanse
    FROM
        Medlem
    INNER JOIN
        Resultat
    ON
        Medlem.MedlemsNr = Resultat.MedlemsNr
    INNER JOIN
        Løp
    ON
        Resultat.LøpsNr = Løp.LøpsNr
    GROUP BY
        Medlem.MedlemsNr
) Medlemmer
ON
    Beste_tider.MedlemsNr = Medlemmer.MedlemsNr
ORDER BY
    Beste_tider.LøpsNr
;




-- (d)
-- Vis utøvere som ikke har deltatt i noe løp.


-- Ser ut som om alle har deltatt så vi legger til vår helt fra den andre siden
-- av dammen slik at vi får minst et resultat og kan bekrefte at spørringen fungerer

INSERT IGNORE INTO Medlem (
    MedlemsNr,
    Fornavn,
    Etternavn,
    Kjønn,
    Fødselsår
) VALUES (
    '420',
    'Jostein',
    'Bæver',
    '$',
    '1994'
)
;

-- Vi kan enkelt bruke resultatsettet fra WHERE spørringen mot og
-- resultat-tabellen og dermed teste om vårt nye medlem med sitt
-- medlemsnummer er oppført i denne

-- Med andre ord: Ingen resultat == Ingen deltagelse i løp

SELECT
    *
FROM
    Medlem
WHERE
    Medlem.MedlemsNr NOT IN (
        SELECT
            MedlemsNr
        FROM
            Resultat
    )
;





-- (e)
-- Vis utøvere som har deltatt i samtlige 400-metersløp siste år.


-- Her må vi bruke en nøsted NOT EXISTS slik at det evaluerer til TRUE
-- Kan lese mer om dette her: https://dev.mysql.com/doc/refman/8.0/en/exists-and-not-exists-subqueries.html

-- UTDARG FRA NETTSIDEN: The last example is a double-nested NOT EXISTS query.
-- That is, it has a NOT EXISTS clause within a NOT EXISTS clause.
-- Formally, it answers the question “does a city exist with a store that is not in Stores”?
-- But it is easier to say that a nested NOT EXISTS answers the question “is x TRUE for all y?”


-- Håper vi slipper slike spørringer på eksamen. Vi er tross alt bare mennesker

SELECT
    MedlemsNr,
    Fornavn,
    Etternavn,
    Kjønn,
    Fødselsår
FROM
    Medlem
WHERE
    NOT EXISTS (
        SELECT
            LøpsNr
        FROM
            Løp
        WHERE
            Løp.Distanse = '400' AND NOT EXISTS (
                SELECT
                    LøpsNr
                FROM
                    Resultat
                WHERE
                    Løp.LøpsNr = Resultat.LøpsNr AND
                    Medlem.MedlemsNr = Resultat.MedlemsNr
            )
    )
;


-- (f)
-- Vis navn og alder for de 10 beste i et bestemt løp.


-- Denne blir enklere da vi har LøpsNr som en egen identitet i tabellen
-- hvor også resultatene og identifikatoren til medlemmene er registrert

-- Vi må likevel koble sammen tabellen Medlem for å få med navn på utøverne
SELECT
    Medlem.Fornavn,
    YEAR(CURDATE())-Medlem.Fødselsår AS Alder
FROM
    Medlem
INNER JOIN
    Resultat
ON
    Medlem.MedlemsNr = Resultat.MedlemsNr
WHERE
    Resultat.LøpsNr = '2'
ORDER BY
    Resultat.Tid
LIMIT
    10
;







-- (g)
-- Flytt alle løpsresultater eldre enn 3 måneder til en
-- historikktabell (med samme struktur som Resultat).


-- Først lager vi tabellen som følger ca samme begrensninger som Resultat-tabellen
-- Her bruker vi bl.annet akkurat samme datatype, noe annet ville vært tøys og tull

CREATE TABLE IF NOT EXISTS Resultat_historikk (
    LøpsNr      INT(11) NOT NULL,
    MedlemsNr   INT(11) NOT NULL,
    Tid         FLOAT
)
;


-- Vi bruker en SELECT i INSERT INTO for å flytte alle rader som oppfyller kriteriet
-- i WHERE klausulen og lar resten forbli i tabellen
INSERT INTO Resultat_historikk
    SELECT
        *
    FROM
        Resultat
    WHERE
        LøpsNr IN (
            SELECT
                LøpsNr
            FROM
                Løp
            WHERE
                DATEDIFF(CURDATE(), Dato) > 90
        )
;


-- Nå kan vi fjerne alle rader som har samme løps-id hvor hver rad med
-- sitt løp har blitt overflyttet til tabellen: Resultat_historikk

-- På denne måten har vi ingen duplikater i databasen friidrett

DELETE FROM
    Resultat
WHERE
    LøpsNr IN (
        SELECT
            LøpsNr
        FROM
            Resultat_historikk
    )
;
