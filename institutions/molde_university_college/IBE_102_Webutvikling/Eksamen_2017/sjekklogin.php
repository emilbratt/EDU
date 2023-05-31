<?php
session_start();
require_once 'felles.php';
if(isset($_POST['verify'])) {
    if(isset($_POST['usr']) and isset($_POST['pwd']) ) {
        $cred = new Credential();
        if($cred->verify($_POST['usr'],$_POST['pwd'])) {
            echo '<h3>Klikk <a href="meny.php">her</a> for å gå videre</h3>';
        }
        else {
            echo 'Feil brukernavn eller passord';
            echo '<h3>Klikk <a href="login.php">her</a> for å prøve igjen videre</h3>';
        }
    }
}


 ?>
