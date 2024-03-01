namespace AoC.Day20;

class Part1
{
    public static string Run(string puzzle_input)
    {
        var modules = GetModules(puzzle_input);

        int button_pushes = 1000;

        int res = PulsePropagation(modules, button_pushes);

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

    private static int PulsePropagation(Dictionary<string, Module> modules, int button_pushes)
    {
        int low_pulses = 0;
        int high_pulses = 0;

        Queue<string> queue = [];

        while (button_pushes > 0)
        {
            // we always start with firng a low pulse into the broadcaster
            low_pulses++;
            queue.Enqueue("broadcaster");

            while (queue.Count > 0)
            {
                string cur_module = queue.Dequeue();
                bool pulse = modules[cur_module].PulseOut;

                foreach (string next_module in modules[cur_module].DestinationModules)
                {

                    if (pulse) high_pulses++;
                    else low_pulses++;

                    // do not handle modules with no input
                    if (!modules.ContainsKey(next_module)) continue;

                    bool pulse_received = modules[next_module].ReseivePulse(pulse, cur_module);

                    if (pulse_received) queue.Enqueue(next_module);
                }
            }

            button_pushes--;
        }

        return high_pulses * low_pulses;
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

}
