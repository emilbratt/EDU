using System.Data;

namespace AoC.Day9;

class Part1
{
    public static string Run(string puzzle_input)
    {
        int[][] oasis_report = GetOasisReport(puzzle_input);

        int result = 0;

        foreach (int[] row in oasis_report)
        {
            result += ExtrapolateForward(row);
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

    public static int ExtrapolateForward(int[] row)
    {
        int extrapolation = 0;
        int last_index = row.Length - 1;

        bool only_zeroes = false;
        while (!only_zeroes)
        {
            extrapolation += row[last_index];

            only_zeroes = true;
            for (int index = 0; index < last_index; index++)
            {
                int subtracted = row[index + 1] - row[index];

                if (subtracted != 0) only_zeroes = false;

                row[index] = subtracted;
            }

            last_index--;
        }

        return extrapolation;
    }
}
