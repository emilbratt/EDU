<?php
    // this file holds the class Main -> important contents for the website
    // most of the files in the source-tree include this file

    class Main{

        public  $PROGNAVN = 'Kryptovaluta A/S';
        private $caller;
        private $files = [ // alias => filename
            'oblig 2 variabler og løkker' => 'oblig2.php',
            'oblig 3 matriser' => 'oblig3.php',
            'oblig 4 vis valuta' => 'oblig4.php',
            'footer' => 'bunn.php',
            'header' => 'topp.php',
            'help' => 'hjelp.php',
            'currencies' => 'valutaer.php',
            'lib' => 'felles.php',
            'news' => 'nyheter.php'
        ];

        public function __construct($arg = false) {
            // for debugging we can see what script creates an object
            // pass the option true when instantiating to print out the filename 
            $this->caller = $_SERVER['SCRIPT_NAME'];
            if($arg == true) {
                echo "
                <pre>
                    this class is initiated by $this->caller
                </pre>
                ";
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

?>
