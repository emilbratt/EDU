<?php
include_once 'felles.php';
$libMain = new Main();

echo '

    <HR>
    <footer>
        <p>
            <i>Om nettstedet</i>:'.Main::$PROGNAVN.'<br>
            Ansvarlig: Meg Selv. Tlf. 123123.
        </p>
    ';
$libMain->showHelpButton();
echo "
    </footer>
</body>
</html>
";

?>
