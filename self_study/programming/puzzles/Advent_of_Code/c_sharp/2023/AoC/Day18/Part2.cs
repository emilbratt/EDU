namespace AoC.Day18;

class Part2
{
    public static string Run(string puzzle_input)
    {
        var dig_plan = GetDigPlan(puzzle_input);

        long res = DigTrench(dig_plan);

        return res.ToString();
    }

    private static (char direction, int meters)[] GetDigPlan(string puzzle_input)
    {
        string[] lines = puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        var dig_plan = new(char direction, int meters)[lines.Length];

        for (int i = 0; i < lines.Length; i++)
        {
            string hex_part = lines[i].Substring(lines[i].Length - 7, 7);

            char direction = hex_part[5] switch
            {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => 'X',
            };

            int meters = Convert.ToInt32(hex_part.Substring(0, 5), 16);

            dig_plan[i] = (direction, meters);
        }

        return dig_plan;
    }

    private static long DigTrench((char direction, int meters)[] dig_plan)
    {
        var direction_map = new Dictionary<char, (int row, int col)>()
        {
            { 'U', (-1,  0) },
            { 'D', ( 1,  0) },
            { 'R', ( 0,  1) },
            { 'L', ( 0, -1) },
        };

        long trench_circumference = 0;
        long trench_area = 0;

        (int x, int y) cur_pos = (0, 0);
        List<(int x, int y)> coordinates = [];

        foreach ((char direction, int meters) in dig_plan)
        {
            var (row, col) = direction_map[direction];

            for (int i = 0; i < meters; i++)
            {
                cur_pos.x += col;
                cur_pos.y += row;
                trench_circumference++;
            }
            coordinates.Add(cur_pos);
        }

        trench_circumference /= 2;

        for (int i = 1; i < coordinates.Count; i++)
        {
            (long x1, long y1) = coordinates[i-1];
            (long x2, long y2) = coordinates[i];

            trench_area += (x1 * y2) - (x2 * y1);
        }

        trench_area /= 2;
        trench_area = Math.Abs(trench_area);

        return trench_area + trench_circumference + 1;
    }
}
