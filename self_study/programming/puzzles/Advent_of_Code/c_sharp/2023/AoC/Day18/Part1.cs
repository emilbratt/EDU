namespace AoC.Day18;

class Part1
{
    public static string Run(string puzzle_input)
    {
        var dig_plan = GetDigPlan(puzzle_input);

        int res = DigTrench(dig_plan);

        return res.ToString();
    }

    private static (char direction, int meters)[] GetDigPlan(string puzzle_input)
    {
        string[] lines = puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        var dig_plan = new(char direction, int meters)[lines.Length];

        for (int i = 0; i < lines.Length; i++)
        {
            string[] parts = lines[i].Split();
            dig_plan[i] = (lines[i][0], int.Parse(parts[1]));
        }

        return dig_plan;
    }

    private static int DigTrench((char direction, int meters)[] dig_plan)
    {
        var direction_map = new Dictionary<char, (int row, int col)>()
        {
            { 'U', (-1,  0) },
            { 'D', ( 1,  0) },
            { 'R', ( 0,  1) },
            { 'L', ( 0, -1) },
        };

        int trench_circumference = 0;
        int trench_area = 0;

        // Calculate the fields offset
        int max_row = 0;
        int min_row = 0;
        int cur_row = 0;
        int max_col = 0;
        int min_col = 0;
        int cur_col = 0;

        foreach ((char direction, int meters) in dig_plan)
        {
            if (direction == 'R') cur_col += meters;
            else if (direction == 'L') cur_col -= meters;
            else if (direction == 'D') cur_row += meters;
            else if (direction == 'U') cur_row -= meters;

            if (cur_col > max_col) max_col = cur_col;
            else if (cur_row > max_row) max_row = cur_row;
            else if (cur_col < min_col) min_col = cur_col;
            else if (cur_row < min_row) min_row = cur_row;
        }

        // Apply offset to row and collumn
        int total_rows = Math.Abs(min_row) + max_row + 1;
        int total_cols = Math.Abs(min_col) + max_col + 1;

        // the field with our trench, every tile == true => part of the trench
        var field = new bool[total_rows, total_cols];

        int row_pointer = 0 + Math.Abs(min_row);
        int col_pointer = 0 + Math.Abs(min_col);

        foreach ((char direction, int meters) in dig_plan)
        {
            var (row, col) = direction_map[direction];

            for (int i = 0; i < meters; i++)
            {
                row_pointer += row;
                col_pointer += col;

                field[row_pointer, col_pointer] = true;

                trench_circumference++;
            }
        }

        // Enqueue rows and collumns from all four sides (if not part of trench)
        Queue<(int row, int col)> queue = [];
        for (int row = 0; row < total_rows; row++)
        {
            if (!field[row, 0]) queue.Enqueue((row, 0));
            if (!field[row, total_cols - 1]) queue.Enqueue((row, total_cols - 1));
        }
        for (int col = 1; col < total_cols - 1; col++)
        {
            if (!field[0, col]) queue.Enqueue((0, col));
            if (!field[total_rows - 1, col]) queue.Enqueue((total_rows - 1, col));
        }

        // Start flood fill on tiles located outside the circumference of the trench
        while (queue.Count > 0)
        {
            (int row, int col) = queue.Dequeue();

            if (field[row, col] == true) continue;

            field[row, col] = true;

            foreach (var direction in direction_map.Values)
            {
                int new_row = row + direction.row;
                int new_col = col + direction.col;

                if (new_row < 0 || new_row >= total_rows || new_col < 0 || new_col >= total_cols) continue;
                if (field[new_row, new_col] == true) continue;

                queue.Enqueue( (new_row, new_col) );
            }
        }

        // What is left after flood fill will give us the area of the trench
        foreach (bool is_outside in field)
        {
            if (!is_outside) trench_area++;
        }

        return trench_circumference + trench_area;
    }
}
