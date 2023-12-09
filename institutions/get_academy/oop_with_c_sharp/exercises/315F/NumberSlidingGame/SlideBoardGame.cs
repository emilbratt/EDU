class SlideBoardGame
{
    public static void Run()
    {
        Console.Write("How many rows: ");

        int rows = 1;

        while (rows == 1)
        {
            if (int.TryParse(Console.ReadLine() ?? "", out int number))
            {
                if (number > 1) rows = number;
                else Console.WriteLine("Number must be a valid integer greater than 1");
            }
        }

        Console.Write("How many collumns: ");

        int collumns = 1;

        while (collumns == 1)
        {
            if (int.TryParse(Console.ReadLine() ?? "", out int number))
            {
                if (number > 1) collumns = number;
                else Console.WriteLine("Number must be a valid integer greater than 1");
            }
        }

        int easy = rows * collumns;
        int normal = easy * 5;
        int hard = normal * 5;
        Console.WriteLine("Shuffle board by sliding random tiles (more slides will introduce more entropy, but never guaranteed)");
        Console.WriteLine($"Here are some suggestions based on the board size:");
        Console.WriteLine($" Easy {easy}");
        Console.WriteLine($" Normal {normal}");
        Console.WriteLine($" Hard {hard}");
        Console.Write("How many slides?: ");

        int slides = 0;

        while (slides == 0)
        {
            if (int.TryParse(Console.ReadLine() ?? "", out int number))
            {
                if (number > 0) slides = number;
                else Console.WriteLine("Number must be a valid integer greater than 0");
            }
        }

        var board   = new SlideBoardModel(rows, collumns);
        var actions = new SlideBoardActions(board);

        actions.ShuffleNumbers(slides);
        actions.PrintBoard();

        int selected_number;
        bool solved = false;

        while (!solved)
        {
            selected_number = actions.SelectNumber();
            solved = actions.SlideNumber(selected_number);
            actions.PrintBoard();
        }

        Console.WriteLine("Congratulations, you completed the board!");
    }
}
