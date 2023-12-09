namespace BettingOddsB;

class FotballMatches
{
    private readonly FotballMatch[] _fotballMatches;

    public FotballMatches(string[] bets)
    {
        _fotballMatches = new FotballMatch[bets.Length];
        for (int i = 0; i < bets.Length; i++)
        {
            _fotballMatches[i] = new FotballMatch(bets[i], i);
        }
    }

    public int MatchCount()
    {
        return _fotballMatches.Length;
    }
    public bool StillInProgress()
    {
        foreach (FotballMatch match in _fotballMatches)
        {
            if (!match.IsConcluded()) return true;
        }
        return false;
    }

    public void ConcludeAllMatches()
    {
        foreach (FotballMatch match in _fotballMatches)
        {
            match.ConcludeMatch();
        }
    }

    public char GetUserCommand(int matchId)
    {
        if (_fotballMatches[matchId].IsConcluded()) return 'X';

        Console.WriteLine("Kommandoer: ");
        Console.WriteLine(" - H = scoring hjemmelag");
        Console.WriteLine(" - B = scoring bortelag");
        Console.WriteLine(" - X = kampen er ferdig\n");

        Console.Write("Angi kommando: ");

        string input = Console.ReadLine()?? String.Empty;
        while (input == String.Empty)
        {
            Console.Write($"Angi kommando: ");
            input = Console.ReadLine()?? String.Empty;
        }

        char command = input.ToUpper().ToCharArray()[0];

        return command switch
        {
            'H' or 'B' or 'X' => command,
            _ => 'C',
        };
    }

    public string AddHomeGoal(int matchId)
    {
        return _fotballMatches[matchId].AddHomeGoal();
    }

    public string AddAwayGaol(int matchId)
    {
        return _fotballMatches[matchId].AddAwayGaol();
    }

    public string ConcludeMatch(int matchId)
    {
        return _fotballMatches[matchId].ConcludeMatch();
    }

    public int TotalCorrectBets()
    {
        int correct = 0;
        for (int matchId = 0; matchId < _fotballMatches.Length; matchId++)
        {
            if (_fotballMatches[matchId].BetIsCorrect()) correct++;
        }
        return correct;
    }

    public string FinalReport()
    {
        string reportMessage = "";
        for (int matchId = 0; matchId < _fotballMatches.Length; matchId++)
        {
            string res = _fotballMatches[matchId].BetIsCorrect() switch
            {
                true => "Du tippet riktig",
                false => "Du tippet feil",
            };
            reportMessage += $"Kamp nr. {matchId+1} {res}\n";

        }
        return reportMessage;
    }
}
