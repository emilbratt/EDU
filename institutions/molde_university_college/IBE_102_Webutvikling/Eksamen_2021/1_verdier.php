<?php
// verdi.php, kandidat 047
require 'lib.php';
echo HTML::start();
function wide_line() {
    return <<<EOT
    <hr>
    EOT;
}

$verdier = array(3, 1, 14, 8, 19, 5, 14);
$sum = 0;
$count = 0;

echo 'Verdiene er: ';
foreach($verdier as $n) {
    echo strval($n) . ', ';
    $sum += $n;
    $count ++;
}
$avg = $sum/$count;
echo '<br>Snitt: ' . $avg;

echo wide_line();

echo 'Verdiene vises sortert, og med grønn farge for de som er under snittet:<br>';
sort($verdier);
foreach($verdier as $n) {
    $col = 'red';
    if($n < $avg) {
        $col = 'green';
    }
    echo '<div id="inline" class="'.$col.'">'. strval($n) . ', </div>';
}

echo wide_line();

$verdier = array(3, 1, 14, 8, 19, 5, 14);
echo 'Hver verdi er vist med antal stjerner:<br>';

foreach($verdier as $n) {
    $num = 1;
    $star = null;
    do {
        $star .= '*';
    }
    while ($n > $num++);

    $col = 'red';
    if($n < $avg) {
        $col = 'green';
    }
    echo $star . '(' . strval($n) . ')' .'<br>';

}
echo wide_line();

echo 'Hver verdi er vist med antal stjerner:<br>';

foreach($verdier as $n) {
    $num = 1;
    $star_sub_avg = null;
    $star = null;

    do {
        if($n < $avg) {
            $star_sub_avg .= '*';
            continue;
        }
        $star .= '*';
    }
    while ($n > $num++);

    $col = 'red';
    if($n < $avg) {
        $col = 'green';
    }
    $number = strval($n);
    echo <<<EOT
    <div id="inline" class="green">$star_sub_avg</div><div id="inline" class="red">$star</div><div id="inline">($number)</div><br>
    EOT;

    // echo '<div id="inline" class="'.$col.'"></div>';
}


echo wide_line();


echo HTML::end();
