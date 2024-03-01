namespace AoC.Day20;

class Part2
{
    public static string Run(string puzzle_input)
    {
        Dictionary<string, Module> modules = GetModules(puzzle_input);

        Dictionary<string, int> lcm_candidates = GetCanditatesForLCM(modules);

        long res = PulsePropagation(modules, lcm_candidates);

        return res.ToString();
    }

    private static Dictionary<string, Module> GetModules(string puzzle_input)
    {
        // temporary store module data in these two..
        Dictionary<string, ModuleType> module_types = [];
        Dictionary<string, string[]> module_destinations = [];

        string[] lines = puzzle_input.Split("\n", StringSplitOptions.RemoveEmptyEntries);

        foreach (var line in lines)
        {
            string[] parts = line.Split(" -> ");

            string name = parts[0];
            string[] destinations = parts[1].Split(", ");
            ModuleType type = name[0] switch
            {
                '%' => ModuleType.FlipFlop,
                '&' => ModuleType.Conjunction,
                 _  => ModuleType.Broadcaster, // no prefix -> we found the boradcaster module
            };

            // remove leading prefix (first character) if '%' or '&'
            if (type != ModuleType.Broadcaster) name = name[1..];

            module_types[name] = type;
            module_destinations[name] = destinations;
        }

        // create the final data for this puzzle
        Dictionary<string, Module> modules = [];

        foreach ( (string module_name, ModuleType type) in module_types)
        {
            Dictionary<string, bool> input_modules = [];

            if (type == ModuleType.Conjunction)
            {
                foreach ( (string input_module_name, string[] output_modules) in module_destinations)
                {
                    foreach (string output_module in output_modules)
                    {
                        if (output_module == module_name)
                        {
                            input_modules[input_module_name] = false; // default pulse is low e.g. false
                            break;
                        }
                    }
                }
            }

            modules[module_name] = new Module
            {
                Type = type,
                DestinationModules = module_destinations[module_name],
                PulseMemory = input_modules,
                PulseOut = false,
            };
        }

        return modules;
    }

    private static Dictionary<string, int> GetCanditatesForLCM(Dictionary<string, Module> modules)
    {
        // OK, we wont be able to write a 100% generic solution to this one..
        // Reason is that we would have to write many more lines of code to solve this problem.

        // We have to make som assumptions about the conventions of the input.
        // Following discussions found in the AoC community, it looks like everyones input do follow the convention.

        // This solution only works if the input module for "rx" is a conjunction module
        // and if that module in turn only has conjunction modules as its input.

        // What's going on?
        // 1st. Check wether rx (last module) exists or not.
        // 2nd. Check if rx's has one input module (lets say it's name is "bb").
        // 3rd: Check if that is the only input for rx AND that it is of type conjunction module.
        // 4th. Keep all input modules for "bb" and assert they are of type conjunction modules too.
        // If all modules except "rx" are conjunction modules (we do not care for rx) then we are good to go..
        // We return a dictionary for the modules that are part of the input of the "bb" module.

        // So basically we want [m1, m2, m3,.. m(N)] which are all inputs to "bb" which in turn is input to "rx".
        // When all modules [m1, m2, m3,.. m(N)] sends a high pulse, then "bb" will send a low pulse to "rx".
        // The total button presses for this to happen is calculated using the least common multiple for these.

        bool rx_found = false;
        bool rx_input_module_is_conjunction = false;
        int rx_input_count = 0;
        string rx_input_module = string.Empty;

        foreach ( (string module_name, Module module) in modules )
        {
            // check destination module for "rx" and
            if (module.DestinationModules.Contains("rx"))
            {
                rx_input_count++;
                rx_found = true;
                rx_input_module = module_name;
                rx_input_module_is_conjunction = modules[rx_input_module].Type == ModuleType.Conjunction;
            }
        }

        System.Diagnostics.Debug.Assert(rx_found, " rx module does not exist found");
        System.Diagnostics.Debug.Assert(rx_input_module != string.Empty, " rx input module not found");
        System.Diagnostics.Debug.Assert(rx_input_module_is_conjunction, " rx input module is not conjunction module");
        System.Diagnostics.Debug.Assert(rx_input_count == 1, " rx module has more than 1 input");

        Dictionary<string, int> lcm_candidates = [];
        bool all_candidates_are_conjunction = true;

        foreach ( (string module_name, Module module) in modules )
        {
            if (module.DestinationModules.Contains(rx_input_module))
            {
                lcm_candidates[module_name] = 0;
                if (module.Type != ModuleType.Conjunction) all_candidates_are_conjunction = false;
            }
        }

        System.Diagnostics.Debug.Assert(lcm_candidates.Count > 0, " no modules found in candidates");
        System.Diagnostics.Debug.Assert(all_candidates_are_conjunction, " should only have conjunction modules");

        return lcm_candidates;
    }

    private static long PulsePropagation(Dictionary<string, Module> modules, Dictionary<string, int> lcm_candidates)
    {
        Queue<string> queue = [];
        int button_pushes = 0;

        while (true)
        {
            // Should have a solution before this many iterations..
            if (button_pushes > 10000)
            {
                Console.WriteLine($"ERROR: Reached 10000 iterations, result might take forever to compute..");
                Environment.Exit(1);
            }

            // we always start with firng a low pulse into the broadcaster
            button_pushes++;
            queue.Enqueue("broadcaster");
            while (queue.Count > 0)
            {
                string cur_module = queue.Dequeue();

                bool pulse = modules[cur_module].PulseOut;
                foreach (string next_module in modules[cur_module].DestinationModules)
                {

                    // do not handle modules with no input
                    if (!modules.ContainsKey(next_module)) continue;

                    bool pulse_received = modules[next_module].ReseivePulse(pulse, cur_module);

                    if (pulse_received) queue.Enqueue(next_module);
                }

                if (lcm_candidates.TryGetValue(cur_module, out int candidate_value))
                {
                    if (modules[cur_module].PulseOut) // if high pulse
                    {
                        if (candidate_value == 0) // prevent adding new numbers, only add if 0 which is default value
                        {
                            lcm_candidates[cur_module] = button_pushes;
                        }
                    }
                }
            }

            bool lcm_ready = true;
            foreach (int n in lcm_candidates.Values)
            {
                if (n == 0) lcm_ready = false;
            }

            if (lcm_ready)
            {
                return LCM(lcm_candidates.Values.ToArray());
            }
        }
    }

    private enum ModuleType
    {
        Broadcaster = -1,
        FlipFlop = 0,
        Conjunction = 1,
    }

    private class Module
    {
        public required ModuleType Type;
        public required string[] DestinationModules;
        public required Dictionary<string, bool> PulseMemory; // only for conjunction module
        public required bool PulseOut; // true => high, low => false

        public bool ReseivePulse(bool pulse, string from)
        {
            if (Type == ModuleType.FlipFlop)
            {
                /*
                    FlipFlop modules (prefix %).
                    If received high pulse -> ignore it
                    If received low pulse -> flip pulse high to low or low to high e.g. on / off
                */
                if (pulse == false)
                {
                    PulseOut = !PulseOut;

                    return true;
                }
            }
            else if (Type == ModuleType.Conjunction)
            {
                /*
                    Conjunction modules (prefix &).
                    When a pulse is received, the conjunction module updates its memory for that module.
                    If it remembers high pulses for all inputs, set low pulse;
                    otherwise, set high pulse.
                */

                PulseMemory[from] = pulse;

                PulseOut = false;
                foreach (bool p in PulseMemory.Values)
                {
                    if (p == false)
                    {
                        PulseOut = true;
                        return true;
                    }
                }

                return true;
            }

            return false;
        }
    }

    private static long LCM(int[] factors)
    {
        // https://en.wikipedia.org/wiki/Least_common_multiple

        // Calculates the least common multiple
        // We do it efficiently by finding the greatest common divisor.

        // Start with the first number (can be any number, but first will do).
        long least_common_multiple = factors[0]; // The lcm will only grow or stay the same from here..

        for (int i = 1; i < factors.Length; i++)
        {
            // Euclidean algorithm for greatest common divisor
            // https://en.wikipedia.org/wiki/Euclidean_algorithm

            long a = least_common_multiple;
            long b = factors[i];

            bool stop_dividing = false;
            while (!stop_dividing)
            {
                if (a > b) a %= b;
                else       b %= a;

                stop_dividing = a == 0 || b == 0;
            }
            // The greatest value holds the gcd..
            long greatest_common_divisor = a > b ? a : b;

            // Use GCD to calculate the LCM for two numbers
            least_common_multiple = least_common_multiple * factors[i] / greatest_common_divisor;
        }

        return least_common_multiple;
    }
}
