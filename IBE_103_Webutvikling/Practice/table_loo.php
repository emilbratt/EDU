<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Table Example With While Loop</title>
</head>

<!--
Taking another shot at tables.
Dont know if focusing on neat html formating
is the smart option or if I should
mainly focus on formating the php code

I opted for a nice html source-code this time

Guess I gotta see how I adapt as I move forward
with this new language
-->

<html>
<body>

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

</body>
</html>
