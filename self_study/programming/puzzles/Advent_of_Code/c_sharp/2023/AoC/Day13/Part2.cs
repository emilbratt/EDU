namespace AoC.Day13;

class Part2
{
    public static string Run(string puzzle_input)
    {
        var mirror_patterns = GetMirrorPatterns(puzzle_input);

        int res = 0;

        foreach ((bool[][] horizontal, bool[][] vertical_rotated) in mirror_patterns)
        {
            int res_horizontal = ReflectionWithSmudge(horizontal);
            int res_vertical = ReflectionWithSmudge(vertical_rotated);

            res += 100*res_horizontal + res_vertical;
        }

        return res.ToString();
    }

    // return two versions where the vertical is rotated 90 degrees so we can use same implementations for it
    private static List<(bool[][] horizontal, bool[][] vertical_rotated)> GetMirrorPatterns(string puzzle_input)
    {
        List<(bool[][] horizontal, bool[][] vertical)> patterns = [];

        string[] parts = puzzle_input.Split("\n\n", StringSplitOptions.RemoveEmptyEntries);

        foreach (string part in parts)
        {
            string[] part_lines = part.Split('\n', StringSplitOptions.RemoveEmptyEntries);

            int rows = part_lines.Length;
            int columns = part_lines[0].Length;

            bool[][] arr_horizontal = new bool[rows][];
            bool[][] arr_vertical = new bool[columns][];

            int i = 0; // we can use i for both rows and columns
            while (i < rows || i < columns)
            {
                // horizontal iteration (rows) -> extract and keep as is
                if (i < rows)
                    arr_horizontal[i] = part_lines[i]
                        .Select(s => s == '#')
                        .ToArray();

                // vertical iteration (columns) -> transform to horizontal representation (rotated 90 degrees)
                if (i < columns)
                    arr_vertical[i] = part_lines
                        .Select(line => line[i] == '#')
                        .ToArray();

                i++;
            }

            patterns.Add( (arr_horizontal, arr_vertical) );
        }

        return patterns;
    }

    private static int ReflectionWithSmudge(bool[][] bools)
    {
        int matches = 0;
        int mismatches = 0;

        int row_count = bools.Length;
        int col_count = bools[0].Length;

        for (int row = 1; row < row_count; row++)
        {
            // start comparison between these two lines adjacent lines
            bool[] line_a = bools[row-1];
            bool[] line_b = bools[row];

            matches = line_a.Zip(line_b, (a, b) => a == b ? 1 : 0).Sum();
            mismatches = col_count - matches;
                
            (int up, int down) = (row-2, row+1);
            while (mismatches < 2 && up >= 0 && down < row_count)
            {
                // continue comparison as we go one line up and one line down
                line_a = bools[up];
                line_b = bools[down];

                matches = line_a.Zip(line_b, (a, b) => a == b ? 1 : 0).Sum();
                mismatches += col_count - matches;

                up--;
                down++;
            }

            // if exaclty 1 smudge, return row-amount equivalent to the amount or rows above current row
            if (mismatches == 1) return row_count - (row_count - row);
        }

        return 0;
    }
}
