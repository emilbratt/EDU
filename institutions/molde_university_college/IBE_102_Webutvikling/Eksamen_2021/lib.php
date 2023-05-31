<?php
// EN ENKEL MÅTE Å FORMATERE HTML START OG SLUTT FOR ALLE SCRIPT
class HTML {
    public static function start($title = 'page') {
        return <<<EOT
        <!DOCTYPE html>
        <html lang="no">
        <head>
            <meta charset="utf-8">
            <link rel="stylesheet" href="/style.css">
            <title>$title</title>
        </head>
        <body>
        EOT;
    }

    static public function end() {
        return <<<EOT
        </body>
        </html>
        EOT;
    }
}

// VIS TABELL FOR OPPGAVE 2
function table($time_stamp = null, $old_val = null, $new_val = null) {
     $last_edit = 'Sist endret: ';

    if($time_stamp == null) {
        $last_edit .= 'Ikke vært endret før';
    }
    $last_edit .= $time_stamp;

    $last_edit = htmlentities($last_edit);
    $old_val = htmlentities($old_val);
    $new_val = htmlentities($new_val);

    echo '
    <table>
    <tr>
        <td>Forrige tekst:<br>' . $last_edit .'</td>
        <td>' . $old_val . '</td>
    </tr>
    <tr>
        <td>+Tekst som skal legges til:</td>
        <td>' . $new_val . '</td>
    </tr>
        <td>=Ny tekst:</td>
        <td>' . $old_val.$new_val . '</td>

    </table>
    ';
}


// KLASSE FOR DATABASETILKOBLING FOR OPPG 3
class Database {
    protected $cnxn;
    private $usr = 'user';
    private $psw = 'password';
    private $host = '127.0.0.1';
    private $port = '3306';
    private $db = 'exam2021';
    protected $ivalid_characters = [ // FORHINDRE INNFØRING VED HJELP AV REGEX
        'Ulovlige tegn for addresser' => '/^[A-C]{1}(Kj|Loft|[1-3])-[0-9]{1,3}-[0-8]$/',
    ];

    function __construct() {
        $usr = $this->usr;
        $psw = $this->psw;
        $host = $this->host;
        $port = $this->port;
        $db = $this->db;
        try {
    	$this->cnxn = new PDO("mysql:host=$host;dbname=$db;" .
                "port=$port",$usr,$psw
            );
            $this->cnxn->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
        }
        catch(Exception $e)
        {
         	echo '<h2>';
            print_r($e->getMessage());
            echo '</h2>';
            die;
        }

    }

    // LEGG INN ADDRESSE FOR OPPG 3
    public function insert($val) {

        // REGEX ER EN MERE KOSTBAR PROSESS, VI FILTRERE UT MED Å SJEKKE LENGDE FØRST
        if(strlen($val) > 15) {
            return 'Adressen ['.$val.'] har for mange karakterer';
        }

        // NÅ SJEKKER VI REGULÆRT UTTRYKK
        if(preg_match($this->ivalid_characters['Ulovlige tegn for addresser'], $val) == false) {
            return 'Adressen ['.$val.'] inneholder ugyldige tegn';
        }

        // STRENG TATT IKKE NØDVENDIG, MEN VI KAN BESKYTTE OSS MOT SCRIPT-ANGREP LIKEVEL
        $val = htmlentities($val);

        $stmt = $this->cnxn->prepare('
            SELECT key_id, adr FROM address
            WHERE adr = :v
        ');
        $stmt->bindParam(':v', $val);
        $stmt->execute();
        $res = $stmt->fetch(PDO::FETCH_ASSOC);
        if($res) {
            return 'Feil i execute. Duplicate entry ' .  $val .
             ' for key ' . strval($res['key_id']);
        }
        $stmt = $this->cnxn->prepare('
            INSERT INTO address
                (adr)
            VALUES
                (:v)
        ');
        $stmt->bindParam(':v', $val);
        // $stmt->execute();
        if($stmt) {
            return $val . ' Har blitt ført opp i databasen';
        }
        return 'En feil skjedde og ' . $val . ' ble ikke lagt inn, kontakt system-administrator';
    }

}
