namespace FallingParticles;

internal class Paddle : IComponents
{
    private static readonly string BOARD = "========";
    private readonly int _Y_POSITION = Console.WindowHeight - 1;
    private readonly int _MOVE_LEN = BOARD.Length / 2;
    private int _x_start = Console.WindowWidth / 2 - BOARD.Length / 2;
    private int _x_end => _x_start + BOARD.Length - 1;

    public void DrawComponent()
    {
        Console.SetCursorPosition(_x_start, _Y_POSITION);
        Console.Write(BOARD);
    }

    public void ResetComponent()
        => _x_start = Console.WindowWidth / 2 - BOARD.Length / 2;

    public (int x_start, int x_end) Move(ConsoleKey key)
    {
        _x_start += key switch
        {
            ConsoleKey.LeftArrow => -_MOVE_LEN,
            ConsoleKey.RightArrow => _MOVE_LEN,
            _ => 0, // no change in direction
        };

        // a simple collition detection forcing the board back inside the console window
        _x_start = _x_start > Console.WindowWidth - BOARD.Length - 1 ? Console.WindowWidth - BOARD.Length - 1
                    : _x_start < 0 ? 0
                    : _x_start;

        // return the boards position
        return (_x_start, _x_end);
    }
}
