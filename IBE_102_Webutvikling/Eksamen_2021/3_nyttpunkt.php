<?php
require 'lib.php';
echo HTML::start('oppgave 3');

// FOR TESTING
// $val = 'A2-144-3';
// $val = 'CLoft-144-3';
// $val = 'CKj-144-2';
// $val = 'B3-144-3';
// $db = new Database();
// echo $db->insert($val);
// $cnxn = null;

// OPPGAVE STARTER HER

// SEND TIL hjelp.php HVIS KNAPP BLE TRYKKET
if(isset($_GET['help'])) {
    header('Location: 3_hjelp.php');
}
// LEGG INN NYTT PUNKT
if(isset($_GET['addr'])) {
    if($_GET['addr']) {
        $db = new Database();
        echo $db->insert($_GET['addr']);
    }
}

echo <<<EOT
<form action="" id="text_form" method="get">
<label for="tekst_get">Skriv inn addresse:</label>
<input type="text" name="addr" id="tekst_get">

<input type="submit" value="Legg til adresse">

<input type="submit" name="help" value="Help">
</form>
EOT;

echo HTML::end();
