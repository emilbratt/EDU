-- Oppgave 3
    -- Student-ID ******
    -- Studentnavn: Emil Bratt Børsting (ikke i noen gruppe)

    -- VIKTIG
    -- forberedelse du må gjøre før du kjører script
        --endre verdi på linje 92 og 104

    -- for å kjøre script
    -- logg inn på din database-tjener og skriv: SOURCE /path/to/oppg3.sql

-- klargjøring av tom database
DROP DATABASE IF EXISTS Bil;
CREATE DATABASE Bil;
USE Bil;


-- primærnøkler for gruppe
CREATE TABLE Gruppe(
    gruppe_id   INT UNSIGNED    NOT NULL AUTO_INCREMENT,
    gruppe_navn VARCHAR(30)     NOT NULL,

    PRIMARY KEY
        (gruppe_id)
);

-- primærnøkler for kommune
CREATE TABLE Kommune(
    kom_id  CHAR(4)         NOT NULL,
    kom_navn VARCHAR(40)    NOT NULL,

    PRIMARY KEY
        (kom_id)
);


-- hoved-data for biler
CREATE TABLE Bilmodell(
    merke       VARCHAR(30)         NOT NULL,
    modell      VARCHAR(30)         NOT NULL,
    gruppe_id   INT UNSIGNED        NOT NULL,
    hk          SMALLINT   NOT NULL,

    PRIMARY KEY
        (merke, modell),

    CONSTRAINT
        Bilmodell_gruppe_id_FK
    FOREIGN KEY
        (gruppe_id)
    REFERENCES
        Gruppe (gruppe_id)
);

-- legge til en constraint som begrenser hk verdier
ALTER TABLE Bilmodell
    ADD CONSTRAINT Bilmodell_hk_limit
    CHECK (hk BETWEEN 1 AND 299);

-- data for salg
CREATE TABLE Bilsalg(
    merke   VARCHAR(30)         NOT NULL,
    modell  VARCHAR(30)         NOT NULL,
    kom_id  CHAR(4)             NOT NULL,
    aar     SMALLINT UNSIGNED,
    maaned  SMALLINT UNSIGNED,
    antall  SMALLINT UNSIGNED   NOT NULL,

    PRIMARY KEY
        (merke, modell, aar, maaned),

    CONSTRAINT
        Bilsalg_kom_id_FK
    FOREIGN KEY
        (kom_id)
    REFERENCES
        Kommune (kom_id),

    CONSTRAINT
        Bilsalg_aar_limit
    CHECK
        (aar BETWEEN 1980 AND 2100),
    --
    CONSTRAINT
        Bilsalg_maaned_limit
    CHECK
        (maaned BETWEEN 1 AND 12)
);

-- klargjør primærverdier for tabellen Kommune
LOAD DATA LOCAL INFILE
    '/path/to/Kommune.csv'
INTO TABLE
    Kommune
FIELDS TERMINATED BY
    ','
LINES TERMINATED BY
    '\n'
IGNORE
    1 ROWS;

-- klargjør primærverdier for tabellen Gruppe
LOAD DATA LOCAL INFILE
    '/path/to/Gruppe.csv'
INTO TABLE
    Gruppe
FIELDS TERMINATED BY
    ','
LINES TERMINATED BY
    '\n'
IGNORE
    1 ROWS;


-- siden vi legger inn kjøretøy med over 299 hk, så dropper vi constraint for hk
ALTER TABLE Bilmodell
    DROP CONSTRAINT Bilmodell_hk_limit;

-- legge inn rader i tabellen Bilmodell
INSERT INTO Bilmodell(
    merke, modell, gruppe_id, hk
    )
VALUES
    ('Toyota','Camry','1','70'),
    ('Mazda','Miata','1','120'),
    ('Honda','Civic','1','82'),
    ('Scania','KEB','4','450'),
    ('Volvo','B8','3','562'),
    ('Wolksvagen','Caravelle','2','160')
;

-- gjøre en select med join for å hente data om bilene
SELECT
    merke, modell, hk, gruppe_navn
FROM
    Gruppe
INNER JOIN
    Bilmodell
ON(
    Gruppe.gruppe_id = Bilmodell.gruppe_id
);

-- endre verdi i en rad i Bilmodell
UPDATE
    Bilmodell
SET
    hk = '75'
WHERE(
    merke = 'Toyota' AND modell = 'Camry'
);

-- gjøre en ny select med join for å vise raden vi endret
SELECT
    merke, modell, hk AS ny_hk, gruppe_navn
FROM
    Gruppe
INNER JOIN
    Bilmodell
ON(
    Gruppe.gruppe_id = Bilmodell.gruppe_id
)
WHERE(
    merke = 'Mazda' AND modell = 'Miata'
);

-- slett en rad fra tabellen Bilmodell
DELETE FROM
    Bilmodell
WHERE(
    merke = 'Toyota' AND modell = 'Camry'
);

-- gjøre en select med join for å vise at en rad er slettet
SELECT
    merke, modell, hk, gruppe_navn
FROM
    Gruppe
INNER JOIN
    Bilmodell
ON(
    Gruppe.gruppe_id = Bilmodell.gruppe_id
);


-- legge inn rader i tabellen Bilsalg
INSERT INTO
    Bilsalg(
        merke, modell, kom_id, aar, maaned, antall
    )
VALUES
    ('Toyota','Camry','0233','2021','01','1'),
    ('Honda','Civic','0821','2021','01','1')
;

-- gjøre en select på tabellen Bilsalg
SELECT
    merke, modell, kom_navn, antall
FROM
    Kommune
INNER JOIN
    Bilsalg
ON
    Kommune.kom_id = Bilsalg.kom_id
;
