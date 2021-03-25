<h1>Oppgave Beskrivelse</h1>
<pre>
Øvelsen er frivillig, men legges inn som en oppgave.
Målet er å få øvd på kap 8 om å skrive og lese mot fil + å få en smakebit på AJAX,
som er en mye brukt Javascript-teknikk som læreboka bare såvidt nevner.

Vi ønsker en enkel utvidelse av nettstedet for kryptovaluta.
De som er pålogget kan skrive til en chat og se hverandres skriverier.
En enkel frittstående chat ble vist i forelesningen 11/3.

Kildekoden vil avsløre en del. Dette er en utvidet variant med tidsstempel og brukerID,
samt automatisk oppdatering av innleggene.


NYTT INNLEGG
Lag "chat.php" som nåes fra hovedmenyen i index.php.
På chatsiden vises et felt der brukeren skriver inn sitt innlegg.
Under tekstfeltet vises chatmeldingene, som lagres i "chat.txt".

Når man trykker <enter> blir innlegget sendt til "chatinnlegg.php" som legger det nye innlegget sist i "chat.txt".
Deretter skal chattesiden (referer) automatisk lastet inn. Det kan gjøres med header (det er beskrevet i læreboken):

<strong>
// chatinnlegg.php
// legg innlegget inn i chat.txt
header('Location: ' . $\_SERVER['HTTP_REFERER']);
</strong>
Tekstfilen "chat.txt" har 1 linje per innlegg.
Fint om de vises nyeste-først.
Også fint om bare de 10 siste blir vist.


AUTOMATISK OPPDATERING

For å få se meldinger fra andre samtidige chattere, må en nå relaste siden,
som er litt "gammeldags".  Mer brukervennlig er å oppdatere automatisk.
Følgende utvidelse av "chat.php" vil gjøre at chatmeldingene oppdateres hvert 1000 millisekund.
Til dette trengs AJAX og et navngitt område.
<strong>
&lt;script&gt;
  vischat();  // utføres nå
  setInterval (vischat, 1000);  // deretter hvert sekund
  function vischat() {   // https://www.w3schools.com/js/js_ajax_intro.asp
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
        document.getElementById("chatmeldinger").innerHTML = this.responseText;
      }
    };
    xhttp.open("GET", "chathent.php", true);
    xhttp.send();
  }

&lt;/script&gt;
&lt;div id="chatmeldinger"> &lt;/div&gt;
</strong>

For at denne skal virke må du lage "chathent.php" som skriver ut meldingene.
Her er en eksempelkjøring av kun chathent.php (Lenker til en ekstern side.) .

MER DETALJERT MELDINGSINFO

Som vist i eksempelkjøringen kan en også vise tidspunkt og hvem som skrev meldingen,
her i en tabell med tre kolonner.
Hver linje i "chat.txt" kan da være &lt;tid&gt; &lt;brukerID&gt; &lt;innlegg>, der tid hentes med time()
eller date() og brukerID ligger i sesjonsvariablene.
</pre>

<h1>Oppgave Besvarelse</h1>
<pre>
Last inn alle php filene samt kontoer.json i din webrot-mappe.
PHP må ha skrivetilgang til webrotmappa da den genererer en
fil "chat.txt" som brukes til å lagre chattemeldinger.
</pre>
