namespace AoC.Day4;

class Part1
{
    public static string Run(string[] puzzle_input)
    {
        int result = HandlePuzzleInput(puzzle_input);
        return result.ToString();
    }

    private static IEnumerable<int> IterStringNumbers(string str, char first_delimiter, char last_delimiter)
    {
        // iterating over all numbers from a string 
        // ..which are delimited by whitespace
        // ..and wrapped inside a first and last delimiter

        bool yield_value = false;
        int i = 0;
        while (i < str.Length)
        {
            if (str[i] == first_delimiter) yield_value = true;
            if (yield_value && str[i] == last_delimiter && last_delimiter != ' ') yield_value = false;

            string number = "";
            while (char.IsNumber(str[i]) && yield_value)
            {
                number += str[i].ToString();
                i++;
                if (i == str.Length) break;
            }
            if (number.Length > 0 && yield_value) yield return int.Parse(number);

            i++;
        }
    }

    private static int HandlePuzzleInput(string[] puzzle_input)
    {
        int total = 0;

        foreach (string line in puzzle_input)
        {
            // a binary doubles for each 0 added, lets take advantage of that
            string binary = String.Empty;
            bool awaiting_first = true;

            foreach (int card_number in IterStringNumbers(line, ':', '|'))
            {
                foreach (int winning_number in IterStringNumbers(line, '|', ' '))
                {
                    if (card_number == winning_number)
                    {
                        binary += awaiting_first ? "1" : "0"; // first digit must be 1, though
                        awaiting_first = false;
                    }
                }

            }
            if (!awaiting_first) total += Convert.ToInt32(binary, 2);
        }

        return total;
    }
}
