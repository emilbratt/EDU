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

        Dictionary<string, Module> modules = [];
        foreach ( (string name, ModuleType type) in module_types)
        {
            Dictionary<string, ModulePulse> modules_from_pulse = [];
            foreach ( (string module_from, string[] modules_to) in module_destinations)
            {
                foreach (string module_to in modules_to)
                {
                    if (module_to == name)
                    {
                        modules_from_pulse[module_from] = ModulePulse.Low;
                        break;
                    }
                }
            }

            modules[name] = new Module
            {
                Type = type,
                DestinationModules = module_destinations[name],
                PulseMemory = modules_from_pulse,
                PulseOut = ModulePulse.Low,
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
            // we always start with a low pulse when queueing the broadcaster
            button_pushes--;
            low_pulses++;
            queue.Enqueue("broadcaster");

            while (queue.Count > 0)
            {
                string cur_module = queue.Dequeue();

                foreach (string next_module in modules[cur_module].DestinationModules)
                {
                    ModulePulse pulse = modules[cur_module].PulseOut;

                    if (pulse == ModulePulse.High) high_pulses++;
                    else if (pulse == ModulePulse.Low) low_pulses++;

                    // do not handle modules with no input
                    if (!modules.ContainsKey(next_module)) continue;

                    bool pulse_received = modules[next_module].ReseivePulse(pulse, cur_module);

                    if (pulse_received) queue.Enqueue(next_module);
                }
            }
        }

        return high_pulses * low_pulses;
    }

    private enum ModulePulse
    {
        Low  = 0,
        High = 1,
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
        public required Dictionary<string, ModulePulse> PulseMemory;
        public required ModulePulse PulseOut;

        public bool ReseivePulse(ModulePulse pulse, string from)
        {
            PulseMemory[from] = pulse;

            if (Type == ModuleType.FlipFlop)
            {
                /*
                    FlipFlop modules (prefix %).
                    If received high pulse -> ignore it
                    If received low pulse -> flip pulse high to low or low to high e.g. on / off
                */
                if (pulse == ModulePulse.Low)
                {
                    PulseOut = PulseOut switch
                    {
                        ModulePulse.High => ModulePulse.Low,
                        ModulePulse.Low => ModulePulse.High,
                        _ => PulseOut,
                    };

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
                PulseOut = ModulePulse.Low;
                foreach (ModulePulse p in PulseMemory.Values)
                {
                    if (p == ModulePulse.Low)
                    {
                        PulseOut = ModulePulse.High;
                        return true;
                    }
                }

                return true;
            }

            return false;
        }
    }

}
