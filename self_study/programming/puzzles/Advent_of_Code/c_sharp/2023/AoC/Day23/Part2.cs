namespace AoC.Day23;

class Part2
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
        int R = map.GetLength(0);
        int C = map.GetLength(1);

        (int r, int c) start = (0, 1);
        (int r, int c) target = (R - 1, C - 2);

        (int row, int col)[] DIRECTIONS = [(-1,  0), ( 1,  0), ( 0, -1), ( 0,  1)];

        HashSet<(int r, int c)> VERTICES = [];
        for (int r = 0; r < R; r++)
        {
            for (int c = 0; c < C; c++)
            {
                if (map[r,c] == '#') continue;

                int pathways = 0;
                for (int i = 0; i < 4; i++)
                {
                    int nr = DIRECTIONS[i].row + r;
                    int nc = DIRECTIONS[i].col + c;

                    if (nr >= R || nr < 0) continue;
                    if (nc >= C || nc < 0) continue;
                    if (map[nr,nc] == '#') continue;

                    pathways++;
                }

                if (pathways > 2) VERTICES.Add((r,c));
            }
        }
        VERTICES.Add(start);
        VERTICES.Add(target);

        // Improve time/performance for the brute-force by "compressing" the map meaning that
        // we represent each vertex and its edges with a graph datastructure e.g. an adjacency list.
        Dictionary<(int r, int c), Dictionary<(int r, int c), int>> GRAPH = [];

        // Build graph e.g. connect nodes (vertices via edges).
        foreach ((int r, int c) v in VERTICES)
        {
            GRAPH[v] = [];

            Stack<(int id, int dist, int r, int c)> v_dfs = [];
            v_dfs.Push((0, 0, v.r, v.c));

            // We keep track of each path by assigning an id to it.
            Dictionary<int, bool[,]> v_visited = [];
            v_visited[0] = new bool[R, C];
            v_visited[0][v.r,v.c] = true;

            while (v_dfs.Count > 0)
            {
                (int id, int dist, int r, int c) = v_dfs.Pop();

                int new_id = id;
                int new_dist = dist + 1;

                for (int i = 0; i < 4; i++)
                {
                    int nr = DIRECTIONS[i].row + r;
                    int nc = DIRECTIONS[i].col + c;

                    if (nr == R || nr < 0) continue;
                    if (nc == C || nc < 0) continue;
                    if (v_visited[id][nr,nc]) continue;
                    if (map[nr,nc] == '#') continue;

                    v_visited[id][nr,nc] = true;

                    (int r, int c) new_pos = (nr, nc);

                    if (VERTICES.Contains(new_pos))
                    {
                        GRAPH[v][new_pos] = new_dist;
                    }
                    else
                    {
                        v_dfs.Push((new_id, new_dist, nr, nc));
                    }

                    if (new_id > id)
                    {
                        bool[,] nvisited = new bool[R,C];
                        for (int vr = 0; vr < R; vr++)
                        {
                            for (int vc = 0; vc < C; vc++)
                            {
                                nvisited[vr,vc] = v_visited[id][vr,vc];
                            }
                        }
                        v_visited[new_id] = nvisited;
                    }

                    new_id++;
                }
            }
        }

        int ans = 0;

        // Find longest path using the adjacency list structure usng depth first search.
        HashSet<(int r, int c)> visited = [];
        void RecDFS(int dist, (int r, int c) pos)
        {
            if (pos == target) ans = (dist > ans) ? dist : ans;
            foreach (var node in GRAPH[pos])
            {
                (int r, int c) new_pos = node.Key;
                int new_dist = dist + node.Value;

                if (!visited.Contains(new_pos))
                {
                    visited.Add(new_pos);
                    RecDFS(new_dist, new_pos);
                    visited.Remove(new_pos);
                }
            }
        }

        RecDFS(0, start);

        return ans;
    }
}
