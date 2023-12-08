namespace AoC.Day2;

class Part1
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

        if (!GameIsPossible(game_record)) return 0;

        string string_game_id = colon_split[0].Split(" ")[1];
        bool game_id_extracted = int.TryParse(string_game_id, out int game_id);
        if (!game_id_extracted) Environment.Exit(1);

        return game_id;
    }

    private static bool GameIsPossible(string game_record)
    {
        foreach (string set in game_record.Split(";"))
        {
            foreach (string subset in set.Split(","))
            {
                // remove leading and trailing whitespace
                string trimmed_subset = subset.Trim();

                bool is_numeric = int.TryParse(trimmed_subset.Split(" ")[0], out int cube_count);
                if (!is_numeric) Environment.Exit(1);

                string cube_color = trimmed_subset.Split(" ")[1];

                bool not_possible = cube_color switch
                {
                    "red" => cube_count > 12,
                    "green" => cube_count > 13,
                    "blue" => cube_count > 14,
                    _ => false,
                };

                if (not_possible) return false;
            }
        }

        return true;
    }
}
