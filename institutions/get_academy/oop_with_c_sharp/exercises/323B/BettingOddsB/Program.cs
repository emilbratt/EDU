namespace BettingOddsB;

internal class Program
{
    private static void Main()
    {
        int betCount = 2;
        var fBets = new FotballBets(betCount);
        fBets.Run();
        fBets.CheckResult();
    }
}
