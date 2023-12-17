namespace AoC.Day7;

class Part1
{
    public static string Run(string puzzle_input)
    {
        (string cards, int bid)[] camel_cards = GetCamelCards(puzzle_input);

        var filtered_camel_cards = FilterAllHands(camel_cards);

        int result = CalculateTotalWinnings(filtered_camel_cards);

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
            int type = CamelCardLib.GetTypeRank(hand);
            dict_out[type].Add((hand, bid));
        }

        return dict_out;
    }

    public static int CalculateTotalWinnings(Dictionary<int, List<(string hand, int bid)>> cards)
    {
        for (int type = 0; type <= 6; type++)
        {
            List<(string hand, int bid)> l = cards[type];
            Console.WriteLine($"\nType: {CamelCardLib.TypeAlias(type)}");

            int i = 0;
            while (i < l.Count)
            {
                string hand = l[i].hand;
                int[] arr = new int[5];
                string c_arr = String.Empty;
                for (int j = 0; j < 5; j++)
                {
                    arr[j] = CamelCardLib.CardValue(hand[j]);
                    c_arr += $"{arr[j], 2} ";
                }
                int bid = l[i].bid;
                i++;
                Console.WriteLine($" - Nr. {i, 3} | Hand: {hand} | Values: {c_arr} | Bid: {bid, 3}");
            }
        }
        return 0;
    }
}

class CamelCardLib
{
    public static int GetTypeRank(string hand)
    {
        // start of with an array with 13 elements, each element represent a card
        int[] cards = new int[13]; // 13 different cards: Ace, 2, 3,.., King

        foreach (char c in hand)
        {
            // we start from 0 -> because we use this as array index
            int index = c switch
            {
                '2' => 0,
                '3' => 1,
                '4' => 2,
                '5' => 3,
                '6' => 4,
                '7' => 5,
                '8' => 6,
                '9' => 7,
                'T' => 8,
                'J' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
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

    public static string TypeAlias(int type)
    {
        return type switch
        {
            6 => "Five of a kind (6)",
            5 => "Four of a kind (5)",
            4 => "Full house (4)",
            3 => "Three of a kind (3)",
            2 => "Two pair (2)",
            1 => "One pair (1)",
            0 => "High card (0)",
            _ => "Not a valid rank (ERROR)",
        };
    }

    public static int CardValue(char c)
    {
        return c switch
        {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0,
        };
    }

    public static string StringCardValue(char c)
    {
        return c switch
        {
            '2' => " 2",
            '3' => " 3",
            '4' => " 4",
            '5' => " 5",
            '6' => " 6",
            '7' => " 7",
            '8' => " 8",
            '9' => " 9",
            'T' => "10",
            'J' => "11",
            'Q' => "12",
            'K' => "13",
            'A' => "14",
            _ => "00",
        };
    }
}
