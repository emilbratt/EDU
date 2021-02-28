<?php
include_once 'felles.php';
$libMain = new Main();

echo <<<EOT

    <HR>
    <footer>
        <p>
            <i>Om nettstedet</i>: $libMain->PROGNAVN<br>
            Ansvarlig: Meg Selv. Tlf. 123123.
        </p>
EOT;
$libMain->showHelpButton();
echo "
    </footer>
</body>
</html>
";

?>
