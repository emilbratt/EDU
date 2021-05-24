<?php
session_start();
require_once 'felles.php';
if(isset($_POST['register'])) {
    if(isset($_POST['usr']) and isset($_POST['pwd']) and isset($_POST['pwd_']) ) {
        $cred = new Credential;
        if($cred->register($_POST['usr'], $_POST['pwd'], $_POST['pwd_'])) {
            die('start');
        }
        die('stopp');
    }
}
header('location: login.php');

?>
