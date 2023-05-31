<?php
require_once 'felles.php';
$libMain = new Main();

echo '
    <HR>
    <footer>
        <p>
            <i>Om nettstedet</i>: '.$libMain::PROGNAVN.'<br>
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
