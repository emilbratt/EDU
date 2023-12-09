namespace BettingOdds;

class FotballBets
{
    private readonly string _bet;
    private readonly FotballMatch _fotballMatch;

    public FotballBets()
    {
        Console.WriteLine("Gyldig tips: ");
        Console.WriteLine(" - Enkeltuftall: H, U, B");
        Console.WriteLine(" - Halvgardering: HU, HB, UB");
        Console.WriteLine(" - Helgardering: HUB\n");

        Console.Write("Hva har du tippet for denne kampen? ");

        string bet = Console.ReadLine()?? "";
        if (bet == "") Environment.Exit(1);

        _bet = bet.ToUpper();
        _fotballMatch = new FotballMatch();
    }

    public void Run()
    {
        bool matchIsRunning = true;

        while (matchIsRunning)
        {
            matchIsRunning = _fotballMatch.GetUserCommand() switch
            {
                'H' => _fotballMatch.AddHomeGoal(),
                'B' => _fotballMatch.AddAwayGaol(),
                'X' => _fotballMatch.ConcludeMatch(),
                _ => true,
            };
        }
    }

    public void CheckResult()
    {
        string result = _bet.Contains(_fotballMatch.Result()) switch
        {
            true => "Du tippet riktig",
            false => "Du tippet feil",
        };

        Console.WriteLine(result);
    }
}
