<?php
session_start();
require_once 'felles.php';
Session::end();
echo 'Utlogget. ';
echo '<a href="login.php">Logg inn igjen</a>';

 ?>
