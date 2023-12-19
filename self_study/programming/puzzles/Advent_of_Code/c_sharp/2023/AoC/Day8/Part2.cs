namespace AoC.Day8;

class Part2
{
    private static bool[] _instructions = Array.Empty<bool>();
    private static Dictionary<string, NextNode> _nodes = new();

    public static string Run(string puzzle_input)
    {
        _instructions = GetLeftRightInstructions(puzzle_input);

        _nodes = GetNodes(puzzle_input);

        string[] start_nodes = GetNodesEndingWith('A');

        List<int> result_steps = new();
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

        Dictionary<string, NextNode> nodes = new();

        string[] lines  = puzzle_input.Split('\n').ToArray();

        int ends_with_A = 0;
        int ends_with_Z = 0;

        int index = 2; // nodes start from 3rd line in puzzle input
        while (index < lines.Length)
        {
            string line = lines[index];
            if (line.Length < 16) break; // might have a new line in the end

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
        var nodes_ending_with = new List<string>();

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

        // start with 1 (it will only grow from here)
        long current_lcm = 1;

        for (int i = 0; i < factors.Length; i++)
        {
            // Euclidean algorithm for greatest common divisor
            // https://en.wikipedia.org/wiki/Euclidean_algorithm
            long a = current_lcm;
            long b = factors[i];
            bool stop_dividing = false;
            while (!stop_dividing)
            {
                if (a > b) a = a %= b;
                else       b = b % a;

                stop_dividing = (a == 0 || b == 0);
            }
            // take the greatest value that holds the gcd
            long gcd = (a > b) ? a : b;

            // use greatest common divisor to calculate least common multiple for two numbers
            current_lcm = current_lcm*factors[i] / gcd;
        }

        // this will be the final least common multiple
        return current_lcm;
    }
}

// the datatype annotation grew so big, lets introduce a class
internal class NextNode
{
    private string _left;
    private string _right;

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
