DROP DATABASE IF EXISTS anonseportal;
CREATE DATABASE anonseportal;
USE anonseportal;

CREATE TABLE soker_handling
(
    id_soker_handling   TINYINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    beskrivelse         VARCHAR(30)
);

INSERT INTO soker_handling
    (beskrivelse)
VALUES
    ('ny søknad'),
    ('send inn søknad'),
    ('aktiver søknad'),
    ('deaktiver søknad'),
    ('endre søknad'),
    ('trekk søknad'),
    ('slett søknad')
;

CREATE TABLE stilling_handling
(
    id_stilling_handling    TINYINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    beskrivelse             VARCHAR(30)
);

INSERT INTO stilling_handling
    (beskrivelse)
VALUES
    ('ny stilling'),
    ('send inn stilling'),
    ('aktiver stilling'),
    ('deaktiver stilling'),
    ('endre stilling'),
    ('trekk stilling'),
    ('stilling besatt'),
    ('slett stilling')
;

CREATE TABLE org_kontakt_handling
(
    id_org_kontakt_handling  TINYINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    beskrivelse         VARCHAR(30)
);


INSERT INTO org_kontakt_handling
    (beskrivelse)
VALUES
    ('søknad åpnet'),
    ('søknad vurdert'),
    ('søknad akseptert'),
    ('søknad avvist')
;

CREATE TABLE organisasjon
(
    org_nr          CHAR(9) PRIMARY KEY,
    firmanavn       VARCHAR(128) NOT NULL,
    postnr          CHAR(4),
    gate            VARCHAR(128),
    sted            VARCHAR(64)
);

CREATE TABLE org_kontakt
(
    id_org_kontakt      SMALLINT UNSIGNED AUTO_INCREMENT NOT NULL,
    org_nr              CHAR(9) NOT NULL,
    fornavn             VARCHAR(36),
    etternavn           VARCHAR(36),
    epost               VARCHAR(50),
    telefon             CHAR(8),

    PRIMARY KEY (id_org_kontakt),

    FOREIGN KEY (org_nr)
        REFERENCES organisasjon (org_nr)
);

CREATE TABLE stilling_kategori
(
    id_kat_stilling SMALLINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    beskrivelse     VARCHAR(48) NOT NULL
);

INSERT INTO stilling_kategori
    (beskrivelse)
VALUES
    ('selger'),
    ('IT'),
    ('omsorg'),
    ('offentlig'),
    ('veldedighet'),
    ('skipsfart')
;

CREATE TABLE stilling
(
    id_stilling         INTEGER UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    id_org_kontakt      SMALLINT UNSIGNED NOT NULL,
    opprettet           DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (id_org_kontakt)
        REFERENCES org_kontakt (id_org_kontakt)
);

CREATE TABLE stilling_data
(
    id_stilling             INTEGER UNSIGNED,
    id_kat_stilling         SMALLINT UNSIGNED NOT NULL,
    id_stilling_handling    TINYINT UNSIGNED DEFAULT '1',
    tittel                  VARCHAR(50),
    beskrivelse             VARCHAR(2048) NOT NULL,
    endret                  DATETIME ON UPDATE CURRENT_TIMESTAMP,
    sok_frist               DATE NOT NULL,

    FOREIGN KEY (id_stilling)
        REFERENCES stilling (id_stilling),

    FOREIGN KEY (id_kat_stilling)
        REFERENCES stilling_kategori (id_kat_stilling),

    FOREIGN KEY (id_stilling_handling)
        REFERENCES stilling_handling (id_stilling_handling)
);

CREATE TABLE soker
(
    id_soker    INTEGER UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    fornavn     VARCHAR(36) NOT NULL,
    etternavn   VARCHAR(36) NOT NULL,
    epost       VARCHAR(50),
    cv          VARCHAR(4096)
);

CREATE TABLE soker_data
(
    id_soker        INTEGER UNSIGNED NOT NULL,
    dokument        BLOB NOT NULL,

    FOREIGN KEY (id_soker)
        REFERENCES soker (id_soker)
);


CREATE TABLE soker_stilling
(
    id_soknad           INTEGER UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    id_soker            INTEGER UNSIGNED NOT NULL,
    id_stilling         INTEGER UNSIGNED NOT NULL,
    id_soker_handling   TINYINT UNSIGNED DEFAULT '1' NOT NULL,
    soknad_tekst        VARCHAR(4096) NOT NULL,
    cv_tilgang          TINYINT DEFAULT '0',
    dok_tilgang         TINYINT DEFAULT '0',
    dato_sok            DATETIME DEFAULT CURRENT_TIMESTAMP,
    endret              DATETIME ON UPDATE CURRENT_TIMESTAMP,

    FOREIGN KEY (id_soker)
        REFERENCES soker (id_soker),

    FOREIGN KEY (id_stilling)
        REFERENCES stilling (id_stilling),

    FOREIGN KEY (id_soker_handling)
        REFERENCES soker_handling (id_soker_handling)
);

CREATE TABLE aktivitet_soknad
(
    id_soknad               INTEGER UNSIGNED NOT NULL,
    id_soker                INTEGER UNSIGNED,
    id_soker_handling       TINYINT UNSIGNED,
    id_org_kontakt          SMALLINT UNSIGNED,
    id_org_kontakt_handling TINYINT UNSIGNED,

    FOREIGN KEY (id_soknad)
        REFERENCES soker_stilling (id_soknad),

    FOREIGN KEY (id_soker_handling)
        REFERENCES soker_handling (id_soker_handling),

    FOREIGN KEY (id_soker)
        REFERENCES soker (id_soker),

    FOREIGN KEY (id_org_kontakt)
        REFERENCES org_kontakt (id_org_kontakt),

    FOREIGN KEY (id_org_kontakt_handling)
        REFERENCES org_kontakt_handling (id_org_kontakt_handling)
);


SHOW TABLES;

SELECT 'Registrerer aktivitet gjort på søknader '
AS 'Tabell: aktivitet_soknad';

SELECT 'Data for kontakter innenfor en organisasjon'
AS 'Tabell: org_kontakt';

SELECT 'Gyldige handlinger for stillingsutlyser/organisasjonen'
AS 'Tabell: org_kontakt_handling';

SELECT 'Oppføring av alle registrerte organisasjoner'
AS 'Tabell: organisasjon';

SELECT 'Oppføring av alle registrerte søkere'
AS 'Tabell: soker';

SELECT 'Filopplastinger for søkere'
AS 'Tabell: soker_data';

SELECT 'Gyldige handlinger for søker'
AS 'Tabell: soker_handling';

SELECT 'Registrerer søknad og knytter denne til en stilling '
AS 'Tabell: soker_stilling';

SELECT 'Oversikt over alle stillinger'
AS 'Tabell: stilling';

SELECT 'Beskrivelse og detaljer for stillingen'
AS 'Tabell: stilling_data';

SELECT 'Gyldige handlinger for stillinger'
AS 'Tabell: stilling_handling';

SELECT 'Gyldige kategorier for stillingsutlysninger'
AS 'Tabell: stilling_kategori';
