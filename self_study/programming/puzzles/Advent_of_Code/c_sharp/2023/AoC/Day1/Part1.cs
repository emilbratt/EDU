namespace AoC.Day1;

class Part1
{
    public static string Run(string puzzle_input)
    {
        var list = new List<string>();
        foreach (string line in puzzle_input.Split('\n'))
        {
            list.Add(line);
        }
        list.RemoveAt(list.Count - 1);

        string[] input = list.ToArray();

        string[] number_list = new string[input.Length];

        for (int line = 0; line < input.Length; line++)
        {
            string s = input[line];

            int? first_digit = FirstDigitFromString(s);
            int? last_digit = LastDigitFromString(s);

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

    private static int? FirstDigitFromString(string s)
    {
        for (int i = 0; i < s.Length; i++)
        {
            char c = s[i];
            if (char.IsNumber(c))
            {
                int digit = c - '0';
                return digit;
            }
        }

        return null;
    }

    private static int? LastDigitFromString(string s)
    {
        for (int i = s.Length-1; i >= 0; i--)
        {
            char c = s[i];
            if (char.IsNumber(c))
            {
                int digit = c - '0';
                return digit;
            }
        }

        return null;
    }
}
