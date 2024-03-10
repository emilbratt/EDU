namespace AoC.Day21;

class Part2
{
    public static string Run(string puzzle_input)
    {
        char[,] map = GetMap(puzzle_input);

        AssertInput(map);

        (int start_row, int start_col) = GetStartPosition(map, 'S');

        const int steps = 26501365;

        long res = CalculateReachableGardenPlotsBFS(map, start_row, start_col, steps);

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

    private static void AssertInput(char[,] map)
    {
        int row_count = map.GetLength(0);
        int C = map.GetLength(1);

        // Our grid should have an odd number of rows and columns.
        System.Diagnostics.Debug.Assert(row_count % 2 == 1, " input has an even amount of rows");
        System.Diagnostics.Debug.Assert(C % 2 == 1, " input has an even amount of columns");

        // We expect our upper and lower edge to have only '.' (no rocks)
        for (int row = 0; row < row_count; row++)
        {
            System.Diagnostics.Debug.Assert(map[row, 0] == '.', " perimeter has blocking rocks '#'");
            System.Diagnostics.Debug.Assert(map[row, C - 1] == '.', " perimeter has blocking rocks '#'");
        }

        // We expect our left and right edge to have only '.' (no rocks)
        for (int col = 0; col < C; col++)
        {
            System.Diagnostics.Debug.Assert(map[0, col] == '.', " perimeter has blocking rocks '#'");
            System.Diagnostics.Debug.Assert(map[row_count - 1, col] == '.', " perimeter has blocking rocks '#'");
        }

        // Make sure we are only working with a squared grid
        System.Diagnostics.Debug.Assert(row_count == C, " grid is not a perfect square");
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

    private static long CalculateReachableGardenPlotsBFS(char[,] map, int sr, int sc, int steps)
    {
        int R = map.GetLength(0);
        int C = map.GetLength(1);

        bool STEPS_ARE_EVEN = steps % 2 == 0;

        int EXPAND_MAP = 5; // How many map copies we allow for increasing the map size in all directions.
        int EXPAND_RANGE = EXPAND_MAP*2+1;

        Dictionary<(int row, int col), int[,]> DISTANCES = [];
        for (int i = 0; i < EXPAND_RANGE * EXPAND_RANGE; i++)
        {
            // We get both row and col by doing this and can skip one extra nesting..
            // It is the same as doing:
            // for (int tr = -EXPAND_MAP; tr <= EXPAND_MAP; tr++)
            //      for (int tc = -EXPAND_MAP; tc <= EXPAND_MAP; tc++)
            int tr = (i / EXPAND_RANGE) - EXPAND_MAP;
            int tc = (i % EXPAND_RANGE) - EXPAND_MAP;

            DISTANCES[(tr, tc)] = new int[R, C];

            for (int j = 0; j < R * C; j++)
            {
                int r = j / R;
                int c = j % C;
                DISTANCES[(tr,tc)][r,c] = -1; // set all default to -1
            }
        }

        Queue<(int tr, int tc, int r, int c, int d)> Q = [];

        // We add the distance offset "1" because 0 == not visited, otherwise we would have used 0..
        Q.Enqueue( (0, 0 ,sr, sc, 0) );
        long ans = 0;

        // Breadth First Search - BFS ..for getting the distance for a subset of the inf. size of the map.
        // Each map-part is represented as a tile which holds all sub-tiles (rows and cols) for the copy of the map.
        while (Q.Count > 0)
        {
            // Q values: tile row, tile column, row, column, distance | tile row and tile column => the map copy.
            (int tr, int tc, int r, int c, int d) = Q.Dequeue();

            if (r < 0)
            {
                tr -= 1;
                r = R - 1;
            }
            if (r == R)
            {
                tr += 1;
                r = 0;
            }
            if (c < 0)
            {
                tc -= 1;
                c = C - 1;
            }
            if (c == C)
            {
                tc += 1;
                c = 0;
            }

            if (map[r,c] == '#') continue; // not allowed to step on
            if (Math.Abs(tr) > EXPAND_MAP) continue; // outside vertical bounds
            if (Math.Abs(tc) > EXPAND_MAP) continue; // outside horizontal bounds
            if (DISTANCES[(tr, tc)][r,c] != -1) continue; // already visited

            if (d <= steps)
            {
                bool dist_is_even = d % 2 == 0;
                if (dist_is_even == STEPS_ARE_EVEN) ans += 1;
            }

            DISTANCES[(tr,tc)][r,c] = d;

            Q.Enqueue( (tr, tc, r+1, c, d+1) );
            Q.Enqueue( (tr, tc, r-1, c, d+1) );
            Q.Enqueue( (tr, tc, r, c+1, d+1) );
            Q.Enqueue( (tr, tc, r, c-1, d+1) );
        }

        Dictionary<(int distance, bool is_corner), long> memoized = [];

        // Iterate over our increased but fixd subset of the infinite map to add all steps occuring outside it.
        // Only distances marked along the perimeter of our increased map are evaluated.
        for (int i = 0; i < R * C; i++)
        {
            int r = i / R;
            int c = i % C;

            // Not visited for original map means it is not visited for all copies too..
            if (DISTANCES[(0, 0)][r, c] == -1) continue;

            for (int j = 0; j < EXPAND_RANGE * EXPAND_RANGE; j++)
            {
                int tr = (j / EXPAND_RANGE) - EXPAND_MAP;
                int tc = (j % EXPAND_RANGE) - EXPAND_MAP;

                bool is_inside_perimeter = Math.Abs(tr) < EXPAND_MAP && Math.Abs(tc) < EXPAND_MAP;
                if (is_inside_perimeter) continue;

                int d = DISTANCES[ (tr,tc) ][r,c];

                bool is_corner = Math.Abs(tr) == EXPAND_MAP && Math.Abs(tc) == EXPAND_MAP;

                (int distance, bool is_corner) key = (d, is_corner);

                bool is_memoized = memoized.ContainsKey(key);

                if (is_memoized) ans += memoized[key];
                if (is_memoized) continue;

                memoized[key] = 0;
                for (int k = 1; k <= (steps-d) / R; k++)
                {
                    int new_dist = d + (k * R);
                    if (new_dist > steps) continue;

                    // Only add for every other tile (this works for both odd and even input for steps..).
                    bool dist_is_even = new_dist % 2 == 0;
                    if (dist_is_even != STEPS_ARE_EVEN) continue;

                    if (is_corner) memoized[key] += k;

                    memoized[key] += 1;
                }

                ans += memoized[key];
            }
        }

        return ans;
    }
}
