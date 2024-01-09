#pragma warning disable CS8602 // Dereference of a possibly null reference.

namespace AoC.Day19;

class Part2
{
    private static Dictionary<string, string[]>? _workflows; // key => workflow/rules

    public static string Run(string puzzle_input)
    {
        _workflows = GetWorkflows(puzzle_input);

        // we have made a class that contains all the ranges and also provides methods to handle it
        WorkflowRange wr = new() {
            DestinationWF = "in",
            X = (1, 4000),
            M = (1, 4000),
            A = (1, 4000),
            S = (1, 4000),
        };

        long result = RecursiveRunWorkflowRanges(wr);

        return result.ToString();
    }

    // 1st part of input
    public static Dictionary<string, string[]> GetWorkflows(string puzzle_input)
    {
        string first_part = puzzle_input.Split("\n\n")[0];

        string[] workflows = first_part.Split("\n", StringSplitOptions.RemoveEmptyEntries);

        Dictionary<string, string[]> input_workflows = [];
        foreach (string wf in workflows)
        {
            string[] key_and_rules = wf.Split("{");

            // a key can for example be: "px"
            string key = key_and_rules[0];

            // a rule might look like this: [ "a<2006:qkq", "m>2090:A", "rfg" ]
            string[] rules = key_and_rules[1].Split("}")[0].Split(',');

            input_workflows.Add(key, rules);
        }

        return input_workflows;
    }

    public static ( (int low, int high) a, (int low, int high) b) BisectRange((int low, int high) range, int split)
    {
        // from input range, slice it into two ranges using ..split
        var a = ( range.low, split - 1 );
        var b = ( split, range.high );

        return ( a, b );
    }

    public static long RecursiveRunWorkflowRanges(WorkflowRange ranges)
    {
        // if destination is a new workflow
        if (_workflows.ContainsKey(ranges.DestinationWF))
        {
            long res = 0;
            string[] wfs = _workflows[ranges.DestinationWF];

            foreach (string wf in wfs)
            {
                if (wf.Contains(':')) // if workflow is a contitional one
                {
                    WorkflowRange new_ranges = new()
                    {
                        X = ranges.Get('x'),
                        M = ranges.Get('m'),
                        A = ranges.Get('a'),
                        S = ranges.Get('s')
                    };
                    char key = Convert.ToChar(wf[0]); // x, m, a or s
                    char op =  Convert.ToChar(wf[1]); // > or <
                    int target_val = int.Parse(wf.Split(':')[0].Split(op)[1]); // target value to check against
                    string target_wf = wf.Split(':')[1]; // if satisfied, this is the next assigned workflow
                    new_ranges.DestinationWF = target_wf; // these ranges will be sent if condition is satisfied

                    (int low, int high) satisfied;
                    (int low, int high) rest;
                    if (op == '>') (rest, satisfied) = BisectRange(ranges.Get(key), target_val + 1);
                    else (satisfied, rest) = BisectRange(ranges.Get(key), target_val);

                    ranges.Set(key, rest);
                    new_ranges.Set(key, satisfied);

                    res += RecursiveRunWorkflowRanges(new_ranges);
                }
                else // workflow is not conditional, it is only the key that points to the next workflow
                {
                    ranges.DestinationWF = wf; // should only be the next workflow (also, not "A" or "R")
                    return res + RecursiveRunWorkflowRanges(ranges);
                }
            }
        }

        // this is the base case (the final endpoint "A"-ccepted or "R"-ejected)
        return ranges.DestinationWF == "A" ? ranges.Product() : 0;
    }
}

internal class WorkflowRange
{
    public string DestinationWF { get; set; } = String.Empty;
    public (int low, int high) X { get; set; }
    public (int low, int high) M { get; set; }
    public (int low, int high) A { get; set; }
    public (int low, int high) S { get; set; }

    public (int low, int high) Get(char c)
    {
        return c switch
        {
            'x' => X,
            'm' => M,
            'a' => A,
            's' => S,
            _ => (0, 0),
        };
    }
    public void Set(char c, (int low, int high) new_tupple)
    {
        if      (c == 'x') X = new_tupple;
        else if (c == 'm') M = new_tupple;
        else if (c == 'a') A = new_tupple;
        else if (c == 's') S = new_tupple;
    }
    public long Product()
    {
        long x = X.high + 1 - X.low;
        long m = M.high + 1 - M.low;
        long a = A.high + 1 - A.low;
        long s = S.high + 1 - S.low;
        return x * m * a * s;
    }
}
