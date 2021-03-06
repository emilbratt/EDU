<h1>Oppgave Beskrivelse</h1>
<pre>
Her skal du trene på varierte måter å organisere data på i matriser, samt gjenta litt om gjentakelser og HTML.

Først leser du kap 4.1 om matriser.  Resten av kap 4 kommer i neste modul.

Svaret leverer du som en fil "oblig3.php".
Samme krav til leveransen som i oblig 2.
Her er eksempelkjøring:
http://webutvikling.stud.himolde.no/~kd/IBE102/kryptovaluta/oblig3.php

Den viser først matrisens innhold med PRE og print_r( matrisenavn ).

Del 1:  Først skal du øve på det at info ligger spredt i
flere atskilte matriser.
Mer spesifikt, vi har alt i 5 matriser som har navn ticker,
navn, k1, k2, k3. Se lenger ned for innhold og struktur.
Du må lage koden som lager disse matrisene og deretter
viser innholdet med while.
Visingen skal være ett avsnitt (P) per valuta.

Del 2: Her skal du øve på at vi har flyttet all info
inn i en og samme matrise (som skal hete v,
se lenger ned for innhold og struktur i denne matrisen).
Vis innholdet av matrisen med foreach,
der hver valuta vises per punkt,
i en unummerert liste.

Del 3: Her skal du øve på assosiativ matrise der valutaens
ticker er nøkkel (blir vel samme som dictionary i Python).
Også her ligger all info i en matrise med navn v2,
men strukturering er litt annerledes (se eksempel lenger ned).
Her skal du skrive ut alle valutaene i en HTML tabell (bruk foreach),
en valuta per rad i tabellen.

OBS: Bruk array(...) for å opprette matrisene.

Frivillig:  Hver ticker skal ha en logo.
Disse lagres i mappe "kryptovaluta/logoer/<ticker>.png"
f.eks. "ETH.png" for valuta med ticker ETH.
Dermed kan logonavnet beregnes (trenger ikke lagre det)
og vises med IMG src=logoer/<ticker>.png.
</pre>

<h1>Oppgave Besvarelse</h1>
<pre>
Filen oblig3.php sammen med mappen "logoer er filene som jeg leverer.
Mappen "logoer" inneholder bildefiler som oblig3.php er avhengig av.
</pre>
