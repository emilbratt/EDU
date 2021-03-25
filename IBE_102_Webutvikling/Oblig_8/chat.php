<?php
    require_once "felles.php";
    $Main = new Main();
    $Main->include('header');

    $Chat = new Chat($_SESSION['user']);


    $Chat->form();
    $Chat->show();

    if(!empty($_POST['chatmsg'])) {
        $Chat->append($_POST['chatmsg']);
    }


    $Main->include('footer');
?>
