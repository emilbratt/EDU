using System.Reflection;

namespace AoC.Day6;

class Part2
{
    public static string Run(string puzzle_input)
    {
        (long time, long distance) input = ParseInput(puzzle_input);

        int res = RunPuzzle(input.time, input.distance);

        return res.ToString();
    }

    public static (long time, long distance) ParseInput(string puzzle_input)
    {
        // split all content by new line (and get only the first line)
        string line_1 = puzzle_input.Split("\n")[0];
        string line_2 = puzzle_input.Split("\n")[1];

        // split each part by whitespace
        string[] numbers_line_1 = line_1.Split(' ', StringSplitOptions.RemoveEmptyEntries);
        string[] numbers_line_2 = line_2.Split(' ', StringSplitOptions.RemoveEmptyEntries);

        string time = String.Empty;
        string distance = String.Empty;

        // from the 2nd element (the first one is the description) add numbers to string
        for (int i = 1; i < numbers_line_1.Length; i++)
        {
            time += numbers_line_1[i];
            distance += numbers_line_2[i];
        }

        // return a tuple with both strings parsed into their respective number
        return (time: long.Parse(time), distance: long.Parse(distance));
    }

    public static int RunPuzzle(long time, long distance)
    {


        return CountPossibilities(time, distance);

    }

    public static int CountPossibilities(long time, long record_distance)
    {
        // starting from the half (time / 2) will always yield the best distance
        // ..this means that starting from there, we can find the number
        // which does not beat the record by incrementing +1 up or -1 down

        int count = 0;

        // start from half
        long push_time = time / 2;
        long travel = push_time * (time - push_time);

        // ..and increment downwards
        while (travel > record_distance)
        {
            count++;
            push_time--;
            travel = push_time * (time - push_time);
        }

        // start from half + 1
        push_time = (time / 2) + 1;
        travel = push_time * (time - push_time);

        // ..and increment upwards
        while (travel > record_distance)
        {
            count++;
            push_time++;
            travel = push_time * (time - push_time);
        }

        return count;
    }
}
