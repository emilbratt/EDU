namespace AoC.Day17;

class Part1
{
    public static string Run(string puzzle_input)
    {
        int[,] graph = GetMap(puzzle_input);

        (int row, int col) start = (0, 0); // top left
        (int row, int col) target = (graph.GetLength(0) - 1, graph.GetLength(1) - 1); // bottom right

        List<(int, int)> path = Dijkstra(graph, start, target);

        Console.Write("Path: ");
        if (path.Count > 0) foreach (var (row, col) in path) Console.Write($"({row},{col}) ");
        else Console.Write($"..not found");
        Console.WriteLine();

        int res = 0;

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
                grid[row, col] = int.Parse(s_grid[row][col].ToString());
            }
        }

        return grid;
    }

    private static List<(int, int)> Dijkstra(int[,] graph, (int, int) start, (int, int) target)
    {
        List<(int, int)> path = [];

        int row_count = graph.GetLength(0);
        int col_count = graph.GetLength(1);

        Node[,] nodes = new Node[row_count, col_count];
        for (int row = 0; row < row_count; row++)
        {
            for (int col = 0; col < col_count; col++)
            {
                nodes[row, col] = new Node(graph[row, col]);
            }
        }

        nodes[start.Item1, start.Item2].Distance = 0;
        nodes[start.Item1, start.Item2].Cost = 0;

        Queue<(int, int)> queue = [];

        (int row, int col)[] directions = [(1,  0), ( 0,  1), ( 1,  0), ( 0, -1)];

        queue.Enqueue(start);
        bool target_found = false;
        int k = 0;

        while (queue.Count > 0 && !target_found)
        {
            k++;

            (int row, int col) cur_position = queue.Dequeue();

            if (nodes[cur_position.row, cur_position.col].Visited) continue;

            nodes[cur_position.row, cur_position.col].Visited = true;
            target_found = target == cur_position;

            foreach (var next_position in directions)
            {
                int new_row = cur_position.row + next_position.row;
                int new_col = cur_position.col + next_position.col;

                if (new_row < 0 || new_row >= row_count || new_col < 0 || new_col >= col_count) continue;
                if (nodes[new_row, new_col].Visited) continue;

                int new_distance = nodes[cur_position.row, cur_position.col].Distance + nodes[new_row, new_col].Cost;

                if (new_distance < nodes[new_row, new_col].Distance)
                {
                    nodes[new_row, new_col].Distance = new_distance;
                    nodes[new_row, new_col].Previous = cur_position;

                    queue.Enqueue( (new_row, new_col) );
                }
            }
        }

        Console.WriteLine($"Dijkstra loop iterations: {k}");

        if (!target_found) return path;

        // reconstruct path and from the nodes array and add to list
        (int row, int col) previous = target;
        while (previous != start)
        {
            path.Add( previous );
            previous = nodes[previous.row, previous.col].Previous;
        }
        path.Add(start);
        path.Reverse();

        return path;
    }

    // we create a generic type representing every vertex (node)
    private class Node(int cost)
    {
        public int Cost = cost; // cost of distance for this node
        public int Distance = int.MaxValue; // distance from start node (all distances start as infinity)
        public bool Visited = false;
        public (int row, int col) Previous; // undefined until visited
    }
}
