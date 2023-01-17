<h1>Oppgave Beskrivelse</h1>
<pre>
Dette blir for mange en vanskelig oppgave. Lurt å begynne tidlig.

For å øve hukommelse (kap. 7) skal du lage innlogging og utlogging til valutasidene dine.
Innhold skal kun vises til de som er innlogget.
Du skal bruke sesjonsvariabler, og får dermed dessverre ikke øvd på GET,
skjulte elementer og cookies, selv om dette er del av pensum.

Lag "login.php", som enten viser et skjema for bruker og passord, eller behandler utfylt skjema.
Det skal godta brukere "Per" og "Kari" med passord hardkodet som "1111" og "2222".
Etter godkjent innlogging vil login.php vise "Trykk her for å gå videre",
som lenker til hovedsiden index.php.
Se eksempelkjøring: http://webutvikling.stud.himolde.no/~kd/IBE102/kryptovaluta/

Frivillig: Om du ønsker å være mer avansert og søke gjennom en tabell med registrerte
brukernavn og passord er det selvsagt helt greit.

En algoritme som kan brukes for "login.php" er:
<strong>
(start algoritme)
start sesjon (må alltid gjøres først!)
hvis bruker har fylt ut skjema:
   hvis godkjent bruker og passord (kombinasjon)
      sett/husk sesjonens "brukerID" og "starttid"

hvis ikke innlogget (altså sesjonens brukerID ikke satt):
   vis skjema (brukernavn og passord)
(slutt algoritme).
 </strong>
Hukommelsen består da kun av to variabler:  "brukerID" som er navnet på den som er innlogget.
Og, "starttid" som husker når innloggingen ble godkjent.

Skriv om "topp.php" slik at den i tillegg viser "Innlogget som bruker <brukerID> siden <starttid>".
Hvis brukeren ikke er innlogget skal skjermen vise "Ikke innlogget", med ei lenke tilbake til login.php.
Tanken er at topp.php er inkludert fra alle andre sider, og dermed er det greit å legge tilgangskontroll dit.

Toppen skal også vise "Logg ut", ei lenke til "loggut.php" som vil slette sesjonsdata,
slik at brukeren MÅ logge seg inn igjen for å fortsette.
</pre>


<h1>Oppgave Besvarelse</h1>
<pre>
Legg alle filer med .php endelse + mappen logoer i webroot
Gyldige brukere er som nevnt i oppgaven.
    Brukernavn: Per
    Passord: 1111
    ..eller
    Brukernavn: Kari
    Passord: 2222

Det var veldig gøy å fortsette med OOP fordi denne oppgaven ga en ekstra utfordring.
Implementering av innlogging var litt knotete i starten, men gikk veldig greit
da jeg klarte å skjønne hvordan strukturen i OOP gjorde at dette føltes riktig
sett fra et logiskt perspektiv.

Et eksempel jeg liker å belyse for hvorfor OOP ga god mening er at vi benytter konstruerings funksjonen
"\__construct()" for klassen som alle sidene initialiserer.
Ved å plassere @session_start() i construct så sikrer vi at alle sidene sjekker at den har lastet inn sesjonen.
Jeg lagde også en blokk som sjekker om innlogging er gyldig i denne. Da har vi slått 2 fluer i en smekk.
</pre>
