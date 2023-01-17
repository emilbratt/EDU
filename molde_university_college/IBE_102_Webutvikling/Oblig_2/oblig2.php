<!--
    Student-ID: ******
    Studentnavn: Emil Bratt Børsting
-->
<!DOCTYPE html>
<html lang="en">

<head>
<meta name="author" content="Emil Bratt Børsting">
<meta charset="utf-8">
<title>Oblig 2</title>

<?php // oblig 2: Ingen gruppe
    // SET COLOUR FOR TABLES
    $even = array(
        "#ffeceb",
        "#ebe4e4",
        "#fcf1f0"
    );

    $odd = array(
        "#f5f7df",
        "#e4e6cf",
        "#e7e8da"
    );

    // ASSIGN COLOUR ARRAY DEPENDING ON TIMESTAMP IN SECONDS (ODD OR EVEN)
    if ((date("s") % 2) == 1) {
        $colours = $odd;
    }
    else {
        $colours = $even;
    }
?>

<!--CSS-->
<?php
    echo "
    <style>
        table {
            margin-left: 10px;

            font-family: arial;
            border-collapse: collapse; /*no boarder between each cell*/
            width: 80%; /* take up whole horizontal space in browser */
        }

        caption,td, th {
            border: 3px solid $colours[0];
            padding: 8px;
            text-align: center;
        }
            caption, tr:nth-child(even) {
            background-color: $colours[1];
        }
            tr:nth-child(odd) {
            background-color: $colours[2];
        }
    </style>
    ";
?>
</head>

<body>
<?php // TITTEL MED FARGE HVIS S OVER 30
    if (date('s') > 30) {
        echo '<h1 style="color:green">Innlevering i IBE102</h1>';
    }
    else {
        echo '<h1>Innlevering i IBE102</h1>';
    }
?>

<?php // TALLREKKE
    echo '<h2>Tallrekke</h2>';
    echo '<p style="font-size:30px">20';

    for ($i=21; $i<50; $i++) {
        if (($i%2)==1) {
            echo "\n<span style='color:red'>$i</span>\n";
        }
        else {
            echo "$i\n";
        }
    }
    echo '</p>';
?>

<?php // GANGETABELL
    echo "
        <h2>Gangetabell</h2>
    ";

    echo "
    <table border='1'>
        <caption>
            ...ved bruk av dobbel do while
        </caption>
        <tr>";

    // Kolonnenavn
    for ($i=1; $i<10; $i+=1) {
        $base = $i;
        echo "
            <th>
                $base-Gangen
            </th>";
    }
    echo "
        </tr>";

    $row = 1;
    do {
        echo "
            <tr>";
        $val = 1;
        do {
            if ($val == $row) {
                echo "
                <td style= 'font-weight:bold;'>
                    ".$val*$row."
                </td>";
            }
            else {
                echo "
                <td>
                    ".$val*$row."
                </td>";
            }
        }
        while (9 > $val++);

        echo "
            </tr>";
    }
    while (9 > $row++);
    echo "
    </table>";

    echo "<br>";

    echo "
<table>
    <caption>
        ...ved bruk av dobbel for loop
    </caption>
        <tr>";

    // Kolonnenavn
    for ($i=1; $i<10; $i+=1) {
        $base = $i;
        echo "
            <th>
                $base-Gangen
            </th>";
    }
    echo "
        </tr>";

    // generer hver rad
    for ($i=1; $i<10; $i++) {
        $base = $i;
        echo "
        <tr>";

        // generer verdier
        for ($n=1; $n<10; $n++) {
            if ($n == $i) {
                echo "
                <td style= 'font-weight:bold;'>"
                    .$base*$n.
                "</td>";
            }
            else {
            echo "
            <td>"
                .$base*$n.
            "</td>";
            }
        }

        echo "
        </tr>
        ";
    }
    echo "
</table>";
?>

<br>

<?php // SIMULERE TERNINGKAST

    // legge terningens verdier i en array
    $diceValues = array();
    for ($i=1; $i<7; $i++) {
        $diceValues[$i] = 0;
    }

    // generere antall kast av hver verdi ved bruk av switch case
    // vi simulerer kunn 100 kast her
    $totalRolls = 100;
    $min = 1;
    $max = 6;
    for ($i=1; $i < $totalRolls; $i++) {
        $res = rand($min,$max);
        switch($res) {
            case 1:
                $diceValues[1] = $diceValues[1] + 1;
                    break;
            case 2:
                $diceValues[2] = $diceValues[2] + 1;
                    break;
            case 3:
                $diceValues[3] = $diceValues[3] + 1;
                    break;
            case 4:
                $diceValues[4] = $diceValues[4] + 1;
                    break;
            case 5:
                $diceValues[5] = $diceValues[5] + 1;
                    break;
            case 6:
                $diceValues[6] = $diceValues[6] + 1;
                    break;
            default:
                echo "Håper for guds skyld dette ikke blir skrevet ut..";
            }
    }

    echo "<br><h2>Terningkast ved bruk av switch og case (100 kast)</h2>\n";
    echo "Terningkast: 1 ble kastet $diceValues[1] ganger<br>\n";
    echo "<br>Terningkast: 2 ble kastet $diceValues[2] ganger<br>\n";
    echo "<br>Terningkast: 3 ble kastet $diceValues[3] ganger<br>\n";
    echo "<br>Terningkast: 4 ble kastet $diceValues[4] ganger<br>\n";
    echo "<br>Terningkast: 5 ble kastet $diceValues[5] ganger<br>\n";
    echo "<br>Terningkast: 6 ble kastet $diceValues[6] ganger<br>\n";

    // legge nye terningens verdier i en array
    unset($diceValues);

    $diceValues = array();
    for ($i=1; $i<7; $i++) {
        $diceValues[$i] = 0;
    }

    // generere antall kast av hver verdi uten bruk av switch case
    // her simulerer vi 1 million kast
    $totalRolls = 1000000;
    $min = 1;
    $max = 6;
    for ($i=1; $i < $totalRolls; $i++) {
        $res = rand($min,$max);
        $diceValues[$res] = $diceValues[$res] + 1;
    }

    echo "<br><h2>Terningkast uten bruk av switch og case (1 million kast)</h2>\n";
    echo "Terningkast: 1 ble kastet $diceValues[1] ganger<br>\n";
    echo "<br>Terningkast: 2 ble kastet $diceValues[2] ganger<br>\n";
    echo "<br>Terningkast: 3 ble kastet $diceValues[3] ganger<br>\n";
    echo "<br>Terningkast: 4 ble kastet $diceValues[4] ganger<br>\n";
    echo "<br>Terningkast: 5 ble kastet $diceValues[5] ganger<br>\n";
    echo "<br>Terningkast: 6 ble kastet $diceValues[6] ganger<br>\n";



    // sette 30 width på terningkast-tabell da denne er mye slankere
    echo "
    <style>
        .terningkast  {
            width: 30%; /* take up whole horizontal space in browser */
        }

    </style>
    ";

    // skrive ut verdiene i en tabell
    echo "
    <br>
    <table class='terningkast' width=150 border=1>
        <caption>
            Antall ganger hver verdi ble kastet fra en million kast
        </caption>
        <tr><th>Utfall</th>	<th>Antall</th></tr>
    ";
    for ($i=1; $i < count($diceValues); $i++) {
        echo "
        <tr>
            <td>
                $i
            </td>
            <td>
                $diceValues[$i]
            </td>
        </tr>
        ";
    }
    echo "
    </table>
    ";
?>

</body>

</html>
