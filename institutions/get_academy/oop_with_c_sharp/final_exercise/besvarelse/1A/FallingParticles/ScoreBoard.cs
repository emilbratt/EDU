namespace FallingParticles;

internal class ScoreBoard : IComponents
{
    public int Level = 1;
    public int Score = 0;

    public void DrawComponent()
    {
        string s = $"Score: {Score} | Level: {Level}";
        Console.SetCursorPosition(Console.WindowWidth / 2 - s.Length/2, 0);
        Console.Write(s);
    }

    public void ResetComponent()
    {
        Level = 1;
        Score = 0;
    }
}
