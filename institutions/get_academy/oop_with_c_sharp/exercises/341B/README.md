Koden under er et enkelt klikkerspill:

```c#
int points = 0;
int pointsPerClick = 1;
int pointsPerClickIncrease = 1;

while (true)
{
    Console.Clear();
    Console.WriteLine("Kommandoer:\r\n - SPACE = klikk (og få poeng)\r\n"
    + " - K = kjøp oppgradering \r\n       øker poeng per klikk \r\n       " 
    + "koster 10 poeng\r\n - S = kjøp superoppgradering \r\n       "
    + "øker \"poeng per klikk\" for den vanlige oppgraderingen.\r\n       "
    + "koster 100 poeng\r\n - X = avslutt applikasjonen");
    Console.WriteLine($"Du har {points} poeng.");
    Console.WriteLine("Trykk tast for ønsket kommando.");
    var command = Console.ReadKey().KeyChar;
    if (command == 'X') Environment.Exit(0);
    else if (command == ' ') points += pointsPerClick;
    else if (command == 'K' && points >= 10)
    {
        points -= 10;
        pointsPerClick += pointsPerClickIncrease;
    }
    else if (command == 'S' && points >= 100)
    {
        points -= 100;
        pointsPerClickIncrease++;
    }
}
```

Skriv om til mer objektorientert.
 - Innfør en klasse "ClickerGame" med objektvariabler for de tre int-variablene med metode for hver kommando.
 - Innfør interfacen "ICommand" med metoden "Run".
 - Lag en klasse for hver kommando i applikasjonen.
 - Lag klassen "CommandSet". Denne har en array med et objekt av hver kommando.

Når brukeren taster et tegn så skal dette sendes til metoden RunCommand i CommandSet.
Der slår den opp hvilken kommando som har den aktuelle bokstaven og kjører så Run på denne kommandoen.

For hint, se Program.cs i løsningsforslaget: github.com/GetAcademy/Clicker
