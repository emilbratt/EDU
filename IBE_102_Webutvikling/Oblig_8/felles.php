<?php
    // classes: Main, Login, Chat

    class Main {

        const PROGNAVN = 'Kryptovaluta A/S';
        private $caller;
        private $files = [
            'oblig 2 variabler og løkker' => 'oblig2.php',
            'oblig 3 matriser' => 'oblig3.php',
            'oblig 4 vis valuta' => 'oblig4.php',
            'oblig 6 vis valuta med søk' => 'oblig6.php',
            'oblig 8 chat' => 'chat.php',
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
            if(basename($_SERVER['SCRIPT_NAME'], '.php') != "login") {

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
    class Login {
        protected $accounts;
        public $gender;
        function __construct() {
            @session_start();
            $this->accounts = file_get_contents("kontoer.json");
        }

        function verify($usrInput,$pwdInput) {
            $this->accounts = json_decode($this->accounts);

            foreach($this->accounts as $k => $v) {
                if(ucfirst($usrInput) == $k) {
                    // we use password_verify() because the passwords are hashed
                    if(password_verify($pwdInput,$v->password)) {
                        $_SESSION['verified'] = true;
                        $_SESSION['user'] = htmlentities($_POST['user']);
                        $_SESSION['timeStart'] = date("\k\l\. H:i\ \- d/m-Y",time());
                        $_SESSION['gender'] = $v->sex;
                        unset($_POST['password']);
                        unset($_POST['user']);
                        unset($pwdInput);
                        unset($usrInput);
                        unset($this->accounts);
                        return true;
                    }
                }
            }
            return false;
        }

        public function logout() {
                unset($_SESSION["verified"]);
                unset($_SESSION["user"]);
                unset($_SESSION["timeStart"]);
                unset($_SESSION['gender']);
                unset($_POST['btnLogin']);
                unset($_POST['ananas']);
                echo '
                    <p>Du er nå logget ut<br>Gå tilbake til innlogging</p>
                    <a href="login.php">Klikk her</a>
                ';

        }

        function hashPassword($pwdInput) {
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


    // this class handles the chat function described in the
    // voluntary assignment 8 where we use AJAX for chat reloading
    class Chat {
        private $chatPath = 'chat.txt'; // file path
        private $chatOpen; // for use when opening file
        private $user;


        public function __construct($user = false) {

            // the constructor will make sure to create text.txt if it does not exist
            // make sure to have your php server be owner or have write access webroot
            $this->user = $user;
            if($user != false) {
                if(!(file_exists($this->chatPath))) {
                    touch("chat.txt");
                    $skeleton = '<caption>Meldinger</caption><tr>';
                    $this->chatOpen = fopen($this->chatPath, 'a')  or die("Unable to open $chatPath");
                    fwrite($this->chatOpen, $skeleton."\n");
                    fclose($this->chatOpen);
                }
            }
            else {
                die ('killing script because there was an error loading your username');
            }
        }

        public function append($msg) {

            // convert to our characterset and sanitize input

            mb_convert_encoding($msg, "UTF-8", "ISO-8859-1");
            $msg = htmlentities($msg);

            // append html tags for nice presentation of chat
            $row = '
            <tr>
                <td>'.$msg.'</td>
                <td>'.$this->user.'</td>
                <td>'.date("\k\l\. H:i\ \- d/m-Y",time()).'</td>
            </tr>
            ';

            // remove new lines and make it a one line append
            $row = trim(preg_replace('/\s\s+/', ' ', $row));
            $this->chatOpen = fopen($this->chatPath, 'a')  or die("Unable to open $chatPath");
            fwrite($this->chatOpen, $row."\n");
            fclose($this->chatOpen);
        }

        public function form() {
            if($_SESSION['gender'] == "Mann") {
                $msg = 'Hva ønsker vår herremann ';
            }
            else {
                $msg = 'Hva ønsker vår kjære ';
            }
            echo '
            <div style="margin-left: 50px;">
                <form action="" method="post" style="text-aling: center;">
                    '.$msg.$_SESSION['user'].' å skrive i dag?
                    <input type="text"  autofocus="autofocus" onfocus="this.select()" name="chatmsg">
                    <input type="submit" name="chatbtn" value="Send inn!">
                </form>
            </div>
            ';
        }

        public function show() {

            // by using AJAX we can serve only the changes without reloading the whole page
            // once this part is loaded in your browser, it will self update every second
            echo <<<EOT
            <script>
                streamChat();
                setInterval (streamChat, 1000);
                function streamChat() {
                var xhttp = new XMLHttpRequest();
                    xhttp.onreadystatechange = function() {
                        if (this.readyState == 4 && this.status == 200) {
                        document.getElementById("chat_messages").innerHTML = this.responseText;
                        }
                    };
                xhttp.open("GET", "chat.txt", true);
                xhttp.send();
                }
            </script>
            <table border='1' id="chat_messages">
            </table>
            EOT;
        }

        function __destruct() {

        }
    }

?>
