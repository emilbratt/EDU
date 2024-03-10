namespace AoC.Day6;

class Part2
{
    public static string Run(string puzzle_input)
    {
        (long time, long distance) = ParseInput(puzzle_input);

        long res = BinarySearchPossibleWinConditions(time, distance);

        return res.ToString();
    }

    private static (long time, long distance) ParseInput(string puzzle_input)
    {
        // split all content by new line (and get only the first line)
        string time = puzzle_input.Split("\n")[0];
        // and the second line
        string distance = puzzle_input.Split("\n")[1];

        // split each part by whitespace
        string[] time_numbers = time.Split(' ', StringSplitOptions.RemoveEmptyEntries);
        string[] distance_numbers = distance.Split(' ', StringSplitOptions.RemoveEmptyEntries);

        time = String.Empty;
        distance = String.Empty;
        // from the 2nd element (the first one is the description) apply digits to corresponding string
        for (int i = 1; i < time_numbers.Length; i++)
        {
            time += time_numbers[i];
            distance += distance_numbers[i];
        }

        // return a tuple with both strings parsed into their respective number
        return (time: long.Parse(time), distance: long.Parse(distance));
    }

    private static long BinarySearchPossibleWinConditions(long time, long record_distance)
    {
        // Dividing time by 2 yields best pushtime for longest distance.

        // Binary search - find lowest push time while still beating the record distance.
        long lo = 0;
        long hi = time / 2; // Start from middle instead of top, we want to approach lower end..

        while (lo <= hi)
        {
            long middle = (lo + hi) / 2;

            long push_time = middle / 2;
            long travel = push_time * (time - push_time);

            if (travel > record_distance) hi = middle - 1;
            else if (travel <= record_distance) lo = middle + 1;
        }

        long ans = time - hi;

        return ans;
    }
}
