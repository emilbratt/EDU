<!--
    Studentnavn: Emil Bratt Børsting
    Obligatorisk øvelse 5
    Kommentarer er gjort på engelsk
-->
<?php
    // this index file depends entirely on Main class found in felles.php
    include_once 'felles.php';
    $main = new Main();
    $main->include('header');
    $main->generatePageLinks();
    $main->include('footer');
?>
