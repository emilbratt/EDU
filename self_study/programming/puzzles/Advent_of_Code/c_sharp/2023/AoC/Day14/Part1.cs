namespace AoC.Day14;

class Part1
{
    public static string Run(string puzzle_input)
    {
        string[] grid = GetPlatformGrid(puzzle_input);

        int res = CalculateLoad(grid);

        return res.ToString();
    }

    private static string[] GetPlatformGrid(string puzzle_input)
    {
        return puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
    }

    private static int CalculateLoad(string[] grid)
    {
        // roll rocks north and calculate the load

        int total_load = 0;

        int row_count = grid.Length;
        int col_count = grid[0].Length;

        // work our way through the input one vertical line at a time
        for (int col = 0; col < col_count; col++)
        {
            int current_load = row_count;
            int round_rocks = 0;

            for (int row = 0; row < row_count; row++)
            {
                if (grid[row][col] == 'O') round_rocks++;

                if (grid[row][col] == '#' || row == row_count - 1)
                {
                    // we hit a wall, lets calculate load using our counted round rocks
                    while (round_rocks > 0)
                    {
                        total_load += current_load;
                        current_load--;
                        round_rocks--;
                    }

                    current_load = row_count - row - 1;
                }
            }
        }

        return total_load;
    }
}
