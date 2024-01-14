namespace AoC.Day12;

class Part1
{
    public static string Run(string puzzle_input)
    {
        (char[] symbols, int[] numbers)[] records = SpringRecordings(puzzle_input);

        long res = 0;
        foreach (var (symbols, numbers) in records)
        {
            res += RecCA(0, 0, 0, symbols, numbers);
        }

        return res.ToString();
    }

    private static (char[] symbols, int[] numbers)[] SpringRecordings(string input)
    {
        string[] lines = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        var condition_records = new (char[] symbols, int[] numbers)[lines.Length];

        int i = 0;
        while (i < lines.Length)
        {
            string[] parts = lines[i].Split(); // "???.### 1,1,3" -> [ "???.###" "1,1,3" ]

            char[] symbols = parts[0].ToCharArray();
            int[] numbers = Array.ConvertAll(parts[1].Split(','), int.Parse);

            condition_records[i] = (symbols, numbers);

            i++;
        }

        return condition_records;
    }

    // RecCA - Recursive Calculate Arrangements
    private static long RecCA(int k, int i, int j, char[] symbols, int[] numbers)
    {
        /*
         *
         *  state of computation:
         *      k = how many '#' accumulated so far
         *      i = index symbols ('.', '#', '?)
         *      j = index numbers
         *
         */

        bool last_number = j == numbers.Length - 1;
        bool past_last_number = j == numbers.Length;
        bool accumulation_match = !past_last_number && numbers[j] == k;
        bool past_last_symbol = i == symbols.Length;
        bool no_accumulation = k == 0;

        // base case for when we have processed all symbols in the char array
        if (past_last_symbol)
        {
            if (past_last_symbol && past_last_number && no_accumulation) return 1;
            else if (past_last_symbol && last_number && accumulation_match) return 1;
            else return 0;
        }


        (long res, int k_up, int i_up, int j_up) = (0, k+1, i+1, j+1);
        char symbol = symbols[i];
        if (symbol == '?' || symbol == '.')
        {
            if (no_accumulation)
            {
                res += RecCA(0, i_up, j, symbols, numbers);
            } 
            else if (!past_last_number && accumulation_match)
            {
                res += RecCA(0, i_up, j_up, symbols, numbers);
            }
        }

        if (symbol == '#' || symbol == '?')
        {
            res += RecCA(k_up, i_up, j, symbols, numbers);
        }

        return res;
    }
}
