<?php
require 'lib.php';
echo HTML::start('oppgave 2');

// SLETT COOKIE HVIS ELDRE ENN EN TIME
if(isset($_COOKIE['timestamp'])) {
    if($_COOKIE['timestamp'] < time()-3600) {
        unset($_COOKIE['timestamp']);
        unset($_COOKIE['text_store']);
    }
}

// SLETT COOKIE HVIS AVHUKET FOR SLETT TEKST
if(isset($_GET['text_del'])) {
    unset($_COOKIE['timestamp']);
    unset($_COOKIE['text_store']);
}

// HENT INN VERDIER FRA GET OG COOKIE FOR Å VISUALISERE I TABELL
if(isset($_GET['user_reg'])) {
    if(isset($_GET['text_store'])) {
        $time_stamp = null;
        if(isset($_COOKIE['timestamp'])) {
            $time_stamp = $_COOKIE['timestamp'];
        }
        $old = null;
        $new = null;

        if(!(isset($_COOKIE['text_store']))) {
            setcookie('text_store', $new, time()+3600);
            // $old = $_GET['text_store'];
        }
        else {
            $old = $_COOKIE['text_store'];
        }

        $new = $_GET['text_store'];
        setcookie('text_store', $new, time()+3600);
        setcookie('timestamp', time(), time()+3600);

    }
    table($time_stamp, $old, $new);
    setcookie('text_store', $old.$new, time()+3600);
}


function form() {
    echo '
    <form action="" id="text_form" method="get">
    <input type="hidden" name="user_reg" value="true">
    <label for="tekst_get">Skriv inn tekst:</label>
    <input type="text" name="text_store" id="tekst_get">

    <br>
    <label for="text_del">Slett tekst:</label>
    <input type="checkbox" name="text_del" value="true">
    <br>

    <input type="submit" value="Utfoer">
    </form>
    ';
}




form();


echo HTML::end();
