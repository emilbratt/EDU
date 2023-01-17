<?php
    // her genererer vi matrise som skal bruke i skjema for visvalute.php
    // Man legge på en ekstra oppføring i hver av disse matrisene
    // hvis man ønsker flere valg i skjemaet (NB: hver matrise må være like lang)
    $tickers = array(
        "BTC","ETH","LTC","XMR","XRP"
    );
    $navn = array(
        "Bitcoin","Ethereum","Litecoin","Monero","Ripple"
    );
    $k1 = array(
        1,2,3,4,5
    );
    $k2 = array(
        11,12,13,14,15
    );
    $k3 = array(
        17,27,37,47,57
    );

?>
<?php
    // denne tar verdiene fra matrisene over og genererer en matrise for alle verdiene som deretter
    // vil bli behandlet i visvaluta.php
    $v2 = array();
    foreach($tickers as $index => $tick) {
        $v2[$tick] = array();
        $v2[$tick]["navn"] = $navn[$index];
        $v2[$tick]["k1"] = $k1[$index];
        $v2[$tick]["k2"] = $k2[$index];
        $v2[$tick]["k3"] = $k3[$index];
    }
?>
