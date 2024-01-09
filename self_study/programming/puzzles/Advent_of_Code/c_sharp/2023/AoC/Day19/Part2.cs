#pragma warning disable CS8602 // Dereference of a possibly null reference.

namespace AoC.Day19;

class Part2
{
    private static Dictionary<string, string[]>? _workflows; // key => workflow/rules

    public static string Run(string puzzle_input)
    {
        _workflows = GetWorkflows(puzzle_input);

        // introducing a special type for the ranges (will be duplicated when run through workflow)
        WorkflowRanges ranges = new() {
            DestinationWF = "in",
            X = (1, 4000),
            M = (1, 4000),
            A = (1, 4000),
            S = (1, 4000),
        };

        long result = RecursiveRunWorkflowRanges(ranges);

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
        // from input range, slice it into two tuples with ranges using ..split
        var a = ( range.low, split - 1 );
        var b = ( split, range.high );

        return ( a, b );
    }

    public static long RecursiveRunWorkflowRanges(WorkflowRanges ranges)
    {
        // Workflows are comma separated strings and might look like this: s<1665:A,a>1628:km,m>875:tr,gmm
        // For eaxmple, the 2nd part is an expression saying if the a-value is greater than 1628; then goto workflow "km"
        // last value is always a direct goto.
        // If any goto value is either "A" or "R" (final result) that means it is accepted or rejected respectively..

        bool has_workflow = _workflows.ContainsKey(ranges.DestinationWF);
        if (has_workflow)
        {
            long accumulated_result = 0;

            foreach (string expression_or_goto in _workflows[ranges.DestinationWF])
            {
                // expressions contain a ':' symbol
                if (expression_or_goto.Contains(':'))
                {
                    // extract the key to look up the next workflo (..or it might be "A" or "R"..)
                    string next_wf_key = expression_or_goto.Split(':')[1];

                    // which range to use in condition e.g. x, m, a or s
                    char key = Convert.ToChar(expression_or_goto[0]);

                    // conditional operation - greater than > or less than <
                    char op =  Convert.ToChar(expression_or_goto[1]);

                    // target value for conditional opearation
                    int target_val = int.Parse(expression_or_goto.Split(':')[0].Split(op)[1]);

                    // the range that passes the condition goes to the conditions next workflow
                    (int low, int high) satisfied_range; // greater than or less than (satisfied)
                    (int low, int high) failed_range;    // not greater than or not less than (failed)


                    // THIS IS WHERE THE CORE OF THE LOGIC HAPPENS - KEEPING THE SATSIFIED RANGES AND PASSING THOSE ON

                    // if greater than, the satisfied range starts from greater than and goes up to high
                    if (op == '>') (failed_range, satisfied_range) = BisectRange(ranges.Get(key), target_val + 1);
                    // if less than, the satisfied range starts at low and goes up to (not including) less than
                    else (satisfied_range, failed_range) = BisectRange(ranges.Get(key), target_val);

                    // make a copy of the current ranges (will hold the satisfied range)
                    WorkflowRanges satisfied_ranges = ranges.Clone();

                    // next assigned workflow for satisfied condition
                    satisfied_ranges.DestinationWF = next_wf_key;

                    // apply the satisfied range to the copy
                    satisfied_ranges.Set(key, satisfied_range);

                    // pass on the satsified copy
                    accumulated_result += RecursiveRunWorkflowRanges(satisfied_ranges);

                    // apply the un-satisfied range to the original
                    ranges.Set(key, failed_range);

                }
                else
                {
                    // the remaining one is finally passed on (has next workflow or one of "A" and "R")
                    ranges.DestinationWF = expression_or_goto;
                    return accumulated_result + RecursiveRunWorkflowRanges(ranges);
                }
            }
        }

        // base case - the final endpoint "A"-ccepted or "R"-ejected
        return ranges.DestinationWF == "A" ? ranges.Product() : 0;
    }
}

internal class WorkflowRanges
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

    // returns a new object with the same values
    public WorkflowRanges Clone()
    {
        var clone = new WorkflowRanges
        {
            DestinationWF = DestinationWF,
            X = X,
            M = M,
            A = A,
            S = S
        };
        return clone;
    }
}
