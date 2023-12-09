class SlideBoardActions
{
    private readonly SlideBoardModel Board;

    public SlideBoardActions(SlideBoardModel board)
    {
        Board = board;
    }

    public void ShuffleNumbers(int slides)
    {
        Random random = new();

        int number = random.Next(1, Board.GetLargestNumber());

        // enforce more entropy by disallowing sliding of the previous number
        int previous_number = number + 1;

        while (slides > 0)
        {
            if (number != previous_number && Board.NumberCanMove(number))
            {
                SlideNumber(number);
                previous_number = number;

                if (!Board.IsSolved())
                {
                    // only subtract if the board is not solved
                    // ..it avoids starting out with a solved board (not a likely scenario though)
                    // however, it would be guaranteed to happen if you for example
                    // ..select only 2 rows and 2 collumns and then select 12 shuffles
                    slides--;
                }
            }

            number = random.Next(1, Board.GetLargestNumber());
        }
    }

    public int SelectNumber()
    {
        Console.Write("Number to slide: ");
        while (true)
        {
            if (int.TryParse(Console.ReadLine() ?? "", out int number))
            {
                if (Board.NumberCanMove(number)) return number;
            }
            Console.WriteLine("Type in a number adjacent to the blank tile");
        }
    }

    public bool SlideNumber(int selected_number)
    {
        int blank_tile = Board.GetLargestNumber();

        int selected_row = Board.GetRow(selected_number);
        int selected_col = Board.GetCollumn(selected_number);

        int blank_row = Board.GetRow(blank_tile);
        int blank_col = Board.GetCollumn(blank_tile);

        Board.SetNumber(selected_row, selected_col, blank_tile);
        Board.SetNumber(blank_row, blank_col, selected_number);

        if (Board.IsSolved()) return true;
        else return false;
    }

    public void PrintBoard()
    {
        Console.Clear();
        Console.WriteLine("Board:");
        foreach (int row in Board.Rows())
        {
            string number_row = "|";
            foreach (int col in Board.Columns())
            {
                string tile = "";

                int number = Board.GetNumber(row, col);
                if (number < Board.GetLargestNumber()) tile = number.ToString();

                tile = tile.PadLeft(Board.GetLargestNumber().ToString().Length, ' ');
                number_row += " " + tile + " | ";
            }
            if (row == 1) Console.WriteLine("".PadLeft(number_row.Length-1, '-'));
            Console.WriteLine(number_row);
            Console.WriteLine("".PadLeft(number_row.Length-1, '-'));
        }
    }
}
