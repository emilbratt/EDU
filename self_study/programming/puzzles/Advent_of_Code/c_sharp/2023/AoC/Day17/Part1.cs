namespace AoC.Day17;

/*
    regel: må snu 90 grader etter 3 noder
*/

class Part1
{
    public static string Run(string puzzle_input)
    {
        int[,] graph = GetMap(puzzle_input);

        (int row, int col) start = (0, 0); // top left
        (int row, int col) target = (graph.GetLength(0) - 1, graph.GetLength(1) - 1); // bottom right

        List<(int, int)> path = Dijkstra(graph, start, target);

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

    private static List<(int, int)> Dijkstra(int[,] graph, (int row, int col) start, (int row, int col) target)
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

        List<(int row, int col, int same_direction_count)> queue = [];

        (int row, int col)[] directions = [(-1,  0), ( 1,  0), ( 0, -1), ( 0,  1)];

        queue.Add( (start.row, start.col, 0 ));
        DrawGraph(graph);

        while (queue.Count > 0)
        {
            // (int row, int col, int same_direction_count) current_item = queue.Dequeue();
            queue.Sort((a, b) =>
                nodes[a.row, a.col].Distance < nodes[b.row, b.col].Distance ? -1 : 1);

            (int row, int col, int same_direction_count) current_item = queue[0];
            queue.RemoveAt(0);

            (int row, int col) cur_position = (current_item.row, current_item.col);

            Node current_node = nodes[cur_position.row, cur_position.col];
            
            if (current_node.Visited) continue;
            current_node.Visited = true;
            Console.SetCursorPosition(current_item.col, current_item.row);
            Console.BackgroundColor = ConsoleColor.DarkCyan;
            Console.Write(current_node.Cost);
            Console.BackgroundColor = ConsoleColor.Black;
            Thread.Sleep(50);
            /* if (target == (cur_position.row, cur_position.col)) break; */

            for (int i = 0; i < 4; i++)
            {
                var next_position = directions[i];
                int new_row = cur_position.row + next_position.row;
                int new_col = cur_position.col + next_position.col;

                if (new_row < 0 || new_row >= row_count || new_col < 0 || new_col >= col_count) continue;

                Node next_node = nodes[new_row, new_col];

                int new_distance = current_node.Distance + next_node.Cost;

                if (new_distance < next_node.Distance)
                {
                    next_node.Distance = new_distance;
                    next_node.Previous = cur_position;

                    next_node.LastDirection = (Direction)i;
                    bool is_queued = false;
                    if (current_node.LastDirection == next_node.LastDirection)
                    {
                        int next_same_direction_count = current_item.same_direction_count + 1;
                        if (next_same_direction_count < 3)
                        {
                            queue.Add( (new_row, new_col, next_same_direction_count) );
                            is_queued = true;
                        }
                    }
                    else
                    {
                        queue.Add( (new_row, new_col, 0) );
                        is_queued = true;
                    } 
                    if (is_queued)
                    {
                        Console.SetCursorPosition(new_col, new_row);
                        Console.BackgroundColor = ConsoleColor.DarkGreen;
                        Console.Write(next_node.Cost);
                        Console.BackgroundColor = ConsoleColor.Black;
                    }
                    // queue.Add( (new_row, new_col, 0) );
                }
            }
        }

        // reconstruct path and from the nodes array and add to list
        (int row, int col) previous = target;
        while (previous != start)
        {
            path.Add( previous );
            previous = nodes[previous.row, previous.col].Previous;
        }
        path.Add(start);
        path.Reverse();

        Print(nodes, path);
        return path;
    }

    // we create a generic type representing every vertex (node)
    private class Node(int cost)
    {
        public int Cost = cost; // cost of distance for this node
        public int Distance = int.MaxValue; // distance from start node (all distances start as infinity)
        public bool Visited = false;
        public (int row, int col) Previous; // undefined until visited
        public Direction LastDirection;
    }

    private enum Direction
    {
        Up,
        Down,
        Left,
        Right,
    }

    private static void Print(Node[,] nodes, List<(int, int)> path)
    {
        char[] arrows = ['^', 'v', '<', '>'];
        
        int total_arrow_cost = 0;
        path.RemoveAt(0);
        Console.CursorVisible = false;
        Console.BackgroundColor = ConsoleColor.Yellow;
        foreach (var (row, col) in path)
        {
            total_arrow_cost += nodes[row, col].Cost;
            Console.SetCursorPosition(col, row);
            Console.Write(arrows[(int)nodes[row, col].LastDirection]);
            Thread.Sleep(50);
        }
        Console.BackgroundColor = ConsoleColor.Black;
        Console.WriteLine($"\nTotal cost of printed arrows: {total_arrow_cost}");
        Console.WriteLine($"\nTotal distance stored in end node: {nodes[nodes.GetLength(0) - 1, nodes.GetLength(1) - 1].Distance}");

        Console.CursorVisible = true;
    }

    private static void DrawGraph(int[,] graph)
    {
        for (int row = 0; row < graph.GetLength(0); row++)
        {
            for (int col = 0; col < graph.GetLength(1); col++)
            {
                Console.SetCursorPosition(col, row);
                Console.Write(graph[row, col]);
            }
        }
    }
}

/*
2>>34^>>>1323  0>>34^>>>1323  2413432311323
32v>>>35v5623  32v>>>35v>623  3215453535623
32552456v>>54  325524565v>54  3255245654254
3446585845v52  3446585845v52  3446585845452
4546657867v>6  4546657867v>6  4546657867536
14385987984v4  14385987984v4  1438598798454
44578769877v6  44578769877v6  4457876987766
36378779796v>  3637877979<v3  3637877979653
465496798688v  4654967986v87  4654967986887
456467998645v  4564679986v>>  4564679986453
12246868655<v  122468686556v  1224686865563
25465488877v5  254654888773v  2546548887735
43226746555v>  432267465553v  4322674655533
*/
