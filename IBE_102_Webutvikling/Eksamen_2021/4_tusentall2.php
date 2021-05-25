<?php
session_start();
require 'lib.php';
echo HTML::start('oppgave 4 med session');

// LAG MATRISE EM ALLE TALL 1 - 1000
$num = 1;
$numbers = array();
do {
    $numbers[strval($num)] = $num;
}
while (1000 > $num++);


// SJEKK HVILKE TALL SOM ER SENDT, VI BRUKER POST FORDI GET GJØR AT URL-
if(isset($_GET['run'])) {
    $sum = 0;
    $str = null;
    foreach($numbers as $k => $v) {
        if(isset($_GET[$k])) {
            $_SESSION[$k] = $v;
            $str .= $k . '+';
            $sum += $_GET[$k];
        }
    }
    if($str != null) {
        $str = rtrim($str, '+');
        echo $str.'=';
        echo $sum;
        echo '<hr>';
    }
}

echo '
<form action="" method="get">
<input type="hidden" name="run" value"true">
<input type="submit" value="summer">
';
foreach($numbers as $k => $v) {
    $k = strval($v);
    if(!(isset($_SESSION[$k]))) {
        echo <<<EOT
            <input type="checkbox" name="$k" value="$v">
            <label for="$k">$k</label>
        EOT;
    }

}
echo '
</form>
';
echo HTML::end();
