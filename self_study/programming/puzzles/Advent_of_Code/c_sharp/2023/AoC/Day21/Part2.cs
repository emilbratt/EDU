namespace AoC.Day21;

class Part2
{
	public static string Run(string puzzle_input)
	{
		char[,] map = GetMap(puzzle_input);

        (int start_row, int start_col) = GetStartPosition(map, 'S');

        map[start_row, start_col] = '.';

		// int steps = 26501365;
		int steps = 100;

		int res = CalculateReachableGardenPlots(map, start_row, start_col, steps);

        Console.WriteLine($"\nresult: {res}");
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

        (int row, int col) start_bound = (0, 0);

        Dictionary<(int row, int col), bool[,]> bounds_visited = [];
        Dictionary<(int row, int col), int[,]> bounds_steps = [];

        bounds_visited[start_bound] = new bool[map.GetLength(0), map.GetLength(1)];
        bounds_steps[start_bound] = new int[map.GetLength(0), map.GetLength(1)];

        (int row, int col)[] directions = [(-1,  0), ( 0, -1), ( 1,  0), ( 0,  1)];

        int reachable_tiles = 0;

        void RecNextStep(int row, int col, int remaining_steps, (int row, int col) bound)
        {
            if (remaining_steps < 0) return;

            if (!bounds_visited[bound][row, col])
            {
                reachable_tiles += remaining_steps % 2 == 0 ? 1 : 0;
            }

            bounds_visited[bound][row, col] = remaining_steps % 2 == 0;
            remaining_steps--;

            for (int i = 0; i < 4; i++)
            {
                int new_row = row + directions[i].row;

                int new_bound_row = bound.row;
                int new_bound_col = bound.col;

                if (new_row < 0)
                {
                    new_bound_row--;
                    new_row = row_count - 1;
                }
                else if (new_row == row_count)
                {
                    new_bound_row++;
                    new_row = 0;
                };

                int new_col = col + directions[i].col;
                if (new_col < 0)
                {
                    new_bound_col--;
                    new_col = col_count - 1;
                }
                else if (new_col == col_count)
                {
                    new_bound_col++;
                    new_col = 0;
                }

                if (!bounds_visited.ContainsKey( (new_bound_row, new_bound_col) ))
                {
                    bounds_visited[ (new_bound_row, new_bound_col) ] = new bool[map.GetLength(0), map.GetLength(1)];
                }

                if (!bounds_steps.ContainsKey( (new_bound_row, new_bound_col) ))
                {
                    bounds_steps[ (new_bound_row, new_bound_col) ] = new int[map.GetLength(0), map.GetLength(1)];
                }

                if (map[new_row, new_col] == '#') continue;

                // Comment out this block => you'll be waiting for an eternity (or until the stack overflows)..
                if (bounds_visited[ (new_bound_row, new_bound_col) ][new_row, new_col])
                {
                    // If tile is visited and if we used more or an equal amount of steps getting there
                    // (which vaguely indicates going in circles),
                    // then we skip this tile as we have already visited it via a shorter route.
                    // We end up having to run less than a fraction of the iterations we would otherwise have to run.
                    if (bounds_steps[ (new_bound_row, new_bound_col) ][new_row, new_col] >= remaining_steps) continue;
                }

                bounds_steps[ (new_bound_row, new_bound_col) ][new_row, new_col] = remaining_steps;

                RecNextStep(new_row, new_col, remaining_steps,  (new_bound_row, new_bound_col) );
            }
        }

        RecNextStep(start_row, start_col, total_steps, start_bound);

        return reachable_tiles;
    }
}
