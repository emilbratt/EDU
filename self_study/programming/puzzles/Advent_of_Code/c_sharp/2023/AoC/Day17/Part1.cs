using System.Formats.Tar;

namespace AoC.Day17;

class Part1
{
    public static string Run(string puzzle_input)
    {
        int[,] graph = GetMap(puzzle_input);
        (int row, int col) start_node = (7, 6);
        // (int row, int col) target_node = (graph.GetLength(0) - 1, graph.GetLength(1) - 1);
        (int row, int col) target_node = (3, 3);
        Dijkstra(graph, start_node, target_node);

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

    private static void Dijkstra(int[,] graph, (int row, int col) start_node, (int row, int col) target_node)
    {
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
        nodes[start_node.row, start_node.col].Distance = 0;
        nodes[start_node.row, start_node.col].Previous = start_node;
        nodes[start_node.row, start_node.col].Cost = 0;
        List<(int row, int col)> queue = []; // first in first out
        (int row, int col)[] directions = [
            (-1,  0),
            ( 0,  1),
            ( 1,  0),
            ( 0, -1),
        ];

        // hvilke data må vi ha med
        // queue --> nodes from graph[row, col]
        // cost/distance --> value in graph[row, col]
        // visited --> array with bool[node] true/false
        // previous_nodes --> array (row, col)[row,col] inneholde forrige node ved å slå opp nåværende node --> (forrige)[nåværende]

        queue.Add(start_node);
        int wd = 0;
        /// DIJKSTRA STARTS HERE
        while (queue.Count > 0)
        {
            wd++;
            if (wd > 1000) return;

            // dequeue
            (int cur_row, int cur_col) = queue[0];
            queue.RemoveAt(0);
            Node cur_node = nodes[cur_row, cur_col];

            if (cur_node.Visited) continue;
            cur_node.Visited = true;

            if (cur_row == target_node.row && cur_col == target_node.col) break;


            foreach (var direction in directions)
            {
                int new_row = cur_row + direction.row;
                int new_col = cur_col + direction.col;

                bool valid = new_row >= 0 && new_row < row_count && new_col >= 0 && new_col < col_count;

                if (valid && !nodes[new_row, new_col].Visited)
                {
                    int new_distance = nodes[cur_row, cur_col].Distance + nodes[new_row, new_col].Cost;
                    if (new_distance <= nodes[new_row, new_col].Distance)
                    {
                        // Console.WriteLine($"new dist: {cur_row},{cur_col}");
                        nodes[new_row, new_col].Distance = new_distance;
                        nodes[new_row, new_col].Previous = (cur_row, cur_col);
                    }
                    queue.Add((new_row, new_col));
                }
            }
        }
        
        /// DIJKSTRA ENDS HERE

        Console.WriteLine($"Distance to target: {nodes[target_node.row, target_node.col].Distance}");
        /* Console.WriteLine($"Distance to target: {nodes[target_node.row, target_node.col].Previous}");

        List<(int row, int col)> path = []; */
        
        for (int row = 0; row < row_count; row++)
        {
            for (int col = 0; col < col_count; col++)
            {
                if(nodes[row, col].Visited)
                {
                    Console.ForegroundColor = ConsoleColor.Green;
                }
                string text = $"{graph[row, col]} {nodes[row, col].Previous} ";
                Console.Write($"{graph[row, col]} ");
                Console.ForegroundColor = ConsoleColor.White;
                
            }
            Console.WriteLine();
        }
        Console.WriteLine();
        Console.SetCursorPosition(0, row_count + 3);
        Console.Write(nodes[start_node.row, start_node.col].Previous);

        int s_row = target_node.row;
        int s_col = target_node.col;
        int current_line = row_count;
        // while (!(s_row == start_node.row && s_col == start_node.col))
        List<(int row, int col)> path =[];
        int k = 0;
        while (true)
        {
            k++;
            if (k > 1000) return;
            Console.ForegroundColor = ConsoleColor.Red;
            Console.SetCursorPosition(s_col * 2, s_row +1);
            Console.Write(graph[s_row, s_col]);

            if (s_row == start_node.row && s_col == start_node.col) break;
            s_row = nodes[s_row, s_col].Previous.row;
            s_col = nodes[s_col, s_col].Previous.col;


            /* Console.ForegroundColor = ConsoleColor.White;
            current_line++;
            Console.SetCursorPosition(0, current_line);
            Console.Write(nodes[s_row, s_col].Previous); */

            path.Add( (s_row, s_col) );
        }
        path.Reverse();

        Console.SetCursorPosition(0, row_count + 3);
        Console.ForegroundColor = ConsoleColor.White;
    }

    private class Node(int cost)
    {
        public int Cost = cost;
        public int Distance = int.MaxValue;
        public bool Visited = false;
        public (int row, int col) Previous = (-1, -1);
    }
}

/*
    hente ut --> node = queue[0] --> queue.RemoveAt(0);

    legge til --> queue.Add(node)
*/


/*

graph

    2 4 1 3 4 3 2 3 1 1 3 2 3
    3 2 1 5 4 5 3 5 3 5 6 2 3
    3 2 5 5 2 4 5 6 5 4 2 5 4
    3 4 4 6 5 8 5 8 4 5 4 5 2
    4 5 4 6 6 5 7 8 6 7 5 3 6
    1 4 3 8 5 9 8 7 9 8 4 5 4
    4 4 5 7 8 7 6 9 8 7 7 6 6
    3 6 3 7 8 7 7 9 7 9 6 5 3
    4 6 5 4 9 6 7 9 8 6 8 8 7
    4 5 6 4 6 7 9 9 8 6 4 5 3
    1 2 2 4 6 8 6 8 6 5 5 6 3
    2 5 4 6 5 4 8 8 8 7 7 3 5
    4 3 2 2 6 7 4 6 5 5 5 3 3


Distance to target: −2147483636
*/
