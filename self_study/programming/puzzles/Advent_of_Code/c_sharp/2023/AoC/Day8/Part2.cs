namespace AoC.Day8;

class Part2
{
    private static bool[] _instructions = [];
    private static Dictionary<string, NextNode> _nodes = [];

    public static string Run(string puzzle_input)
    {
        _instructions = GetLeftRightInstructions(puzzle_input);

        _nodes = GetNodes(puzzle_input);

        string[] start_nodes = GetNodesEndingWith('A');

        List<int> result_steps = [];
        foreach (string start_node in start_nodes)
        {
            result_steps.Add(TraverseNodes(start_node));
        }

        long result = LeastCommonMultiple(result_steps.ToArray());

        return result.ToString();
    }

    private static bool[] GetLeftRightInstructions(string puzzle_input)
    {
        // left = false & right = true

        char[] first_line  = puzzle_input.Split('\n')[0].ToArray();

        bool[] instructinos = new bool[first_line.Length];

        for (int index = 0; index < first_line.Length; index++)
        {
            instructinos[index] = first_line[index] == 'R'; // L -> false
        }

        return instructinos;
    }

    private static Dictionary<string, NextNode> GetNodes(string puzzle_input)
    {
        // The nodes are represented as three characters,
        // ..we cast these to their corresponding ascii values
        // then we append them as int to an array holding the 3 numbers

        Dictionary<string, NextNode> nodes = [];

        string second_part = puzzle_input.Split("\n\n")[1].ToString();
        string[] lines  = second_part.Split('\n', StringSplitOptions.RemoveEmptyEntries).ToArray();

        int ends_with_A = 0;
        int ends_with_Z = 0;

        int index = 0;
        while (index < lines.Length)
        {
            string line = lines[index];

            string str_main_node = line.Substring(startIndex: 0, length: 3);

            if (str_main_node[2] == 'A') ends_with_A++;
            if (str_main_node[2] == 'Z') ends_with_Z++;

            string str_left_node = line.Substring(startIndex:7, length: 3);
            string str_right_node = line.Substring(startIndex: 12, length: 3);

            var next_node = new NextNode(str_left_node, str_right_node);

            nodes[str_main_node] = next_node;

            index++;
        }

        // the puzzle description mentioned that these are equal
        if (ends_with_A != ends_with_Z)
        {
            Console.WriteLine("Error: count ends_with_A != count ends_with_Z");
            Environment.Exit(1);
        }

        return nodes;
    }

    private static string[] GetNodesEndingWith(char c)
    {
        List<string> nodes_ending_with = [];

        foreach (var node in _nodes.Keys)
        {
            if (node[2] == c) nodes_ending_with.Add(node);
        }

        return nodes_ending_with.ToArray();
    }

    private static int TraverseNodes(string start_node)
    {
        int steps = 0;
        int index = 0;
        bool go_right;

        string current_node = start_node;

        // the puzzle asks us to only find the first node ending with 'Z'
        while (current_node[2] != 'Z')
        {
            NextNode next_node = _nodes[current_node];

            go_right = _instructions[index];

            current_node = next_node.Get(go_right);

            steps++;

            index++;

            // reset this if out of bounds (instructions repeat from start)
            if (index == _instructions.Length) index = 0;
        }

        return steps;
    }
    private static long LeastCommonMultiple(int[] factors)
    {
        // this will calculate the lowest common multiple
        // between all numbers in an array
        // we do it efficiently by iterating over the arry,
        // then calculating the greatest common divisor
        // between a and b where a is the previous lowest common multiple (or 1)
        // and b is the current integer from the array

        // https://en.wikipedia.org/wiki/Least_common_multiple

        // start with the first number (can be any number from array, but first will do)
        long least_common_multiple = factors[0]; // ..it will only grow or stay the same from here

        for (int i = 1; i < factors.Length; i++)
        {
            // Euclidean algorithm for greatest common divisor
            // https://en.wikipedia.org/wiki/Euclidean_algorithm
            long a = least_common_multiple;
            long b = factors[i];
            bool stop_dividing = false;
            while (!stop_dividing)
            {
                if (a > b) a = a % b;
                else       b = b % a;

                stop_dividing = (a == 0 || b == 0);
            }
            // take the greatest value that holds the gcd
            long greatest_common_divisor = a + b;

            // use greatest common divisor to calculate least common multiple for two numbers
            least_common_multiple = least_common_multiple * factors[i] / greatest_common_divisor;
        }

        // this will be the final least common multiple
        return least_common_multiple;
    }

    // the datatype annotation grew so big, lets introduce a class
    private class NextNode
    {
        private readonly string _left;
        private readonly string _right;

        public NextNode(string left_characters, string right_characters)
        {
            _left = left_characters;
            _right = right_characters;
        }

        public string Get(bool go_right)
        {
            return go_right ? _right : _left;
        }
    }
}
