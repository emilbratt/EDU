namespace BettingOddsB;

class FotballMatch
{
    private int _homeGoals;
    private int _awayGoals;
    private int _matchNumber;
    private bool _concluded;
    private string _bet;

    public FotballMatch(string bet, int matchId)
    {
        _homeGoals = 0;
        _awayGoals = 0;
        _matchNumber = matchId+1;
        _concluded = false;
        _bet = bet.ToUpper();

        Console.WriteLine($"Kamp nr. {_matchNumber} er i gang og du tipper {_bet}");
    }

    private string MatchReport()
    {
        return $"Stillingen for kamp nr. {_matchNumber} er {_homeGoals}-{_awayGoals}.";
    }

    public string AddHomeGoal()
    {
        _homeGoals++;
        Console.WriteLine($"Kamp {_matchNumber} - Hjemmelag scorer!");

        return MatchReport();
    }

    public string AddAwayGaol()
    {
        _awayGoals++;
        Console.WriteLine($"Kamp {_matchNumber} - Bortelag scorer!");

        return MatchReport();
    }

    public string ConcludeMatch()
    {
        if (_concluded) return $"Kamp {_matchNumber} er allerede over!";

        _concluded = true;

        return $"Kamp {_matchNumber} er over!";
    }

    public bool IsConcluded()
    {
        return _concluded;
    }

    public bool BetIsCorrect()
    {
        if (_homeGoals > _awayGoals) return _bet.ToUpper().Contains('H');
        if (_homeGoals < _awayGoals) return _bet.ToUpper().Contains('B');
        if (_homeGoals == _awayGoals) return _bet.ToUpper().Contains('U');
        return false;
    }
}
