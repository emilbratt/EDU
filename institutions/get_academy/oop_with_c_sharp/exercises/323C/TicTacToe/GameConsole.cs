namespace TicTacToe;
class GameConsole
{
    public static void ShowBoard(BoardModel board_model)
    {
        Console.Clear();

        Console.WriteLine(" ┌─────┐");

        foreach (int row in board_model.Rows())
        {
            string row_string = $"{row}│";

            foreach(int col in board_model.Columns())
            {
                row_string += board_model.GetSquareValue(row, col);
                if (col < board_model.Width()) row_string += " ";
            }

            row_string += "│";
            Console.WriteLine(row_string);
        }

        Console.WriteLine(" └─────┘");
        Console.WriteLine("  A B C ");
    }
}
