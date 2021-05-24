<?php
session_start();
include 'felles.php';
Session::check_logged_in();
Session::check_inactivity();

echo '<h3>Du er innlogget som ' . $_SESSION['user'] . '</h3>' .
'<h3><a href="loggut.php">Logg ut</a></h3>';

echo '<a href="visalletickere.php"><h3>Vis alle tickere</h3></a>';
echo '<a href="viskjopogsalg.php"><h3>Vis kjøp og salg</h3></a>';

 ?>
