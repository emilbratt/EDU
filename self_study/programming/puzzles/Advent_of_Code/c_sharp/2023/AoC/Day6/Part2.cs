namespace AoC.Day6;

class Part2
{
    public static string Run(string puzzle_input)
    {
        (long time, long distance) input = ParseInput(puzzle_input);

        long res = RunPuzzle(input.time, input.distance);

        return res.ToString();
    }

    public static (long time, long distance) ParseInput(string puzzle_input)
    {
        // split all content by new line (and get only the first line)
        string the_time = puzzle_input.Split("\n")[0];
        // and the second line
        string the_distance = puzzle_input.Split("\n")[1];

        // split each part by whitespace
        string[] arr_the_time = the_time.Split(' ', StringSplitOptions.RemoveEmptyEntries);
        string[] arr_the_distance = the_distance.Split(' ', StringSplitOptions.RemoveEmptyEntries);

        string time = String.Empty;
        string distance = String.Empty;
        // from the 2nd element (the first one is the description) apply digits to corresponding string
        for (int i = 1; i < arr_the_time.Length; i++)
        {
            time += arr_the_time[i];
            distance += arr_the_distance[i];
        }

        // return a tuple with both strings parsed into their respective number
        return (time: long.Parse(time), distance: long.Parse(distance));
    }

    public static long RunPuzzle(long time, long record_distance)
    {
        // using binary search, find lowest push time while still beating the record distance
        long lo = 0;
        long hi = time / 2;
        while (lo <= hi)
        {
            long middle = (lo + hi) / 2;

            long push_time = middle / 2;
            long travel = push_time * (time - push_time);

            if (travel > record_distance)
            {
                hi = middle - 1;
            }
            else if (travel <= record_distance)
            {
                lo = middle + 1;
            } 
        }

        return time - hi;
    }
}
