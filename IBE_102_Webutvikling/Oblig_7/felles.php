<?php
    // classes: Main, Login

    class Main{

        const PROGNAVN = 'Kryptovaluta A/S';
        private $caller;
        private $files = [
            'oblig 2 variabler og løkker' => 'oblig2.php',
            'oblig 3 matriser' => 'oblig3.php',
            'oblig 4 vis valuta' => 'oblig4.php',
            'oblig 6 vis valuta med søk' => 'oblig6.php',
            'footer' => 'bunn.php',
            'header' => 'topp.php',
            'help' => 'hjelp.php',
            'currencies' => 'valutaer.php',
            'libraries' => 'felles.php',
            'news' => 'nyheter.php'
        ];

        public function __construct($arg = false) {
            @session_start();

            // for debugging we can see what script creates an object
            // pass the option true when instantiating to print out the filename
            $this->caller = $_SERVER['SCRIPT_NAME'];
            if($arg == 'debug') {
                echo "
                <pre>
                    this class is initiated by $this->caller
                </pre>
                ";
            }

            // make sure that every script that is not login.php is checked for login verification
            if($_SERVER['SCRIPT_NAME'] != "/oblig/7_innlogging/login.php") {

                // if login is not verified, load login.php
                if(!isset($_SESSION['verified'])) {
                    header("Location: login.php");
                    exit;
                }
            }

        }

        public function getCaller() {
            return $this->caller;
        }

        public function include($alias) {
            return include $this->files[$alias];
        }

        public function getFilename($key) {
            return $this->files[$key];
        }

        public function getTimestamp($alias) {
            $time = filemtime($this->getFilename($alias));
            $formatDate =  date("d/m-Y \k\l\. H:i\.",$time);
            return $formatDate;
        }

        public function generatePageLinks($re = 'oblig') {
            echo "
            <ul>";
            foreach($this->files as $alias => $page) {
                if(strpos($alias,$re) !== false) {
                    echo "
                    <li><a href=".$page.">$alias</a></li>";
                }
            }
            echo "
            </ul>";
        }

        public function showHelpButton() {
            echo '
            <form action='.$this->getFilename('help').'>
              <input type="submit" value="Trenger du hjelp?">
            </form>
            ';
        }

        function __destruct() {

        }

    }


    // this class is for the final assignment where we implement a login session for
    // our crypto-currency website
    class Login{
        private $accounts;
        public function __construct() {
            @session_start();
            $this->accounts = file_get_contents("kontoer.json");
        }

        public function verify($usrInput,$pwdInput) {
            $accounts = json_decode($this->accounts);

            foreach($accounts as $k => $v) {
                if(ucfirst($usrInput) == $k) {
                    // we use password_verify() because the passwords are hashed
                    if(password_verify($pwdInput,$v->password)) {
                        $_SESSION['verified'] = true;
                        $_SESSION['user'] = htmlentities($_POST['user']);
                        $_SESSION['timeStart'] = date("\k\l\. H:i\ \- d/m-Y",time());

                        return true;
                    }
                }
            }
            return false;
        }

        public function logout() {
                unset($_SESSION["verified"]);
                unset($_SESSION["user"]);
                unset($_SESSION["password"]);
                echo '
                    <p>Du er nå logget ut<br>Gå tilbake til innlogging</p>
                    <a href="login.php">Klikk her</a>
                ';

        }

        public function hashPassword($pwdInput) {
            // this function is for demonstration purpose only and shows
            // how I hashed the passwords for Per and Kari


            // I made an array containing the algorithms that the passsword_hash() function can handle
            // so that we can point to this array instead of hard-typing it
            $algo = array(
                    'default' => PASSWORD_DEFAULT,
                    'blowfish' => PASSWORD_BCRYPT,
                    'argon2i' => PASSWORD_ARGON2I,
                    'argon2id' => PASSWORD_ARGON2ID
                );

            // we chose default
            $pwdHashed = password_hash($pwdInput, $algo['default']); //hash passwords

            // verification is simply done using the password_verify() function which
            // is smart enough to know which algorithm was used when password was hashed
            return $pwdHashed;
        }

        function __destruct() {

        }
    }

?>
