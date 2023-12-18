namespace AoC.Day7;

class Part1
{
    public static string Run(string puzzle_input)
    {
        (string cards, int bid)[] camel_cards = GetCamelCards(puzzle_input);

        var filtered_camel_cards = FilterAllHands(camel_cards);

        long result = CalculateTotalWinnings(filtered_camel_cards);

        return result.ToString();
    }

    public static (string cards, int bid)[] GetCamelCards(string puzzle_input)
    {
        var list_out = new List<(string, int)>();

        string[] lines = puzzle_input.Split("\n");

        foreach (string line in lines)
        {
            if (line.Length == 0) continue; // we might have a new line ending

            string[] arr_line = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);

            list_out.Add((arr_line[0], int.Parse(arr_line[1])));
        }

        return list_out.ToArray();
    }

    public static Dictionary<int, List<(string hand, int bid)>> FilterAllHands((string cards, int bid)[] camel_cards)
    {
        var dict_out = new Dictionary<int, List<(string, int)>>();

        // create each type as key - we rank the types from 0 - 6 where 6 is the best type
        for (int key = 0; key <= 6; key++)
        {
            dict_out.Add(key, new List<(string, int)>());
        }

        // populate where key matches the type
        foreach (var tuple in camel_cards)
        {
            string hand = tuple.cards;
            int bid = tuple.bid;
            int type = GetTypeRank(hand);
            var new_tuple = (hand, bid);

            dict_out[type].Add(new_tuple);
        }

        return dict_out;
    }

    public static long CalculateTotalWinnings(Dictionary<int, List<(string hand, int bid)>> cards)
    {
        long total_winnings = 0;

        int rank = 1;
        for (int type = 0; type <= 6; type++)
        {
            var arr = cards[type].ToArray();

            (int strength, int bid)[] arr_strength = GetCardStrengthForHands(arr);

            arr_strength = SortLowestFirst(arr_strength);

            for (int j = 0; j < arr_strength.Length; j++)
            {
                int bid = arr_strength[j].bid;

                total_winnings += rank * bid;

                rank++;
            }
        }

        return total_winnings;
    }

    public static int GetTypeRank(string hand)
    {
        // start of with an array with 13 elements, each element represent a card
        int[] cards = new int[13]; // 13 different cards: Ace, 2, 3,.., King

        foreach (char c in hand)
        {
            // we start from 0 -> because we use this as array index
            int index = c switch
            {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 9,
                'T' => 8,
                '9' => 7,
                '8' => 6,
                '7' => 5,
                '6' => 4,
                '5' => 3,
                '4' => 2,
                '3' => 1,
                '2' => 0,
                _ => -1, // will force some error if this gets evaluated
            };

            // add +1 to the element in this index
            cards[index]++;
        }

        // sorting the array (and reversing it) will give us the count we want on index 0 and 1
        Array.Sort(cards);
        Array.Reverse(cards);

        int largest_number = cards[0];
        int next_largest_number =  cards[1];

        return (largest_number == 5) ? 6 // five of a kind
             : (largest_number == 4) ? 5 // four of a kind
             : (largest_number == 3 && next_largest_number == 2) ? 4 // full house
             : (largest_number == 3) ? 3 // three of a kind
             : (largest_number == 2 && next_largest_number == 2) ? 2 // two pairs
             : (largest_number == 2 ) ? 1 // one pair
             : 0; // High card
    }

    public static (int strength, int bid)[] GetCardStrengthForHands((string hand, int bid)[] cards)
    {
        /*
         *  From first to last card, get a value representing the strength
         *  This does not take into account the "type" of hand (two pairs or 4 of a kind etc..)
         *  It will only give you the product of each card multiplied with the position
         *  where the first position = 5 counting down to the last = 1.
         */

        var new_arr = new (int strength, int bid)[cards.Length];

        int i = 0;
        while (i < cards.Length)
        {
            int strength = GetStrength(cards[i].hand);
            new_arr[i] = (strength, cards[i].bid);
            i++;
        }

        return new_arr;
    }

    public static int GetStrength(string hand)
    {
        int strength = 0;

        int index = 0;
        while (index < hand.Length)
        {
            int card_value = hand[index] switch
            {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _ => 0,
            };

            int round = hand.Length;
            while (round > index)
            {
                card_value *= 13;
                round--;
            }

            strength += card_value;
            index++;
        }
        return strength;
    }

    public static (int strength, int bid)[] SortLowestFirst((int strength, int bid)[] cards)
    {
        // simple bubble sort implementation to order the strength from lowest

        int n = cards.Length;

        for (int i = 0; i < n; i++)
        {
            for (int j = 0; j < n - i - 1; j++)
            {
                var a = cards[j];
                var b = cards[j+1];

                if (a.strength > b.strength)
                {
                    cards[j] = b;
                    cards[j+1] = a;
                }
            }
        }

        return cards;
    }
}
