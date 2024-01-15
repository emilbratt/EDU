namespace AoC.Day13;

class Part1
{
    public static string Run(string puzzle_input)
    {
        int res = 0;
        var mp = GetMirrorPatterns(puzzle_input);

        foreach ((int[] horizontal, int[] rotated_vertical) in mp)
        {
            int res_horizontal = CalculateReflection(horizontal);
            int res_vertical = CalculateReflection(rotated_vertical);

            res += 100*res_horizontal + res_vertical;
        }

        return res.ToString();
    }

    // return two versions where the vertical is rotated 90 degrees so we can use same implementations for it
    private static List<(int[] horizontal, int[] vertical)> GetMirrorPatterns(string puzzle_input)
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

            int i = 0; // we can use i for both rows and columns
            while (i < rows || i < columns)
            {
                // horizontal iteration (rows) -> extract and keep as is
                if (i < rows)
                {
                    string line = part_lines[i];

                    // create binary repr. of the line converting '#' or '.' to '1' or '0' respectively
                    string base_two = string.Concat(line.Select(s => s == '#' ? '1' : '0' ));

                    // convert our binary to an integer and add to array
                    arr_horizontal[i] = Convert.ToInt32(base_two.ToString(), 2);
                }

                // vertical iteration (columns) -> transform to horizontal representation (rotated 90 degrees)
                if (i < columns)
                {
                    string base_two = string.Concat(part_lines.Select(line => line[i] == '#' ? '1' : '0'));

                    // convert our binary to an integer and add to array
                    arr_vertical[i] = Convert.ToInt32(base_two, 2);
                }

                i++;
            }

            patterns.Add( (arr_horizontal, arr_vertical) );
        }

        return patterns;
    }

    // works for both horizontal and vertical because the vertical array is rotated
    private static int CalculateReflection(int[] numbers)
    {
        int res = 0;
        int i = 1;
        bool reflection = false;
        while (i < numbers.Length && !reflection)
        {
            res = 0; // reset on every iteration (we might have had increase during semi-valid reflection)

            // found reflection
            if (numbers[i-1] == numbers[i])
            {
                reflection = true;
                int k = 0;
                while (i-k-1 >= 0 && i+k < numbers.Length && reflection)
                {
                    // expand left and right
                    (int left, int right) = (i-k-1, i+k);

                    reflection = numbers[left] == numbers[right];
                    res++;
                    k++;
                }
                res += i-k;
            }

            i++;
        }

        return res;
    }
}
