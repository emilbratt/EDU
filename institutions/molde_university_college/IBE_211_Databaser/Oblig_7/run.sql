-- klargjør database
DROP DATABASE IF EXISTS oblig7;
CREATE DATABASE oblig7;
USE oblig7;

-- klargjør tabeller
CREATE TABLE konsulent
( -- hver rad = en konsulent
	id_konsulent 	INT AUTO_INCREMENT NOT NULL,
    fornavn         VARCHAR(36) NOT NULL,
    etternavn       VARCHAR(36) NOT NULL,

    CONSTRAINT  pk_konsulent
        PRIMARY KEY(id_konsulent)

);

CREATE TABLE kunde
( -- hver rad = en kunde
    id_kunde        INT AUTO_INCREMENT NOT NULL,
    fornavn         VARCHAR(36) NOT NULL,
    etternavn       VARCHAR(36) NOT NULL,

    CONSTRAINT pk_kunde
        PRIMARY KEY(id_kunde)
);

CREATE TABLE oppdrag
( -- hver rad = et oppdrag
    id_oppdrag      INT AUTO_INCREMENT NOT NULL,
    id_kunde        INT NOT NULL,
    tittel          VARCHAR(36) NOT NULL,
    beskrivelse     VARCHAR(1024) NOT NULL,
    dato_start      DATE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    dato_slutt      DATE, -- en NULL verdi vil også kunne tolkes som at oppdrag ikke er fullført

    CONSTRAINT pk_oppdrag
        PRIMARY KEY(id_oppdrag),

    CONSTRAINT table_oppdrag_fk_id_kunde
        FOREIGN KEY (id_kunde)
            REFERENCES kunde (id_kunde)
);

CREATE TABLE oppdrag_timeliste
( -- hver rad = en arbeidsdag for et oppdrag
    id_oppdrag      INT NOT NULL,
    id_konsulent    INT NOT NULL,
    dato            DATE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    time_fra        TIME DEFAULT CURRENT_TIMESTAMP,
    time_til        TIME DEFAULT CURRENT_TIMESTAMP,
    logg     VARCHAR(255),

    PRIMARY KEY
        (id_oppdrag, id_konsulent, dato),
    -- ikke nødvendig med en komposit primærnøkkel, men vi sikrer at det ikke blir
    -- to oppføringer på samme dato for å unngå redundans

    CONSTRAINT table_oppdrag_timeliste_fk_id_oppdrag
        FOREIGN KEY (id_oppdrag)
            REFERENCES oppdrag (id_oppdrag),

    CONSTRAINT table_oppdrag_timeliste_fk_id_konsulent
        FOREIGN KEY (id_konsulent)
            REFERENCES konsulent (id_konsulent)
);

-- legg til "dummy data"
INSERT INTO konsulent
    (fornavn,etternavn)
VALUES
(
    'bob','bobby'
),
(
    'jane','janey'
),
(
    'joe','joey'
),
(
    'joy','joylene'
)
;

INSERT INTO kunde
    (fornavn,etternavn)
VALUES
(
    'jack','jackson'
),
(
    'jill','gilligan'
)
;

INSERT INTO oppdrag
(
    id_kunde, tittel, beskrivelse
)
VALUES
(
    '1','Fotografere Bryllup',
    'Bryllupsfoto på Tryvann'
),
(
    '2','Flytteoppdrag',
    'jill skal fltte fra Trondheim til Molde, vi ordner frakta'
)
;

INSERT INTO oppdrag_timeliste
(
    id_oppdrag,
    id_konsulent,
    dato,
    time_fra,
    time_til,
    logg
)
VALUES
    (
        '1','1','2021-03-27','08:00:00','16:12:00',
        'gikk bra, fine bilder ble det og mye å gjøre'
    ),
    (
        '1','2','2021-03-27','08:10:00','16:15:00',
        'ja det var mye å gjøre, men fine bilder'
    ),
    (
        '2','1','2021-03-28','09:00:00','16:00:00',
        'flyttinga gikk fint, men kunne trengt hjelp av noen andre konsulenter'
    )
;

-- vis pågående oppdrag
SELECT
    oppdrag.tittel AS Tittel_pågående_oppdrag,
    oppdrag.beskrivelse
FROM
    oppdrag
WHERE
    dato_slutt IS NULL
;

-- vis alle oppdrag og timelister med all relevant info
SELECT
    kunde.fornavn AS Kunde,
    konsulent.fornavn AS Konsulent,
    oppdrag.tittel AS Tittel_alle_oppdrag,
    oppdrag_timeliste.logg AS Logg,
    oppdrag_timeliste.dato AS Dato

FROM
    oppdrag
INNER JOIN
    oppdrag_timeliste
ON
    oppdrag.id_oppdrag = oppdrag_timeliste.id_oppdrag
INNER JOIN
    kunde
ON
    oppdrag.id_kunde = oppdrag.id_kunde
INNER JOIN
    konsulent
ON
    oppdrag_timeliste.id_konsulent = konsulent.id_konsulent
ORDER BY
    oppdrag.id_oppdrag
;

-- oppdater oppdrag med dato_slutt på oppdrag med id = 1
UPDATE
    oppdrag
SET
    dato_slutt = '2021-03-28'
WHERE
    id_oppdrag = '1'
;

-- vis avsluttede oppdrag
SELECT
    oppdrag.tittel AS Tittel_fullførte_oppdrag,
    oppdrag.beskrivelse AS Beskrivelse
FROM
    oppdrag
WHERE
    dato_slutt IS NOT NULL
;

-- vis hvor lang tid bob har jobbet den 27 mars 2021
SELECT
    konsulent.fornavn AS Konsulent,
    oppdrag_timeliste.dato AS Dato,
    DATE_FORMAT(oppdrag_timeliste.time_fra, '%H:%i') AS Fra,
    DATE_FORMAT(oppdrag_timeliste.time_til, '%H:%i') AS Til,
    TIMESTAMPDIFF(
        minute,
        oppdrag_timeliste.time_fra,
        oppdrag_timeliste.time_til
    ) AS Minutter_203_mars_2021
FROM
    oppdrag_timeliste
INNER JOIN
    konsulent
ON
    oppdrag_timeliste.id_konsulent = konsulent.id_konsulent
WHERE
    oppdrag_timeliste.dato = '2021-03-27'
AND
    oppdrag_timeliste.id_konsulent = '1'
;

-- siden konsulentene leverer timeliste hver måned så må vi kunne
-- vise hvor lang tid bob har jobbet i f.eks hele mars 2021
SELECT
    konsulent.fornavn AS Konsulent,
    oppdrag_timeliste.dato AS Dato,
    DATE_FORMAT(oppdrag_timeliste.time_fra, '%H:%i') AS Fra,
    DATE_FORMAT(oppdrag_timeliste.time_til, '%H:%i') AS Til,
    SUM(TIMESTAMPDIFF
        (
            minute,
            oppdrag_timeliste.time_fra,
            oppdrag_timeliste.time_til
        )
    ) AS Minutter_Mars_2021
FROM
    oppdrag_timeliste
INNER JOIN
    konsulent
ON
    oppdrag_timeliste.id_konsulent = konsulent.id_konsulent
WHERE
    MONTH(oppdrag_timeliste.dato) = '3'
AND
    YEAR(oppdrag_timeliste.dato) = '2021'
AND
    oppdrag_timeliste.id_konsulent = '1'
;
