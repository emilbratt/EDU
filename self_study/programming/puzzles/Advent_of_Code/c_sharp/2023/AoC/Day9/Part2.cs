namespace AoC.Day9;

class Part2
{
    public static string Run(string puzzle_input)
    {
        int[][] oasis_report = GetOasisReport(puzzle_input);

        int result = 0;
        foreach (int[] row in oasis_report)
        {
            result += ExtrapolateBackwards(row);
        }

        return result.ToString();
    }

    public static int[][] GetOasisReport(string puzzle_input)
    {
        var list = new List<int[]>();

        string[] lines = puzzle_input.Split("\n", StringSplitOptions.RemoveEmptyEntries);

        foreach (string line in lines)
        {
            string[] numbers = line.Split(' ');
            int[] int_numbers = numbers.Select(int.Parse).ToArray();
            list.Add(int_numbers);
        }

        return list.ToArray();
    }

    public static int ExtrapolateBackwards(int[] row)
    {
        List<int> first_numbers = new();
        int last_index = row.Length - 1;

        bool only_zeroes = false;
        while (!only_zeroes)
        {
            first_numbers.Add(row[0]);

            only_zeroes = true;
            for (int index = 0; index < last_index; index++)
            {
                int subtracted = row[index + 1] - row[index];

                if (subtracted != 0) only_zeroes = false;

                row[index] = subtracted;
            }

            last_index--;
        }

        int extrapolation = 0;
        for (int i = first_numbers.Count - 1; i >= 0; i--)
        {
            extrapolation = first_numbers[i] - extrapolation;
        }

        return extrapolation;
    }
}
