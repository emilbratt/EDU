<?php
session_start();
include 'felles.php';
include 'aksjer.php';
Session::check_logged_in();
Session::check_inactivity();

echo '<p>Alle tickere:</p>';
foreach($aksjer as $k => $arr) {
    echo '<img src="/' . $arr[0] . '">';
    echo $k;
}
