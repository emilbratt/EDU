namespace AoC.Day4;

class Part2
{
    private static Dictionary<int, int[][]>? _game_cards;
    private static int _last_card_number = 0;
    public static string Run(string[] puzzle_input)
    {
        _game_cards = PrepareGameCards(puzzle_input);

        int result = CalculateGameCards();

        return result.ToString();
    }

    private static IEnumerable<int> IterIntegersFromString(string str, char f_delim, char l_delim)
    {
        // iterating over all whitepsace delimited numbers in a string
        // will only include numbers found inside the first and last delimiter inside str
        // for all numbers
        //      set first delimitter -> f_delim = ' '
        //      set last delimiter   -> l_delim = ' '

        if (f_delim != ' ') str = str.Split(f_delim)[1];
        if (l_delim != ' ') str = str.Split(l_delim)[0];

        int i = 0;
        while (i < str.Length)
        {
            string number = "";
            while (char.IsNumber(str[i]))
            {
                number += str[i].ToString();
                i++;
                if (i == str.Length) break;
            }
            if (number.Length > 0) yield return int.Parse(number);

            i++;
        }
    }

    private static Dictionary<int, int[][]> PrepareGameCards(string[] puzzle_input)
    {
        var game_cards = new Dictionary<int, int[][]>();

        int current_card_number = 0;

        int len_card_numbers = 0;
        foreach (int _ in IterIntegersFromString(puzzle_input[0], ':', '|' )) len_card_numbers++;

        int len_winning_numbers = 0;
        foreach (int _ in IterIntegersFromString(puzzle_input[0], '|', ' ' )) len_winning_numbers++;

        foreach (string line in puzzle_input)
        {
            // create a jagged array of the card numbers + the winning numbers
            int[][] numbers = new int[2][];
            numbers[0]      = new int[len_card_numbers];
            numbers[1]      = new int[len_winning_numbers];

            current_card_number = IterIntegersFromString(line, ' ', ':' ).ToArray()[0];
            numbers[0]          = IterIntegersFromString(line, ':', '|' ).ToArray();
            numbers[1]          = IterIntegersFromString(line, '|', ' ' ).ToArray();

            game_cards.Add(current_card_number, numbers);
        }

        _last_card_number = current_card_number;

        return game_cards;
    }

    private static int CalculateGameCards()
    {
        // all game cards have at elast one copy (the original)
        int[] arr_copies = Enumerable.Repeat(1, _last_card_number).ToArray();

        int last_index = arr_copies.Length - 1;
        int total_copies = 0;
        int index = 0;
        while (index <= last_index)
        {
            int card_number = index + 1; // card number is offset by +1

            int copies = arr_copies[index];

            int matches = TotalMatches(card_number);
            while (copies > 0)
            {
                total_copies++;

                int new_cards = matches;

                // starting from the highest offset (the highest card number)
                while (new_cards > 0)
                {
                    int offset_index = index + new_cards;

                    // the offset for the new copies might go out of the scope of our card count
                    if (offset_index <= last_index) arr_copies[offset_index]++;

                    new_cards--;
                }
                copies--;
            }
            index++;
        }

        return total_copies;
    }

    private static int TotalMatches(int card_number)
    {
        #pragma warning disable CS8602 // Dereference of a possibly null reference.

        int[] card_numbers = _game_cards[card_number][0];
        int[] winning_numbers = _game_cards[card_number][1];

        // using LINQ Intersect() to find intersection and get the sum of with Count()
        return card_numbers.Intersect(winning_numbers).Count();
    }
}
