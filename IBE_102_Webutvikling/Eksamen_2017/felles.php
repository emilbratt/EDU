<?php

class Session {
    public static function check_inactivity() {
        if(isset($_SESSION['timestamp'])) {
            if(time() - 3600 < ($_SESSION['timestamp'])) {
                return $_SESSION['timestamp'] = time();
            }
        }
        header('Location: loggut.php');
    }

    public static function check_logged_in() {
        if(isset($_SESSION['verified']) == false) {
            Session::end();
        }
    }

    public static function end() {
        unset($_SESSION['user']);
        unset($_SESSION['verified']);
        unset($_SESSION['timestamp']);
    }
}

class Database {
    public static function connect() {
        $usr = 'user';
        $psw = 'passord';
        $host = '127.0.0.1';
        $port = '3306';
        $db = 'Vestbase';
        try {
    	$cnxn = new PDO(
                "mysql:host=$host;dbname=$db;" .
                "port=$port",$usr,$psw
            );
            $cnxn->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
        }
        catch(Exception $e)
        {
         	echo '<h2>';
            print_r($e->getMessage());
            echo '</h2>';
            die;
        }
        return $cnxn;
    }
}


class Credential {

    protected $cnxn;

    function __construct() {
        $this->cnxn = Database::connect();
    }

    public function verify($usr, $pwd) {
        $stmt = $this->cnxn->prepare('
            SELECT brukernavn, passord FROM brukere
            WHERE brukernavn = :u
        ');
        $stmt->bindParam(':u', $usr);
        $res = $stmt->execute();
        $res = $stmt->fetch(PDO::FETCH_ASSOC);

        if($res) {
            $pwd_hash = $res['passord'];
            if(password_verify($pwd, $pwd_hash)) {
                $_SESSION['user'] = $usr;
                $_SESSION['verified'] = true;
                $_SESSION['timestamp'] = time();
                return true;
            }
        }
        return false;
    }

    public function register($usr, $pwd, $pwd_) {

        if($pwd !== $pwd_) {
            echo 'Passord er ikke like';
            return false;
        }

        if(strlen($pwd) < 10) {
            echo 'Passord må være lengre enn 10 tegn';
            return false;
        }

        // CHECK IF USER EXISTS
        $stmt = $this->cnxn->prepare('
            SELECT brukernavn FROM brukere
            WHERE brukernavn = :u
        ');
        $stmt->bindParam(':u', $usr);
        $res = $stmt->execute();
        $res = $stmt->fetch(PDO::FETCH_ASSOC);

        if($res) {
            echo 'Brukeren eksisterer';
            return false;
        }
        $pwd_hash = password_hash($pwd, PASSWORD_ARGON2ID);

        $stmt = $this->cnxn->prepare('
        INSERT INTO brukere
            (brukernavn, passord)
        VALUES
            (:u, :p)
        ');
        $stmt->bindParam(':u', $usr);
        $stmt->bindParam(':p', $pwd_hash);
        $stmt->execute();
        echo 'Bruker ' . $usr . ' ble opprettet';
        return true;
    }

}

?>
