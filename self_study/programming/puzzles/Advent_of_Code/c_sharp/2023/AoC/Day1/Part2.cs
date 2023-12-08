namespace AoC.Day1;

class Part2
{
    private static readonly string[] _spelled_digits = new string[]
    {
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    };

    public static string Run(string[] puzzle_input)
    {
        string[] number_list = new string[puzzle_input.Length];

        for (int line = 0; line < puzzle_input.Length; line++)
        {
            string s = puzzle_input[line];

            int? first_digit = FirstNumericDigitOrSpelledDigitFromString(s);
            int? last_digit = LastNumericDigitOrSpelledDigitFromString(s);

            if (first_digit == null || last_digit == null) Environment.Exit(1);

            number_list[line] = first_digit.ToString() + last_digit.ToString();
        }

        int total = 0;

        foreach (string s in number_list)
        {
            bool is_numeric = int.TryParse(s, out int number);
            if (is_numeric) total += number;
        }

        return total.ToString();
    }

    private static int? FirstNumericDigitOrSpelledDigitFromString(string s)
    {
        for (int i = 0; i < s.Length; i++)
        {
            char c = s[i];
            if (char.IsNumber(c))
            {
                int digit = c - '0';
                return digit;
            }

            foreach (string spelled in _spelled_digits)
            {
                int last_index = i + spelled.Length;

                if (last_index > s.Length) continue;

                if (s.Substring(i, spelled.Length) != spelled) continue;
                
                return spelled switch
                {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => null,
                };
            }
        }

        return null;
    }

    private static int? LastNumericDigitOrSpelledDigitFromString(string s)
    {
        for (int i = s.Length; i >= 0; i--)
        {
            char c = s[i-1];
            if (char.IsNumber(c))
            {
                int digit = c - '0';
                return digit;
            }

            foreach (string spelled in _spelled_digits)
            {
                int first_index = i - spelled.Length;

                if (first_index < 0) continue;

                if (s.Substring(first_index, spelled.Length) != spelled) continue;

                return spelled switch
                {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => null,
                };
            }
        }

        return null;
    }
}
