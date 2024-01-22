namespace AoC.Day19;

class Part1
{
    private static Dictionary<string, string[]> _workflows = []; // key => workflow/rules

    public static string Run(string puzzle_input)
    {
        _workflows = GetWorkflows(puzzle_input);

        Dictionary<char, int>[] part_ratings = GetPartRatings(puzzle_input);

        int result = 0;
        foreach (Dictionary<char, int> part_rating in part_ratings)
        {
            bool accepted = RecursiveRunWorkflow("in", part_rating);

            if (accepted) result += part_rating.Values.Sum();
        }

        return result.ToString();
    }

    // 1st part of input
    public static Dictionary<string, string[]> GetWorkflows(string puzzle_input)
    {
        string first_part = puzzle_input.Split("\n\n")[0];

        string[] workflows = first_part.Split("\n", StringSplitOptions.RemoveEmptyEntries);

        Dictionary<string, string[]> new_workflows = new();
        foreach (string wf in workflows)
        {
            string[] key_and_rules = wf.Split("{");

            // a key can for example be: "px"
            string key = key_and_rules[0];

            // a rule might look like this: [ "a<2006:qkq", "m>2090:A", "rfg" ]
            string[] rules = key_and_rules[1].Split("}")[0].Split(',');

            new_workflows.Add(key, rules);
        }

        return new_workflows;
    }

    // 2nd part of input
    public static Dictionary<char, int>[] GetPartRatings(string puzzle_input)
    {
        string second_part = puzzle_input.Split("\n\n")[1];

        string[] ratings = second_part.Split("\n", StringSplitOptions.RemoveEmptyEntries);

        var part_ratings = new Dictionary<char, int>[ratings.Length];

        int index = 0;
        while (index < ratings.Length)
        {
            // every line has the same formating, we can easily extract each value
            string[] comma_separated = ratings[index].Trim('{').Trim('}').Split(',');

            Dictionary<char, int> current_part_rating = new();
            foreach (string part in comma_separated)
            {
                string[] key_and_value = part.Split('=');
                char key = Convert.ToChar(key_and_value[0]); // will be 'x', 'm', 'a' or 's'
                int val = Convert.ToInt32(key_and_value[1]);
                current_part_rating.Add(key, val);
            }

            part_ratings[index] = current_part_rating;
            index++;
        }

        return part_ratings.ToArray();
    }

    public static bool RecursiveRunWorkflow(string workflow_key, Dictionary<char, int> rating_map)
    {
        if (_workflows.ContainsKey(workflow_key))
        {
            foreach (string wf in _workflows[workflow_key])
            {
                // if the value is final (A, R) or a matchign key, this block kicks in
                if (wf == "A" || wf == "R" || _workflows.ContainsKey(wf))
                {
                    return RecursiveRunWorkflow(wf, rating_map);
                }

                // at this point, we are certain there is a rule, lets work through it
                string[] rules_and_next_key = wf.Split(':');

                string rules = rules_and_next_key[0];
                string next_workflow = rules_and_next_key[1];

                char part_key = rules[0]; // 'x', 'm', 'a' and 's'
                int part_val = rating_map[part_key];
                char op_symbol = rules[1]; // operator symbol - either greater than '>' or less than '<'
                int rule_val = int.Parse(rules.Split(op_symbol)[1].Split(':')[0]);

                // this is the same as checking if (a > b) if operator is '>' and (a < b) if operator is '<'
                int res = op_symbol == '>' ? part_val - rule_val : rule_val - part_val;
                if (res > 0) return RecursiveRunWorkflow(next_workflow, rating_map);
            }
        }

        // this is the base case and should only be reached if it is the final end point of a workflow
        return workflow_key == "A";
    }
}
