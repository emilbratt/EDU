using System.Data;

namespace AoC.Day9;

class Part1
{
    public static string Run(string puzzle_input)
    {
        int[][] oasis_report = GetOasisReport(puzzle_input);

        int result = 0;

        foreach (int[] reading in oasis_report)
        {
            result += Extrapolate(reading);
        }


        return result.ToString();
    }

    public static int[][] GetOasisReport(string puzzle_input)
    {
        var list = new List<int[]>();

        string[] lines = puzzle_input.Split(
            new string[] { "\r\n", "\r", "\n" },
            StringSplitOptions.RemoveEmptyEntries
        );

        foreach (string line in lines)
        {
            string[] numbers = line.Split(' ');
            int[] int_numbers = numbers.Select(int.Parse).ToArray();
            list.Add(int_numbers);
        }

        return list.ToArray();
    }

    public static int Extrapolate(int[] reading)
    {
        int extrapolation = 0;
        int last_index = reading.Length -1;

        bool all_not_zero = true;
        while (all_not_zero)
        {
            extrapolation += reading[last_index];

            all_not_zero = false;
            for (int index = 0; index < last_index; index++)
            {
                int subtracted = reading[index + 1] - reading[index];

                if (subtracted != 0) all_not_zero = true;

                reading[index] = subtracted;
            }

            last_index--;
        }

        return extrapolation;
    }
}
