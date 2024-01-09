namespace FallingParticles;

internal class Particle
{
    public float X { get; set; }
    public float Y { get; set; }

    public void Draw()
    {
        Console.SetCursorPosition((int)X, (int)Y);
        Console.Write('O');
    }
}
