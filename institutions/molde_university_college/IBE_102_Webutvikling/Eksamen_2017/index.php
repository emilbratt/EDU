<?php
echo '<h1 style="text-align: center; background-color: #BBDDDD;
color: #6AAC98">Eksamen 2017</h1>';
$dir    = $_SERVER['DOCUMENT_ROOT'].'/';
$scripts = scandir($dir);

foreach($scripts as $script) {
    if(is_file($script) and $script != 'index.php') {
        echo '<h1 style="text-align: center;"><a style="background-color: #AADDDD; text-decoration: none;
        color: #1AAC38;" href="'.$script.'">'. $script . '</a></h1>';
    }

}


?>
