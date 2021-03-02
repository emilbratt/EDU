<?php
include_once 'felles.php';
$libMain = new Main();
$libMain->include('header');
echo  '
<h2>Hjelp til '.$libMain::PROGNAVN.'</h2>
<p>Hvis du har problemer, hjelper vi deg gjerne.<br>Mvh IT support</p>

<p>Dette nettstedet er ikke ekte<br>
Nettstedet ble laget av Emil Bratt Børsting som del av et arbeidskrav i faget IBE 102 Webutvikling<br>
Du kan jo uansett prøve å kontakt oss på kryptoas@vivetbest.no, men jeg tviler på at du får noe svar</p>
';
$libMain->include('footer');
?>
