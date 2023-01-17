
<?php
    include "data.inc.php"; // denne gir oss tilgang til matrisen $v2
?>

<?php
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
            <input type="submit" name="knapp" value="Velg En">
        </form>
    ';
?>

<br>

<?php
    echo '
        <p>Flervalgs sjekkbokser</p>
        <form action="" method="get">
        ';


    foreach($v2 as $key => $array) {
        echo '
            <input type="checkbox" id='.$key.' name='.$key.' value='.$array['navn'].'>
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
?>
