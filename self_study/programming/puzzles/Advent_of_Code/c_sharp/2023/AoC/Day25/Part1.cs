namespace AoC.Day25;

class Part1
{
    public static string Run(string puzzle_input)
    {
        List<(string from, string to)> configuration = GetConfiguration(puzzle_input);

        while (true)
        {
            (int min_cut, int ans) = TryFindMinCut(configuration);
            if (min_cut == 3) return ans.ToString();
        }
    }

    private static List<(string from, string to)> GetConfiguration(string puzzle_input)
    {
        /*
            Reformats input formatted as key => values
                jqt: rhn xhk
                ..
                ..
            ..to a list of tuples
            [
                (jqt, rhn),
                (jqt, xhk),
                (xhk, nvd),
                ..
                ..
            ]
        */
        List<(string from, string to)> configuration = [];

        string[] lines = puzzle_input.Split("\n", StringSplitOptions.RemoveEmptyEntries);
        foreach (var line in lines)
        {
            string[] parts = line.Split(": ");

            string component = parts[0];
            string[] connected_components = parts[1].Split(' ');

            foreach (string connected in connected_components)
            {
                var a = (component, connected);
                var b = (connected, component);
                if (configuration.Contains(a)) continue;
                if (configuration.Contains(b)) continue;

                configuration.Add(a);
            }
        }

        return configuration;
    }

    private static (int min_cut, int ans) TryFindMinCut(List<(string from, string to)> edges)
    {
        Random random = new();

        // Count distinct nodes.
        List<string> distinct = [];
        foreach ((string from, string to) in edges)
        {
            if (!distinct.Contains(to)) distinct.Add(to);
            if (!distinct.Contains(from)) distinct.Add(from);
        }
        int v_count = distinct.Count;

        // Contract edges until only two nodes remain.
        Dictionary<string, List<string>> contracted = [];
        while (v_count > 2)
        {
            (string from, string to) random_edge = edges[random.Next(0, edges.Count)];

            if (!contracted.ContainsKey(random_edge.from))
            {
                contracted[random_edge.from] = [];
            } 
            contracted[random_edge.from].Add(random_edge.to);

            if (contracted.ContainsKey(random_edge.to))
            {
                contracted[random_edge.from].AddRange(contracted[random_edge.to]);
                contracted.Remove(random_edge.to);
            }

            List<(string from, string to)> new_edges = [];
            foreach ((string from, string to) edge in edges)
            {
                if (edge.to == random_edge.to)
                {
                    new_edges.Add((edge.from, random_edge.from));
                }
                else if (edge.from == random_edge.to)
                {
                    new_edges.Add((random_edge.from, edge.to));
                }
                else
                {
                    new_edges.Add(edge);
                }
            }

            new_edges.RemoveAll(edge => edge.from == edge.to);
            edges = new_edges;

            v_count--;
        }

        // Count nodes left in each group (should be 2 groups) and multiply them
        int group_a = contracted.First().Value.Count + 1;
        int group_b = contracted.Last().Value.Count + 1;
        // ..Add +1 -> include the key which is also a component.

        return (edges.Count, group_a * group_b);
    }
}
