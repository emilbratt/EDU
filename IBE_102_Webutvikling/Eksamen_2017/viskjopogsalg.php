<?php
session_start();
include 'felles.php';
include 'aksjer.php';
Session::check_logged_in();
Session::check_inactivity();

echo '<ul>';
foreach($aksjer as $k => $arr) {
    $str = $k . ' ';
    $str .= $arr[0] . ': ';
    $str .= 'Kjøpt: ' . $arr[1] . ' ';
    $str .= 'Solgt: ' . $arr[2] . ' ';
    $str .= '(' . strval($arr[1] - $arr[2]) . ')';
    echo '<li>'.$str.'</li>';

}
echo '</ul>';
