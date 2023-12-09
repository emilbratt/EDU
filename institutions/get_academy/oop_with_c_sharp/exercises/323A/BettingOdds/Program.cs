namespace BettingOdds;

internal class Program
{
    private static void Main()
    {
        var fBets = new FotballBets();
        fBets.Run();
        fBets.CheckResult();
    }
}
