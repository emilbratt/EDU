namespace AoC.Day15;

class Part2
{
    public static string Run(string puzzle_input)
    {
        string[] init_sequence = GetInitializationSequence(puzzle_input);

        var boxes = new Box[256];

        for (int i = 0; i < 256; i++)
        {
            boxes[i] = new Box(i);
        }

        int res = HASHMAP(init_sequence, boxes);

        return res.ToString();
    }

    private static string[] GetInitializationSequence(string puzzle_input)
    {
        string first_line = puzzle_input.Split('\n')[0];
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

    private static int HASHMAP(string[] init_sequence, Box[] boxes)
    {
        foreach (string s in init_sequence)
        {
            if (s.Contains('='))
            {
                string[] split = s.Split('=');

                string label = split[0];
                int focal_length = int.Parse(split[1]);

                int box_id = HASH(label);

                boxes[box_id].AddLense( (label, focal_length) );
            }
            else if (s.Contains('-'))
            {
                string label = s.Split('-')[0];

                int box_id = HASH(label);

                boxes[box_id].RemoveLense(label);
            }
        }

        int res = 0;

        for (int i = 0; i < 256; i++)
        {
            res += boxes[i].SumFocusingPower();
        }

        return res;
    }

    private class Box(int box_number)
    {
        private readonly int _box_number = box_number;

        private readonly List<(string label, int focal_length)> _slots = [];

        public void AddLense((string label, int focal_length) lense)
        {
            // If lens exists with same label, then replace it with the new lens
            // Else add lens to last slot int the box

            for (int i = 0; i < _slots.Count; i++)
            {
                if (_slots[i].label == lense.label)
                {
                    _slots[i] = lense;
                    return;
                }
            }

            _slots.Add(lense);
        }

        public void RemoveLense(string label)
        {
            int remove_index = -1;

            for (int i = 0; i < _slots.Count; i++)
            {
                if (_slots[i].label == label) remove_index = i;
            }

            if (remove_index != -1) _slots.RemoveAt(remove_index);
        }

        public int SumFocusingPower()
        {
            /*
                The focusing power of a single lens is the result of multiplying together:
                    One plus the box number of the lens in question.
                    The slot number of the lens within the box
                    The focal length of the lens.
            */
            int res = 0;

            int slot_number = 1;
            foreach ( var (label, focal_length) in _slots)
            {
                int focusing_power = (1+_box_number) * slot_number * focal_length;

                res += focusing_power;

                slot_number++;
            }

            return res;
        }
    }
}
