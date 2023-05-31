<?php
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

<?php
require_once 'felles.php';
$libMain = new Main();
include $libMain->getFilename('news');

echo "
<!DOCTYPE html>
<html>
<head>
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
    <title>".$libMain::PROGNAVN."</title>
</head>

<body>
    <table>
      <tr>
        <td >Velkommen til ".$libMain::PROGNAVN."</td>
        <td>";
        echo "
        <ul>";
        foreach($newslist as $title => $link) {
                echo '
                <li><a href="'.$link.'">'.$title.'</a></li>';
        }
        echo "
        </ul>";
        echo "
        Sist oppdatert: ".$libMain->getTimestamp('news')."</td>

      </tr>

    </table>
    <HR>
";
?>
