namespace AoC.Day12;

class Part1
{
    public static string Run(string puzzle_input)
    {
        (char[] symbols, int[] numbers)[] records = GetConditionRecords(puzzle_input);

        long res = 0;
        foreach (var (symbols, numbers) in records)
        {
            res += CalculateArrangements(symbols, numbers);
        }

        return res.ToString();
    }

    private static (char[] symbols, int[] numbers)[] GetConditionRecords(string input)
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

    private static long CalculateArrangements(char[] symbols, int[] numbers)
    {

        // RecCA - Recursive Calculate Arrangements
        long RecCA(int k, int i, int j)
        {
            /*
            *
            *  state of computation:
            *      k = current length of the string of hashtags to be compared against
            *      i = current symbol '.', '# or '?
            *      j = current number
            *
            */

            bool no_symbols_left = i == symbols.Length;
            bool no_numbers_left = j == numbers.Length;
            bool numbers_remaining = j < numbers.Length;
            bool is_last_number = j == numbers.Length - 1;
            bool no_hashtags = k == 0;
            bool hashtags_length_matches_number = numbers_remaining && numbers[j] == k;

            // base case for when we have processed all symbols in the record
            if (no_symbols_left)
            {
                if (no_numbers_left && no_hashtags) return 1;
                else if (is_last_number && hashtags_length_matches_number) return 1;
                else return 0;
            }

            char s = symbols[i];

            (int next_K, int next_symbol, int next_number) = (k+1, i+1, j+1);
            (int zero_K, int same_number) = (0, j);

            (bool questionmark, bool dot, bool hashtag) = (s == '?', s == '.', s == '#');

            if (hashtag)
            {
                return RecCA(next_K, next_symbol, same_number);
            }

            long res = 0;

            if (questionmark)
            {
                res += RecCA(next_K, next_symbol, same_number);
            }

            if (hashtags_length_matches_number)
            {
                res += RecCA(zero_K, next_symbol, next_number);
            }
            else if (no_hashtags)
            {
                res += RecCA(zero_K, next_symbol, same_number);
            }



            return res;
        }

        return RecCA(0, 0, 0);
    }

}
