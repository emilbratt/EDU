<?php
require 'lib.php';
echo HTML::start('oppgave 3 hjelp');


echo <<<EOT
<ul>
 <li>Bygningskodene er A, B eller C.</li>
 <li>Etasjene er Kj (for kjeller), Loft, 1, 2 eller 3.</li>
 <li>Romnummer er et løpenummer fra 0 og opp.</li>
 <li>Pluggnummeret er 1, 2, 3, 4, 5, 6, 7 eller 8.</li>
</ul>
<p>Eksempel</p>
<ul>
 <li>A2-144-3 betyr plugg nummer 3 i rom nummer 144 i 2, etasje av Bygg A.</li>
 <li>CLoft-2-1 betyr plubb nummer 1 i rom nummer 2 på loftet i Bygg C.</li>

</ul>
EOT;
echo HTML::end();
