Koden under lar brukeren holde oversikt over alle 12 kampene på en tippekupong.
Den bruker klassen FotballMatch fra forrige oppgave:


```c#
Console.Write("Gyldig tips: \n" +
    " - H, U, B\n" + 
    " - halvgardering: HU, HB, UB\n" + 
    " - helgardering: HUB\n" + 
    "Hva har du tippet for denne kampen? ");
Console.Write($"Skriv inn {antall} tippinger med komma mellom hver (en tipping for hver kamp): ");
var betsText = Console.ReadLine();
var bets = betsText.Split(',');
var matches = new FotballMatch[12];
for (var i = 0; i < 12; i++)
{
    matches[i] = new FotballMatch(bets[i]);
}

while (true)
{
    Console.Write("Skriv kampnr. 1-12 for scoring eller X for alle kampene er ferdige\r\nAngi kommando: ");
    var command = Console.ReadLine();
    if (command == "X") break;
    var matchNo = Convert.ToInt32(command);
    Console.Write("Kommandoer: \n" + 
        " - H = scoring hjemmelag\n" + 
        " - B = scoring bortelag\n" + 
        " - X = kampen er ferdig\n" + 
        "Angi kommando: ");
    var team = Console.ReadLine();
    var selectedIndex = matchNo - 1;
    var selectedMatch = matches[selectedIndex];
    selectedMatch.AddGoal(team == "H");
    var correctCount = 0;
    for (var index = 0; index < matches.Length; index++)
    {
        var match = matches[index];
        var mathNo = index + 1;
        var isBetCorrect = match.IsBetCorrect();
        var isBetCorrectText = isBetCorrect ? "riktig" : "feil";
        if (isBetCorrect) correctCount++;
        Console.WriteLine($"Kamp {matchNo}: {match.GetScore()} - {isBetCorrectText}");
    }

    Console.WriteLine($"Du har {correctCount} rette.");
}
```

Din oppgave er å innføre en ny klasse for en tippekupong.
Den kan f.eks. hete TwelveMatches.
Om du trenger hint, se på hvordan kode ble flyttet ut til en ny klasse i forrige oppgave.
Et annet hint: gå inn i løsningsforslaget og kun se på koden i Program.cs.

Løsningsforslag: github.com/GetAcademy/FootballBetting12
