
<?php
    include_once 'felles.php';
    $lib = new Main();
    $lib->include('header');

    // denne gir oss tilgang til matrisen $v2
    include "data.inc.php";


    function searchData($str, $v2) {
        // funksjonen tar tekst-innput fra bruker og datasett som parameter
        // vi bruker stripos() istedet for strpos() slik at vi kan søke med
        // "case-insensitive" når vi leter etter ticker
        foreach($v2 as $currencies) {
            foreach($currencies as $key => $value) {
                if ($key == 'navn') {
                    if (stripos($value, $str) !== false) {
                        echo '
                            <h3>
                                Siste kjente kurs for '.$currencies['navn'].'
                                er $'.$currencies['k3'].'
                                <br>
                            </h3>
                                ';
                    }
                }
            }
        }
    }
?>

<?php // nedtrekk 1 valg
    echo '
        <p>Envalgsliste:</p>
        <form action="" method="get">
        <select name="envalg">
    ';
    $N = 1;
    foreach($v2 as $key => $array) {
        echo "
            <option value=$key>
                Valg nr. $N $key ".$array['navn']."
            </option>";
    $N++;
    }
    echo '
        </select>
            <input type="submit" name="knapp" value="Velg En"/>
        </form>
    ';
?>

<br>

<?php // sjekkboks flervalg
    echo '
        <p>Flervalgs sjekkbokser</p>
        <form action="" method="get">
        ';


    foreach($v2 as $key => $array) {
        echo '
            <input type="checkbox" id="'.$key.'" name="'.$key.'" value="'.$array['navn'].'">
            <label for='.$key.'>'.$key.' '.$array['navn'].'</label><br>
        ';
    }

    echo '
            <input type="reset" id="reset" value="X">
            <label for="reset"> Klikk her for å fjerne valg</label>
            <br>
            <input type="submit" id="submit" value="✓">
            <label for="submit"> Klikke her for å velge</label>
        </form>
    ';
?>



<?php // søkefelt
    echo '
        <hr>
        <form id="search" action="" method="GET">
            Søk etter valuta: <input type="text" name="let">
            <input type="submit" value="Søk">
        </form>
        <hr>
        <p><i>Om nettstedet</i>:  Ansvarlig:  Meg Selv.  Tlf. 123123.</p>
    ';
?>


<?php
    if(empty($_GET) == false) {
        if(isset($_GET['envalg']) == true) {
            $ticker = $_GET['envalg'];
            echo '
                <h3>
                    Siste kjente kurs for '.$v2[$ticker]['navn'].'
                    er $'.$v2[$ticker]['k3'].'
                    <br>
                </h3>
                ';
        }
        else if(isset($_GET['let']) == true) {
            searchData($_GET['let'], $v2);
        }
        else {
            echo '
                <h3>
                ';
            foreach($_GET as $ticker => $values) {
                echo '
                    Siste kjente kurs for '.$v2[$ticker]['navn'].'
                    er $'.$v2[$ticker]['k3'].'
                    <br>
                    ';
            }
            echo '
                </h3>
                ';
        }

    }

    $lib->include('footer');
?>
