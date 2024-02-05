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

        // shoelace formula
        (long x, long y) current_pos = (0, 0);
        (long x, long y) previous_pos = (0, 0);
        foreach ((char direction, int meters) in dig_plan)
        {
            current_pos.x += direction_map[direction].col * meters;
            current_pos.y += direction_map[direction].row * meters;

            trench_area += (previous_pos.x * current_pos.y) - (current_pos.x * previous_pos.y);

            trench_circumference += meters; // no part of shoelace formula, but we need the perimeter as well

            previous_pos = (current_pos.x, current_pos.y);
        }

        trench_circumference /= 2;
        trench_area /= 2;

        return Math.Abs(trench_area) + trench_circumference + 1;
    }
}
