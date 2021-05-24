<?php

echo '
<h1>Logg inn</h1>
<form action="sjekklogin.php" method="post">
<input type="hidden" value="true" name="verify">
<input type="text" name="usr" required>
<input type="password" name="pwd" required>
<input type="submit" value="Loggin">
</form>

<h1>Registrer ny bruker</h1>
<form action="nybruker.php" method="post">
<input type="hidden" value="true" name="register">
<input type="text" name="usr" required>
<input type="password" name="pwd" required>
<input type="password" name="pwd_" required>
<input type="submit" value="Loggin">
</form>
';
 ?>
