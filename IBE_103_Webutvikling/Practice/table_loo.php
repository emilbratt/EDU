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

    <!--CSS-->
    <style>
        table {
          font-family: arial;
          border-collapse: collapse; /*no boarder between each cell*/
          width: 100%; /* take up whole horizontal space in browser */
        }

        caption,td, th {
          border: 3px solid #CFE3E5;
          padding: 8px;
          text-align: center;
        }
        caption, tr:nth-child(even) {
          background-color: #9BC1D1;
        }
        tr:nth-child(odd) {
          background-color: #7BA3B4;
        }
    </style>
</head>



<body>


<!--PHP CREATE TABLE-->
<?php
    $iter = 11;
    $valA = 0;
    $valB = 100;
    $valC = -100;
    echo "
<table>
    <caption>Mine Verdier</caption>
    <tr>
        <th>Verdi A</th>
        <th>Verdi B</th>
        <th>Verdi C</th>
    </tr>";

    while ($iter != 0) {
        echo "
    <tr>
        <td>".$valA."</td>
        <td>".$valB."</td>
        <td>".$valC."</td>
    </tr>";
        $iter = $iter -1;
        $valA = $valA + 10;
        $valB = ($valB - 10);
        $valC = ($valC + 20);
    }

    echo "
</table>";
?>

<br>

<?php
    $iter = 5; // FOR TABLE WITH MULTIPLE ROWS, SET COUNT
    echo"
<table>
    <caption>Kolonne med flere verdier</caption>
    <tr>
        <th>En verdi</th>
        <th>Flere verdier</th>
    </tr>
    <tr>
    <th rowspan=$iter>Nummer:</th>
    <td>$iter</td>
    ";
    while ($iter > 1) {
            $iter = ($iter -1);
        echo "
    <tr>
    <td>".$iter."</td>
    </tr>"; // VALUE THAT IS PRINTED

    }
    echo "
</table>";
?>




</body>
</html>
