<!--
    Studentnavn: Emil Bratt Børsting
    Obligatorisk øvelse 7, hukommelse - session (innlogging og husking)
    Kommentarer er for det meste gjort på engelsk
-->
<?php
    function displayLogin() {
        $main = new Main('login');
        $main->include('header');

        echo '
        <form action = "'.$_SERVER['PHP_SELF'].'" method = "POST">
            <legend style="margin-left: 10px">
                Logg inn for å gå videre
            </legend>
            <table>
                <tr>
                    <td>
                        <h3>
                            Brukernavn:<br>
                            <input type="text" name="user" placeholder="Per" >

                        </h3>
                    </td>
                    <td>
                        <h3>
                            Passord:<br>
                            <input type="password" name="password" placeholder="1111">
                        </h3>
                    </td>
                    <td>
                        <h3>
                        <input type="checkbox" id="ananas" name="ananas" value="true">
                        <label for="ananas">Jeg liker ananas på pizza</label>
                        </h3>
                    </td>
                </tr>
                <tr >
                    <td colspan="3">

                        <input type="submit" name ="btnLogin" value="logg inn">
                    </td>
                <tr>
            </table>
        </form>
        ';

        $main->include('footer');
    }


    // script starts here
    require_once 'felles.php';
    $libLogin = new Login();

    if(isset($_POST['btnLogin'])) {
        if(isset($_POST['ananas'])) {
            if($libLogin->verify($_POST['user'],$_POST['password']) == true) {
                echo '
                    <h3>
                        Du ble logget inn som: '.$_SESSION['user'].'<br>
                        ..og du liker ananas på pizza<br>
                    </h3>
                    <p>Klikk <a href="index.php">her</a> for å gå videre</p>
                ';
            }
            else {
                // if no match for user or password, reload this page and the login form
                header("Location: {$_SERVER['PHP_SELF']}");
                exit;
            }

        }
        else {
            // no pineapple on pizza you say? We cant allow that :-)
            echo '
                <h3>
                    Du har ikke huket av for at du liker ananas på pizza<br>
                    <a href="login.php">Gå tilbake til innlogging</a>
                </h3>
            ';

        }

    }
    else {
        // if no value is registered in $_POST['btnLogin'], load login form
        displayLogin();
    }

?>
