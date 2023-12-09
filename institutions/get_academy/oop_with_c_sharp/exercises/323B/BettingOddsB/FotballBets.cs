namespace BettingOddsB;

class FotballBets
{
    private readonly FotballMatches _fotballMatches;

    public FotballBets(int betCount)
    {
        Console.WriteLine("Gyldig tips: ");
        Console.WriteLine(" - Enkeltuftall: H, U, B");
        Console.WriteLine(" - Halvgardering: HU, HB, UB");
        Console.WriteLine(" - Helgardering: HUB");
        Console.WriteLine();
        Console.Write($"Skriv inn {betCount} tippinger med komma mellom hver (en tipping for hver kamp): ");

        string userInput = Console.ReadLine()?? "";
        if (userInput == "") Environment.Exit(1);

        var bets = userInput.Split(',');
        if (bets.Length != betCount)
        {
            Console.WriteLine($"Feil: Du skreiv inn {bets.Length} tippinger, forventet {betCount} tippinger");
            Environment.Exit(1);
        }

        _fotballMatches = new FotballMatches(bets);
    }

    public void Run()
    {
        int totalMatches = _fotballMatches.MatchCount();

        bool matchesInProgress = _fotballMatches.StillInProgress();
        while (matchesInProgress)
        {
            Console.WriteLine($"Velg kamp (tall mellom 1-{totalMatches}) eller X for å avslutte alle kamper.");
            Console.Write("Angi kommando: ");

            string input = Console.ReadLine()?? String.Empty;
            if (input == String.Empty) continue;

            string command = input.ToUpper();

            if (command == "X") _fotballMatches.ConcludeAllMatches();

            bool isNumeric = int.TryParse(command, out int matchNumber);
            if (isNumeric)
            {
                int matchId = matchNumber - 1;
                Console.WriteLine($"Valgt kamp: {matchNumber}");
                string message = _fotballMatches.GetUserCommand(matchId) switch
                {
                    'H' => _fotballMatches.AddHomeGoal(matchId),
                    'B' => _fotballMatches.AddAwayGaol(matchId),
                    'X' => _fotballMatches.ConcludeMatch(matchId),
                    _ => "Ugyldig kommando..",
                };
                
                Console.WriteLine(message);
            }

            matchesInProgress = _fotballMatches.StillInProgress();
        }

        Console.WriteLine($"Fotballkampene er ferdig!!");
    }

    public void CheckResult()
    {
        Console.WriteLine("Oversikt");
        string stats = _fotballMatches.FinalReport();
        Console.WriteLine(stats);

        int correctBets = _fotballMatches.TotalCorrectBets();
        Console.WriteLine($"Antall rette: {correctBets}");
    }
}
