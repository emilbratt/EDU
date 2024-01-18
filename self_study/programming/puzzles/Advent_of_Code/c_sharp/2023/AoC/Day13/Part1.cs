namespace AoC.Day13;

class Part1
{
    public static string Run(string puzzle_input)
    {
        var mirror_patterns = GetMirrorPatterns(puzzle_input);

        int res = 0;

        foreach ((int[] horizontal, int[] vertical_rotated) in mirror_patterns)
        {
            int reflection = CalculateReflection(horizontal) * 100;

            // no horizontal reflection, lets try vertical instead
            if (reflection == 0) reflection = CalculateReflection(vertical_rotated);

            res += reflection;
        }

        return res.ToString();
    }

    // return two versions where the vertical is rotated 90 degrees so we can use same implementations for it
    private static List<(int[] horizontal, int[] vertical_rotated)> GetMirrorPatterns(string puzzle_input)
    {
        /*
         * parts look like this
         *
         * .#...##..
         * #.#######
         * ....####.
         * ..##.##.#
         * ..##.##.#
         * ....####.
         * #.#######
         *
         * ..we convert each line to the binary representation
         *
         * The first line ".#...##.." would give us "010001100"
         * We convert that to its int32 representation giving us 140
         * That way we preserve and process every line using less data and less compute.
         */

        List<(int[] horizontal, int[] vertical)> patterns = [];

        string[] parts = puzzle_input.Split("\n\n", StringSplitOptions.RemoveEmptyEntries);

        foreach (string part in parts)
        {
            string[] part_lines = part.Split('\n', StringSplitOptions.RemoveEmptyEntries);

            int rows = part_lines.Length;
            int columns = part_lines[0].Length;

            int[] arr_horizontal = new int[rows];
            int[] arr_vertical = new int[columns];

            string base_two = String.Empty;

            int i = 0; // we can use i for both rows and columns
            while (i < rows || i < columns)
            {
                // create binary repr. of the line converting '#' or '.' to '1' or '0' respectively

                // horizontal iteration (rows) -> extract and keep as is
                if (i < rows)
                {
                    base_two = string.Concat(part_lines[i].Select(character => character == '#' ? '1' : '0' ));

                    arr_horizontal[i] = Convert.ToInt32(base_two, 2);
                }

                // vertical iteration (columns) -> reformat to horizontal representation (rotated 90 degrees)
                if (i < columns)
                {
                    base_two = string.Concat(part_lines.Select(line => line[i] == '#' ? '1' : '0'));

                    arr_vertical[i] = Convert.ToInt32(base_two, 2);
                }

                i++;
            }

            patterns.Add( (arr_horizontal, arr_vertical) );
        }

        return patterns;
    }

    // works for both horizontal and vertical because the vertical array is rotated
    private static int CalculateReflection(int[] mirrror_numbers)
    {
        for (int i = 1; i < mirrror_numbers.Length; i++)
        {
            bool reflection = mirrror_numbers[i - 1] == mirrror_numbers[i];
            int j = 0;
            while (i - j - 1 >= 0 && i + j < mirrror_numbers.Length && reflection)
            {
                // expand left + right and compare numbers, breaks out if not reflection
                reflection = mirrror_numbers[i - j - 1] == mirrror_numbers[i + j];

                j++;
            }

            // if we reached the first or last number while still reflection, success!
            if (reflection) return mirrror_numbers.Length - (mirrror_numbers.Length - i);
        }

        return 0;
    }
}
