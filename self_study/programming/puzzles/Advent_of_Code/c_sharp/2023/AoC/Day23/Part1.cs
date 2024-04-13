namespace AoC.Day23;

class Part1
{
    public static string Run(string puzzle_input)
    {
        char[,] map = GetMapOfSnowIsland(puzzle_input);

        int ans = TraverseSnowIsland(map);

        return ans.ToString();
    }

    private static char[,] GetMapOfSnowIsland(string puzzle_input)
    {
        string[] s_grid = puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        char[,] grid = new char[s_grid.Length, s_grid[0].Length];

        for (int row = 0; row < s_grid.Length; row++)
        {
            for (int col = 0; col < s_grid[row].Length; col++)
            {
                grid[row, col] = s_grid[row][col];
            }
        }

        return grid;
    }

    private static int TraverseSnowIsland(char[,] map)
    {
        int ans = 0;

        int R = map.GetLength(0);
        int C = map.GetLength(1);

        (int row, int col)[] DIRECTIONS = [(-1,  0), ( 1,  0), ( 0, -1), ( 0,  1)];

        (int r, int c) start = (0, 1);
        (int r, int c) target = (R - 1, C - 2);

        // Depth First Search using a stack.
        Stack<(int id, int dist, int r, int c)> dfs = [];
        dfs.Push((0, 0, start.r, start.c));

        // We keep track of each path by assigning an id to it.
        Dictionary<int, bool[,]> visited = [];
        visited[0] = new bool[R, C];

        while (dfs.Count > 0)
        {
            (int id, int dist, int r, int c) = dfs.Pop();

            int nid = id;
            dist++;

            for (int i = 0; i <= 3; i++)
            {
                int nr = DIRECTIONS[i].row + r;
                int nc = DIRECTIONS[i].col + c;

                if (nr == target.r && nc == target.c)
                {
                    if (dist > ans) ans = dist;
                    continue; // we are at trails end
                }

                if (nr == R || nr < 0) continue; // to far up or down

                if (nc == C || nc < 0) continue; // to far left or right

                if (visited[id][nr,nc]) continue; // visited

                if (map[nr,nc] == '#') continue; // forest

                if (map[nr,nc] switch
                {
                    '^' => i == 1,
                    'v' => i == 0,
                    '<' => i == 3,
                    '>' => i == 2,
                    _ => false,
                }) continue; // uphill

                visited[id][nr,nc] = true; // OK, good to go

                // If new id has increased, then we know we have encountered an intersection.
                // Lets preserve visited so far, if first path is shorter, re-starting from here is possible.
                if (nid > id)
                {
                    bool[,] nvisited = new bool[R,C];
                    for (int vr = 0; vr < R; vr++)
                    {
                        for (int vc = 0; vc < C; vc++)
                        {
                            nvisited[vr,vc] = visited[id][vr,vc];
                        }
                    }
                    visited[nid] = nvisited;
                }

                dfs.Push((nid, dist, nr, nc));

                nid++;
            }
        }

        return ans;
    }
}
