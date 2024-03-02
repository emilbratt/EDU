namespace AoC.Day21;

class Part1
{
    public static string Run(string puzzle_input)
    {
        char[,] map = GetMap(puzzle_input);

        (int start_row, int start_col) = GetStartPosition(map, 'S');

        int steps = 64;

        int res = CalculateReachableGardenPlots(map, start_row, start_col, steps);

        return res.ToString();
    }

    private static char[,] GetMap(string puzzle_input)
    {
        string[] s_grid = puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        char[,] map = new char[s_grid.Length, s_grid[0].Length];

        for (int row = 0; row < s_grid.Length; row++)
        {
            for (int col = 0; col < s_grid[row].Length; col++)
            {
                map[row, col] = s_grid[row][col];
            }
        }

        return map;
    }

    private static (int row, int col) GetStartPosition(char[,] map, char marker)
    {
        for (int row = 0; row < map.GetLength(0); row++)
        {
            for (int col = 0; col < map.GetLength(1); col++)
            {
                if (map[row, col] == marker) return (row, col);
            }
        }

        System.Diagnostics.Debug.Assert(false, " could not findd start position");
        return (-1, -1);
    }

    private static int CalculateReachableGardenPlots(char[,] map, int start_row, int start_col, int total_steps)
    {
        int row_count = map.GetLength(0);
        int col_count = map.GetLength(1);

        bool[,] visited = new bool[map.GetLength(0), map.GetLength(1)]; // default => all elements = false
        int[,] steps_used = new int[map.GetLength(0), map.GetLength(1)]; // default => all elements = 0

        (int row, int col)[] directions = [(-1,  0), ( 0, -1), ( 1,  0), ( 0,  1)];

        int reachable_tiles = 0;

        void RecNextStep(int row, int col, int remaining_steps)
        {
            if (remaining_steps < 0) return;

            // For all new tiles visited, add to the count (but only if ..see below)
            if (!visited[row, col])
            {
                // This is the importent part regarding how we count reachable tiles.
                // We are "locked" to the amount of steps in total.
                // Think of a chess board and that we're starting from top left which is white tile.
                // All odd numbers of steps will make only black tiles reachable.
                // All even numbers of steps will make only white tiles reachable.
                // Essentially, we can only reach half of the tiles.
                reachable_tiles += remaining_steps % 2 == 0 ? 1 : 0;
            }

            visited[row, col] = true;
            remaining_steps--;

            for (int i = 0; i < 4; i++)
            {
                int new_row = row + directions[i].row;
                if (new_row < 0 || new_row == row_count) continue;

                int new_col = col + directions[i].col;
                if (new_col < 0 || new_col == col_count) continue;

                if (map[new_row, new_col] == '#') continue;

                // Comment out this block => you'll be waiting for an eternity (or until the stack overflows)..
                if (visited[new_row, new_col])
                {
                    // If tile is visited and if we used more or an equal amount of steps getting there
                    // (which vaguely indicates going in circles),
                    // then we skip this tile as we have already visited it via a shorter route.
                    // We end up having to run less than a fraction of the iterations we would otherwise have to run.
                    if (steps_used[new_row, new_col] >= remaining_steps) continue;
                }
                steps_used[new_row, new_col] = remaining_steps;

                RecNextStep(new_row, new_col, remaining_steps);
            }
        }

        RecNextStep(start_row, start_col, total_steps);

        return reachable_tiles;
    }
}
