namespace AoC.Day17;

class Part2
{
    public static string Run(string puzzle_input)
    {
        int[,] graph = GetMap(puzzle_input);

        (int row, int col) start = (0, 0); // top left
        (int row, int col) target = (graph.GetLength(0) - 1, graph.GetLength(1) - 1); // bottom right

        int res = Dijkstra(graph, start, target);

        return res.ToString();
    }

    private static int[,] GetMap(string puzzle_input)
    {
        string[] s_grid = puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        int[,] grid = new int[s_grid.Length, s_grid[0].Length];

        for (int row = 0; row < s_grid.Length; row++)
        {
            for (int col = 0; col < s_grid[row].Length; col++)
            {
                grid[row, col] = s_grid[row][col] - '0';
            }
        }

        return grid;
    }

    private static (int, int, int, int, int) PopMin(List<(int dist, int, int, int, int)> queue)
    {
        int min_index = 0;
        var min = queue[min_index];
        for (int i = 0; i < queue.Count; i++)
        {
            if (min.dist > queue[i].dist)
            {
                min = queue[i];
                min_index = i;
            }
        }
        queue.RemoveAt(min_index);

        return min;
    }

    private static bool IsDirectionInvalid(int dir_count, int dir_index, int i)
    {
        if (dir_index == -1) return false;
        if (dir_count < 3 && dir_index != i) return true;
        if (dir_count == 9 && dir_index == i) return true;
        return false;
    }

    private static int Dijkstra(int[,] graph, (int row, int col) start, (int row, int col) target)
    {
        List<int> possible_distances = [];

        int row_count = graph.GetLength(0);
        int col_count = graph.GetLength(1);

        List<(int dist, int row, int col, int dir_count, int dir_index)> queue = [];

        queue.Add( (0, start.row, start.col, 0, -1));

        Dictionary<(int row, int col, int dir_count, int dir_index), int> states = [];

        (int row, int col)[] directions = [(-1,  0), ( 1,  0), ( 0, -1), ( 0,  1)];

        while (queue.Count > 0)
        {
            (int dist, int row, int col, int dir_count, int dir_index) cur_item = PopMin(queue);

            var state = (cur_item.row, cur_item.col, cur_item.dir_count, cur_item.dir_index);

            if (states.ContainsKey(state)) continue;

            int cost = graph[cur_item.row, cur_item.col];
            states.Add(state, cost);

            if (cur_item.row == target.row && cur_item.col == target.col)
            {
                if (cur_item.dir_count >= 3) possible_distances.Add(cur_item.dist);
            }

            for (int i = 0; i < 4; i++)
            {
                bool is_reverse = cur_item.dir_index switch
                {
                    -1 => false, // start node is always -1
                    0 => i == 1,
                    1 => i == 0,
                    2 => i == 3,
                    3 => i == 2,
                    _ => true,
                };
                if (is_reverse) continue;

                int new_row = cur_item.row + directions[i].row;
                int new_col = cur_item.col + directions[i].col;
                if (new_row < 0 || new_row >= row_count || new_col < 0 || new_col >= col_count) continue;

                if (IsDirectionInvalid(cur_item.dir_count, cur_item.dir_index, i)) continue;

                int next_dir_count =  cur_item.dir_index != i ? 0 : cur_item.dir_count + 1;

                var new_state = (new_row, new_col, next_dir_count, i);
                if (states.ContainsKey(new_state)) continue;

                int new_distance = cur_item.dist + graph[new_row, new_col];
                queue.Add( (new_distance, new_row, new_col, next_dir_count, i) );
            }
        }

        return possible_distances.Min();
    }
}
