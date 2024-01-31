namespace AoC.Day18;

class Part2
{
    public static string Run(string puzzle_input)
    {
        var dig_plan = GetDigPlan(puzzle_input);

        // foreach ((char direction, int meters) in dig_plan) Console.WriteLine($"Dir. '{direction}' | {meters}m.");

        return String.Empty;
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
}
