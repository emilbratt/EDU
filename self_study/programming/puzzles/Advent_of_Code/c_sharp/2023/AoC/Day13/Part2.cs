namespace AoC.Day13;

class Part2
{
    public static string Run(string puzzle_input)
    {
        // LINQ expression for doing it all in one step
        return GetMirrorPatterns(puzzle_input)
            .Select(ReflectionWithSmudge) // pass in method for all elemetns in "GetMirrorPatterns(puzzle_input)"
            .Sum() // summarize
            .ToString(); // stringify for correct output type
    }

    // return two versions where the vertical is rotated 90 degrees so we can use same implementations for it
    private static List<bool[][]> GetMirrorPatterns(string puzzle_input)
    {
        List<bool[][]> patterns = [];

        string[] parts = puzzle_input.Split("\n\n", StringSplitOptions.RemoveEmptyEntries);

        foreach (string part in parts)
        {
            string[] part_lines = part.Split('\n', StringSplitOptions.RemoveEmptyEntries);

            // foreach string => foreach char => make array and set true or false => boolean iagged array
            bool[][] arr_horizontal = part_lines
                .Select(line =>
                    line.Select(character => character == '#')
                    .ToArray()
                )
                .ToArray();

            patterns.Add(arr_horizontal);
        }

        return patterns;
    }

    private static int ReflectionWithSmudge(bool[][] mirrors)
    {
        int row_count = mirrors.Length;
        int col_count = mirrors[0].Length;

        // check for horizontal reflection
        for (int row = 1; row < row_count; row++)
        {
            int matches = 0;
            int mismatches = 0;
            int i = 0;

            while (row - i - 1 >= 0 && row + i < row_count && mismatches <= 1)
            {
                bool[] line_a = mirrors[row - i - 1];
                bool[] line_b = mirrors[row + i];

                matches = line_a
                    .Zip(line_b, (a, b) => a == b ? 1 : 0)
                    .Sum();

                mismatches += col_count - matches;

                i++;
            }

            // if exaclty 1 smudge, return row-amount equivalent to the amount of rows above current row
            if (mismatches == 1) return 100 * (row_count - (row_count - row));
        }

        // if no horizontal reflection, check for vertical reflection
        for (int col = 1; col < col_count; col++)
        {
            int matches = 0;
            int mismatches = 0;
            int i = 0;

            while (col - i - 1 >= 0 && col + i < col_count && mismatches <= 1)
            {
                // iterating over all rows and selecting every indexed column 
                bool[] line_a = mirrors
                    .Select(line => line[col - i - 1])
                    .ToArray();

                bool[] line_b = mirrors
                    .Select(line => line[col + i])
                    .ToArray();

                matches = line_a
                    .Zip(line_b, (a, b) => a == b ? 1 : 0)
                    .Sum();

                mismatches += row_count - matches;

                i++;
            }

            // if exaclty 1 smudge, return col-amount equivalent to the amount of columns left for current column
            if (mismatches == 1) return col_count - (col_count - col);
        }

        return 0;
    }
}
