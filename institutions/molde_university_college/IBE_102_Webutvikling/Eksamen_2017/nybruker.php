<?php
session_start();
require_once 'felles.php';
if(isset($_POST['register'])) {
    if(isset($_POST['usr']) and isset($_POST['pwd']) and isset($_POST['pwd_']) ) {
        $cred = new Credential;
        if($cred->register($_POST['usr'], $_POST['pwd'], $_POST['pwd_'])) {
            echo 'Brukeren ' . $_POST['usr'] . ' ble opprettet';
            echo '<br>Gå til <a href="login.php">innlogging</a> for å logge inn"';
        }
        else {
            $cred->message();
        }
    }
}
else {
    header('location: login.php');
}
