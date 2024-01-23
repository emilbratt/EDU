namespace AoC.Day15;

class Part1
{
    public static string Run(string puzzle_input)
    {
        string[] init_sequence = GetInitializationSequence(puzzle_input);

        int res = 0;
        foreach (string part in init_sequence) res += HASH(part);
        return res.ToString();
    }

    private static string[] GetInitializationSequence(string puzzle_input)
    {
        string first_line = puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries)[0];
        string[] init_sequence = first_line.Split(',', StringSplitOptions.RemoveEmptyEntries);
        return init_sequence;
    }

    private static int HASH(string s)
    {
        int result = 0;

        foreach (char c in s)
        {
            int ascii_code = c;   // 1. Determine the ASCII code for the current character of the string.
            result += ascii_code; // 2. Increase the current value by the ASCII code you just determined.
            result *= 17;         // 3. Set the current value to itself multiplied by 17.
            result %= 256;        // 4. Set the current value to the remainder of dividing itself by 256.
        }

        return result;
    }
}
