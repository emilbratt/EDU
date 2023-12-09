namespace TicTacToe;
class Game
{
    public static bool Play()
    {
        BoardModel board_model = new();

        bool player_one = true;
        bool player_two = false;

        bool player_ones_turn = true;

        bool no_winner = true;
        bool squares_left = true;

        while (no_winner && squares_left)
        {
            GameConsole.ShowBoard(board_model);

            if (player_ones_turn)
            {
                Console.Write("Skriv inn hvor du vil sette kryss (f.eks. \"a2\"): ");
                string position = Console.ReadLine()?? " ";
                if (board_model.MartPosition(player_one, position)) player_ones_turn = false;
            }
            else
            {
                Thread.Sleep(200);
                if (board_model.MarkRandomPosition(player_two)) player_ones_turn = true;
            }

            squares_left = board_model.AvailableSquares();
            no_winner = board_model.CheckWin(player_one) ? false
                      : board_model.CheckWin(player_two) ? false
                      : true;
        }

        string winner = board_model.CheckWin(player_one) ? "Player One"
                      : board_model.CheckWin(player_two) ? "Player Two"
                      : "No winners..";

        GameConsole.ShowBoard(board_model);

        Console.WriteLine($"Winner: {winner}");
        Console.Write("Restart [y/N]: ");

        string restart = Console.ReadLine()?? " ";

        if (restart == null || restart.Length < 1)
        {
            return false;
        }

        return restart.ToUpper()[0] == 'Y';
    }
}
