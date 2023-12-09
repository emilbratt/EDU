namespace BettingOdds;

class FotballMatch
{
    private int _homeGoals;
    private int _awayGoals;

    public FotballMatch()
    {
        _homeGoals = 0;
        _awayGoals = 0;

        Console.WriteLine("\nKampen er i gang!\n");
    }

    private void ReportStatus()
    {
        Console.WriteLine($"Stillingen er {_homeGoals}-{_awayGoals}.");
    }

    public char GetUserCommand()
    {
        Console.WriteLine("Kommandoer: ");
        Console.WriteLine(" - H = scoring hjemmelag");
        Console.WriteLine(" - B = scoring bortelag");
        Console.WriteLine(" - X = kampen er ferdig\n");

        Console.Write("Angi kommando: ");

        string input = Console.ReadLine()?? "";
        while (input == "")
        {
            Console.Write("Angi kommando: ");
            input = Console.ReadLine()?? "";
        }

        char command = input.ToUpper().ToCharArray()[0];

        return command switch
        {
            'H' or 'B' or 'X' => command,
            _ => 'C',
        };
    }

    public bool AddHomeGoal()
    {
        _homeGoals++;
        Console.WriteLine("Hjemmelag scorer!");
        ReportStatus();
        return true;
    }

    public bool AddAwayGaol()
    {
        _awayGoals++;
        Console.WriteLine("Bortelag scorer!");
        ReportStatus();
        return true;
    }

    public bool ConcludeMatch()
    {
        Console.WriteLine("Kampen er over!");
        ReportStatus();
        return false;
    }

    public char Result()
    {
        return
            (_homeGoals > _awayGoals) ? 'H'
            : (_homeGoals < _awayGoals) ? 'B'
            : 'U';
    }
}
