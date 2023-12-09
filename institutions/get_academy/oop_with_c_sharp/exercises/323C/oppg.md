Vi skal lage Tre på rad

Lag en klasse
    GameConsole
med en metode
    Show()
som viser det hardkodede brettet under.
  a b c
 ┌─────┐
1│O    │
2│    O│
3│X X  │
 └─────┘


Kall så denne fra
    Main()
i Program.cs.


Lag en klasse
    Square
Den skal inneholde informasjon om en enkelt rute som vi kaller
klassen lagrer tilstand for en rute
hvor tilstand 1 er  tom -> " "
og tilstand 2 er tatt -> "X" eller "O" for henholdsvis spiller 1 og 2

Square skal ha 3 metoder
    1 en bool metode for å sjekke om den er tom

    2 en bool metode for å sjekke om player 1 har tatt den.

    3 en metode for å sette "X" eller "O" for henholdsvis spiller 1 og 2 hvis ruten er ledig
      den skal ta en bool som parameter - true (spiller 1) og false (spiller 2).

Lag så en klasse
    BoardModel
med en array som inneholder ni objekter av Square.

Endre så metoden Show i GameConsole, slik at den tar et objekt av BoardModel som parameter og viser frem innholdet dynamisk.
Altså samme brett som før, men "X" der riktig Square i BoardModel sier player 1 og "O" der den sier player 2.

Koden i Main skal være slik
```c#
var boardModel = new BoardModel();
var gameConsole = new GameConsole(boardModel);
while (true)
{
    gameConsole.Show(boardModel);
    Console.Write("Skriv inn hvor du vil sette kryss (f.eks. \"a2\"): ");
    var position = Console.ReadLine();
    boardModel.Mark(position);
}
```
hvor variabelen
    position
innehar kolonne og siffer for å sette kryss der man vil.


Legg til koden under i slutten av while-løkken.
lag metoden
    MarkRandom()
i
    BoardModel
slik at den setter en tilfeldig valgt rute til player 1 eller 2 (ut fra parameteren).

```c#
Thread.Sleep(2000);
boardModel.MarkRandom(false);
```

For å få til det lager du en propert
    private readonly Random _random = new Random();
i klassen
    BoardModel.
Med den kan du få et tilfeldig tall som er min. f.eks. 5 og maks. 15 slik:
    var randomNumber = _random.Next(5, 15);

Mål:
Gjør ferdig det som mangler.
Programmet skal kun tillate å sette kryss i en tom rute.
Programmet skal oppdage om noen har vunnet.
Programmet skal la deg starte spillet på nytt igjen.

