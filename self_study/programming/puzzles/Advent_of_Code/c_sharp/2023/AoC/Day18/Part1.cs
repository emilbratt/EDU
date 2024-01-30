using System.Diagnostics;

namespace AoC.Day18;

class Part1
{
    public static string Run(string puzzle_input)
    {
        var dig_plan = GetDigPlan(puzzle_input);

        int res = DigTrench(dig_plan);

        return res.ToString();
    }

    private static (char direction, int meters, string color)[] GetDigPlan(string puzzle_input)
    {
        string[] lines = puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        var dig_plan = new(char direction, int meters, string color)[lines.Length];

        for (int i = 0; i < lines.Length; i++)
        {
            var parts = lines[i].Split();
            dig_plan[i] = (lines[i][0], int.Parse(parts[1]), parts[2]);
        }

        return dig_plan;
    }

    private static int DigTrench((char direction, int meters, string color)[] dig_plan)
    {
        int max_row = 0;
        int min_row = 0;
        int cur_row = 0;
        int max_col = 0;
        int min_col = 0;
        int cur_col = 0;

        var direction_map = new Dictionary<char, (int row, int col)>()
        {
            { 'U', (-1,  0) },
            { 'D', ( 1,  0) },
            { 'R', ( 0,  1) },
            { 'L', ( 0, -1) },
        };

        foreach (var t in dig_plan)
        {
            if (t.direction == 'R') cur_col += t.meters;
            else if (t.direction == 'L') cur_col -= t.meters;
            else if (t.direction == 'D') cur_row += t.meters;
            else if (t.direction == 'U') cur_row -= t.meters;

            if (cur_col > max_col) max_col = cur_col;
            else if (cur_row > max_row) max_row = cur_row;
            else if (cur_col < min_col) min_col = cur_col;
            else if (cur_row < min_row) min_row = cur_row;
        }

        int total_rows = Math.Abs(min_row) + max_row + 1;
        int total_cols = Math.Abs(min_col) + max_col + 1;

        var field = new int[total_rows, total_cols];

        int row_pointer = 0 + Math.Abs(min_row);
        int col_pointer = 0 + Math.Abs(min_col);

        foreach (var t in dig_plan)
        {
            var current_direction = direction_map[t.direction];
            field[row_pointer, col_pointer] = 1;
            for (int i = 0; i < t.meters; i++)
            {
                row_pointer += current_direction.row;
                col_pointer += current_direction.col;
                field[row_pointer, col_pointer] = 1;
            }
        }

        Queue<(int row, int col)> queue = [];

        bool[,] visited_graph = new bool[total_rows, total_cols];

        for (int i = 0; i < total_rows; i++)
        {
            if (field[i, 0] == 0) queue.Enqueue((i, 0));
            if (field[i, total_cols - 1] == 0) queue.Enqueue((i, total_cols - 1));
        }
        for (int i = 1; i < total_cols - 1; i++)
        {
            if (field[0, i] == 0) queue.Enqueue((0, i));
            if (field[total_rows - 1, i] == 0) queue.Enqueue((total_rows - 1, i));
        }

        char[] directions = ['U', 'D', 'R', 'L'];

        while (queue.Count > 0)
        {
            (int row, int col) = queue.Dequeue();

            if (visited_graph[row, col]) continue;

            field[row, col] = 2;
            visited_graph[row, col] = true;

            foreach (char direction in directions)
            {
                var current_direction = direction_map[direction];

                int new_row = current_direction.row + row;
                int new_col = current_direction.col + col;

                if (new_row < 0 || new_row >= total_rows || new_col < 0 || new_col >= total_cols) continue;
                if (field[new_row, new_col] != 0) continue;

                queue.Enqueue((new_row, new_col));
            }
        }

        int tiles_dug = 0;

        foreach (var num in field)
        {
            if (num != 2) tiles_dug++;
        }

        return tiles_dug;
    }
}
