namespace AoC.Day14;

class Part2
{
    public static string Run(string puzzle_input)
    {
        var grid = GetPlatformGrid(puzzle_input);

        int cycles = (int)Math.Pow(10, 9); // 1E9 (or 10^9)

        int res = CalculateLoadAfterNCycles(cycles, grid);

        return res.ToString();
    }

    private static char[,] GetPlatformGrid(string puzzle_input)
    {
        string[] lines = puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        var grid = new char[lines.Length,lines[0].Length];

        for (int row = 0; row < lines.Length; row++)
        {
            for (int col = 0; col < lines[row].Length; col++)
            {
                grid[row, col] = lines[row][col];
            }
        }

        return grid;
    }

    private static int CalculateLoadAfterNCycles(int cycles, char[,] grid)
    {
        // TODO: fix this optimistic function (it may not work on all AoC 2023 day 14 inputs)..

        int cycle = 0;

        // run 200 cycles unconditionally to enter the cycle pattern more quickly
        while (cycle < 200)
        {
            cycle++;
            grid = SpinCycle(grid);
        }

        Dictionary<int, List<int>> map_load_to_cycles = [];

        // only run 800, should suffice (see TODO on top) for entering some kind of pattern
        while (cycle < 800)
        {
            cycle++;

            grid = SpinCycle(grid);

            int load = CalculateLoad(grid);

            if (map_load_to_cycles.ContainsKey(load)) map_load_to_cycles[load].Add(cycle);
            else map_load_to_cycles.Add(load, [cycle]);

        }

        // only keep values that might be the final load we are looking for
        List<int> possible_loads = [];
        foreach (var current_cycles in map_load_to_cycles)
        {
            int load = current_cycles.Key;
            List<int> list_cycles = current_cycles.Value;

            bool possible_load = false;
            for (int j = list_cycles.Count - 1; j > 0 ; j--)
            {
                int a = list_cycles[j];
                int b = list_cycles[j-1];
                int diff = a - b;

                int remainder = (cycles - b) % diff;

                if (remainder == 0) possible_load = true;
            }

            if (possible_load) possible_loads.Add(load);
        }

        // optimistically predict (see TODO on top) what the load would be on cycle 10^9
        int least_total_remainder = cycles;
        int final_load = -1;
        foreach (int load in possible_loads)
        {
            List<int> list_cycles = map_load_to_cycles[load];

            int total_remainder = 0;
            for (int j = list_cycles.Count - 1; j > 0 ; j--)
            {
                int a = list_cycles[j];
                int b = list_cycles[j-1];
                int diff = a - b;

                int remainder = (cycles - b) % diff;
                total_remainder += remainder;
            }

            if (total_remainder < least_total_remainder)
            {
                least_total_remainder = total_remainder;
                final_load = load;
            }
        }

        return final_load;
    }

    private static int CalculateLoad(char[,] grid)
    {
        int load = 0;

        int row_count = grid.GetLength(0);
        int col_count = grid.GetLength(1);

        for (int row = 0; row < row_count; row++)
        {
            for (int col = 0; col < col_count; col++)
            {
                load += grid[row, col] == 'O' ? row_count-row : 0;
            }
        }

        return load;
    }

    private static char[,] SpinCycle(char[,] grid)
    {
        // tilt north (no rotation)
        grid = RollUpwards(grid);

        // tilt west
        grid = RotateRight(grid);
        grid = RollUpwards(grid);

        // tilt south
        grid = RotateRight(grid);
        grid = RollUpwards(grid);

        // tilt east
        grid = RotateRight(grid);
        grid = RollUpwards(grid);

        // rotate back to original
        grid = RotateRight(grid);
        return grid;
    }

    private static char[,] RotateRight(char[,] grid)
    {
        int row_count = grid.GetLength(0);
        int col_count = grid.GetLength(1);

        char[,] rotated = new char[col_count, row_count];

        // this will also work on non squared 2d-arrays
        for (int row = 0; row < row_count; row++)
        {
            for (int col = 0; col < col_count; col++)
            {
                // transpose (row,col -> col,row) and reverse rows (row->rows-row-1) ..in one line
                rotated[col, row_count-1-row] = grid[row, col];
            }
        }

        return rotated;
    }

    private static char[,] RollUpwards(char[,] grid)
    {
        /*
            all 'O' fall up towards '#' or the edge like shown below

            before |  after
            -------|-------
            ..#.   |   OO#.
            ...#   |   O.O#
            O.O.   |   ...O
            OO.O   |   ....
        */

        int row_count = grid.GetLength(0);
        int col_count = grid.GetLength(1);

        char[,] tilted = new char[row_count, col_count];

        // work our way through the input one vertical line at a time
        for (int col = 0; col < col_count; col++)
        {
            int round_rocks = 0;

            for (int row = row_count - 1; row >= 0; row--)
            {
                tilted[row, col] = grid[row, col] == 'O' ? '.' : grid[row, col];

                if (grid[row, col] == 'O') round_rocks++;

                if (grid[row, col] == '#')
                {
                    // we hit a wall, lets calculate load using our counted round rocks
                    while (round_rocks > 0)
                    {
                        tilted[row+round_rocks, col] = 'O';
                        round_rocks--;
                    }
                }
                else if (row == 0)
                {
                    // we hit a wall, lets calculate load using our counted round rocks
                    while (round_rocks > 0)
                    {
                        tilted[row+round_rocks-1, col] = 'O';
                        round_rocks--;
                    }
                }
            }
        }

        return tilted;
    }
}
