<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="utf-8">
    <title>oblig3</title>
</head>

<body>

<?php





    // DEL 1 ///////////////////////////////

    echo "
    <H2>Del 1</H2>
    <p>Innhold i matrisen:</p>
    ";


    // TICKER
    $ticker = array(
        "BTC","ETH","LTC","XMR","XRP"
    );
    echo "
    <pre>ticker: ";
    print_r($ticker);
    echo "</pre>";


    // NAVN
    $navn = array(
        "Bitcoin","Ethereum","Litecoin","Monero","Ripple"
    );
    echo "<pre>navn: ";
    print_r($navn);
    echo "</pre>";


    // VERDIER
    $k1 = array(
        1,2,3,4,5
    );
    $k2 = array(
        11,12,13,14,15
    );
    $k3 = array(
        17,27,37,47,57
    );

    // VIS -> ARRAY INFO "HUMAN READABLE"
    echo"<pre>k1: ";
    print_r($k1);
    echo"</pre>";
    echo"<pre>k2: ";
    print_r($k2);
    echo"</pre>";
    echo"<pre>k3: ";
    print_r($k3);
    echo"</pre>";


    // VIS INNHOLD I MATRISENE
    echo "<p>Visning av innhold med while</p>";

    $i = 0;
    while ($i < 5) {
        echo "
    <p>
        Valutaen $navn[$i] ($ticker[$i])
        <IMG WIDTH=20 SRC='logoer/$ticker[$i].png'>
        har følgende siste verdier: $k1[$i], $k2[$i] og $k3[$i].
    </p>
    ";
    $i = $i +1;
    }





    // DEL 2 ///////////////////////////////

    echo "<h2>Del 2</h2>
    <p>Innhold i matrisen: </p>
    ";


    // NØSTE SAMMEN ARRAYS VED Å BRUKE EN MIDLERTIDIG ARRAY
    $merge = array(
        $navn,
        $k1,
        $k2,
        $k3,
    );

    $v = array(); // DETTE SKAL BLI NØSTET ARRAY

    // SETTE INN NØKLER
    foreach($ticker as $value) {
        $v[$value] = array();
    }

    // SETTE INN VERDIER FRA MIDLERTIDIG ARRAY
    foreach($merge as $array) {
        foreach($array as $index => $value) {
            array_push($v[$ticker[$index]], $value);
        }
    }


    echo "<pre>v: ";
    print_r($v);
    echo "</pre>\n";

    // VISNING MED FOREACH AV NØSTED MATRISE
    echo "
    <ul>
    ";
    foreach($v as $key => $array) {
        echo "
        <li>
            $key: "
        ;
        for ($i=0;$i<4; $i++) {
            if (is_int($array[$i]) == true) {
                if ($i == 3) {
                    echo "og $array[$i].";
                }
                else {
                    echo "$array[$i],";
                }
            }
            else {
                echo "($array[$i]): ";
            }
        }
        echo "
        </li>\n";
    }
    echo "
    </ul>
    ";





    // DEL 3 ///////////////////////////////

    echo "
    <h2>Del 3</h2>
    <p>Innhold i matrisen: </p>
    ";

    $v2 = array(); // DETTE SKAL BLI ET NØSTET ARRAY MED NØKLER

    // NØSTING MED NØKKEL + VERDIER HENTET FRA TIDLIGERE MATRISER
    foreach($ticker as $index => $tick) {
        $v2[$tick] = array();
        $v2[$tick]["navn"] = $navn[$index];
        $v2[$tick]["k1"] = $k1[$index];
        $v2[$tick]["k2"] = $k2[$index];
        $v2[$tick]["k3"] = $k3[$index];
    }


    echo "<pre>v2: ";
    print_r($v2);
    echo "</pre>\n";

    // VISNING MED FOREACH AV NØSTED MATRISE MED NØKLER
    echo "
    <p>Visning av matrisen med foreach: </p>
    ";
    foreach($v2 as $keyA => $array) {
        echo "
    <p>$keyA ";
        foreach($array as $keyB => $value )
        echo "$value ";
        echo"</p>";
    }


?>

</body>

</html>
