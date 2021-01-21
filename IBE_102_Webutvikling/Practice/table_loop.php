<!DOCTYPE html>
<!--
Taking another shot at tables.
Dont know if focusing on neat html formating
is the smart option or if I should
mainly focus on formating the php code

I opted for a nice html source-code this time

Guess I gotta see how I adapt as I move forward
with this new language
-->
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Table Example With While Loop</title>
</head>




<body>
<?php
    // SET COLOUR FOR TABLES
    $even = array(
        "#F2F0EB",
        "#DEDBD5",
        "#C7C2B5"
    );

    $odd = array(
        "#CFE3E5",
        "#9BC1D1",
        "#7BA3B4"
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
          font-family: arial;
          border-collapse: collapse; /*no boarder between each cell*/
          width: 100%; /* take up whole horizontal space in browser */
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

<!--PHP CREATE STANDARD TABLE WITH A CAPTION-->
<?php
    $iter = 11;
    $valA = 0;
    $valB = 100;
    $valC = -100;

    // CAPTION + TABLE-HEADERS
    echo "
<table>
    <caption>Mine Verdier</caption>
    <tr>
        <th>Verdi A</th>
        <th>Verdi B</th>
        <th>Verdi C</th>
    </tr>";

    // ADDING 3 CONSECUTIVE VALUES 1 ROW AT A TIME
    while ($iter != 0) {
        echo "
    <tr>
        <td>".$valA."</td>
        <td>".$valB."</td>
        <td>".$valC."</td>
    </tr>";

        // AND CHANGING VALUES FOR EACH ITERATION
        $iter = $iter -1;
        $valA = $valA + 10;
        $valB = ($valB - 10);
        $valC = ($valC + 20);
    }

    echo "
</table>";

?>

<br>

<!--CREATE TABLE WHERE AS ONE COLUMN HAS MORE ROWS-->
<?php
    $iter = 5; // FOR TABLE WITH MULTIPLE ROWS, SET COUNT DOWN

    // NOTICE THE USE OF <th rowspan=N>
    // ALL VALUES INSERTED AFTER rowspan WILL
    // BE INSERTED INTO THAT ONE COLUMNG AS
    // INVIVIDUAL ROWS
    echo"
<table>
    <caption>Kolonne med flere verdier</caption>
    <tr>
        <th>En verdi</th>
        <th>Flere verdier</th>
    </tr>
    <tr>
        <th rowspan=$iter>
            Tekst under 'En verdi'
        </th>
            <td>
                Første verdi: $iter
            </td>
    ";

    // POPULATING THE INDIVIDUAL COLUMNG UNDER "Flere verdier"
    // WITH MULTIPLE ROWS
    while ($iter > 1) {
            $iter = ($iter -1);

        // WILL RUN UNTIL THE LAST ITERATION
        if ($iter != 1) {
        echo "
            <tr>
                <td>Påfølgende verdi: ".$iter."</td>
            </tr>";
        }

        // WILL RUN ON THE LAST ITERATION
        else {
            echo "
            <tr>
                <td>Siste verdi: ".$iter."</td>
            </tr>";
        }
    }
    echo "
</table>";
?>

<br>

<!--CREATE A MULTIPLICATION TABLE-->
<?php
    echo "
<table>
    <caption>
        Gangetabellen
    </caption>
        <tr>";

    // CREATE THE COLUMN HEADERS
    for ($i=1; $i<11; $i+=1) {
        $base = $i;
        echo "
            <th>
                $base-Gangen
            </th>";
    }
    echo "
        </tr>";

    // CREATE EACH ROW
    for ($i=1; $i<11; $i++) {
        $base = $i;
        echo "
        <tr>";

        // CREATE VALUES FOR EACH COLUMN IN THAT ROW
        for ($n=1; $n<11; $n++) {
            echo "
            <td>"
                .$base*$n.
            "</td>";
        }

        echo "
        </tr>
        ";
    }
    echo "
</table>";
?>


</body>
</html>
