namespace AoC.Day2;

class Part2
{
    public static string Run(string[] puzzle_input)
    {
        int result = 0;
        for (int i = 0; i < puzzle_input.Length; i++)
        {
            result += HandlePuzzleLine(puzzle_input[i]);
        }

        return result.ToString();
    }

    private static int HandlePuzzleLine(string line)
    {
        string[] colon_split = line.Split(":");
        string game_record = colon_split[1];

        return CalculatePowerOfMinimumSetOfCubes(game_record);
    }

    private static int CalculatePowerOfMinimumSetOfCubes(string game_record)
    {
        int max_red = 0;
        int max_green = 0;
        int max_blue = 0;
        foreach (string set in game_record.Split(";"))
        {
            foreach (string subset in set.Split(","))
            {
                // remove leading and trailing whitespace
                string trimmed_subset = subset.Trim();

                bool is_numeric = int.TryParse(trimmed_subset.Split(" ")[0], out int cube_count);
                if (!is_numeric) Environment.Exit(1);

                string cube_color = trimmed_subset.Split(" ")[1];
                switch (cube_color)
                {
                    case "red":
                        if (cube_count > max_red) max_red = cube_count;
                        break;
                    case "green":
                        if (cube_count > max_green) max_green = cube_count;
                        break;
                    case "blue":
                        if (cube_count > max_blue) max_blue = cube_count;
                        break;
                    default:
                        Environment.Exit(1);
                        break;
                }
            }
        }

        return max_red * max_green * max_blue;
    }
}
