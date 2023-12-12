namespace AoC.Day6;

class Part1
{
    public static string Run(string puzzle_input)
    {
        (int time, int distance)[] input = ParseInput(puzzle_input);

        int res = RunPuzzle(input);

        return res.ToString();
    }

    public static (int time, int distance)[] ParseInput(string puzzle_input)
    {
        // split all content by new line (and get only the first line)
        string line_1 = puzzle_input.Split("\n")[0];
        string line_2 = puzzle_input.Split("\n")[1];

        // split each part by whitespace
        string[] numbers_line_1 = line_1.Split(' ', StringSplitOptions.RemoveEmptyEntries);
        string[] numbers_line_2 = line_2.Split(' ', StringSplitOptions.RemoveEmptyEntries);

        // parse all numbers (skipping first element)
        int[] int_times = numbers_line_1.Skip(1).Select(int.Parse).ToArray();
        int[] int_distances = numbers_line_2.Skip(1).Select(int.Parse).ToArray();

        var parsed_input = new (int time, int distance)[int_times.Length];
        for (int i = 0; i < int_times.Length; i++)
        {
            parsed_input[i].time = int_times[i];
            parsed_input[i].distance = int_distances[i];
        }

        return parsed_input;
    }

    public static int RunPuzzle((int time, int distance)[] input)
    {
        int[] numbers = new int[input.Length];

        for (int i = 0; i < input.Length; i++)
        {
            int time = input[i].time;
            int distance = input[i].distance;
            numbers[i] = CountPossibilities(time, distance);
        }

        // LINQ: using Aggregate to multiply numbers in array
        int product = numbers.Aggregate((current, next) => current * next);
        return product;
    }

    public static int CountPossibilities(int time, int record_distance)
    {
        // starting from the half (time / 2) will always yield the best distance
        // ..this means that starting from there, we can find the number
        // which does not beat the record by incrementing +1 up or -1 down

        int count = 0;

        // start from half
        int push_time = time / 2;
        int travel = push_time * (time - push_time);

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
