namespace AoC.Day5;

class Part2
{
    public static string Run(string puzzle_input)
    {
        var mapper = new Mapper(puzzle_input);

        long lowest_location = mapper.FindLowestLocation();

        return lowest_location.ToString();
    }
}

class InputHandle
{
    public static (long seed, long range)[] GetSeeds(string puzzle_input)
    {
        // split all content by new line (and get only the first line)
        string first_line = puzzle_input.Split("\n")[0];

        // split each part by whitespace
        string[] whitespace_split = first_line.Split(' ', StringSplitOptions.RemoveEmptyEntries);

        // parse all numbers (skipping first element that holds the string "seeds:")
        long[] numbers = whitespace_split.Skip(1).Select(long.Parse).ToArray();

        var list_of_tuples = new List<(long, long)>();

        for (int i = 0; i < numbers.Length; i += 2)
        {
            (long, long) _t = (numbers[i], numbers[i+1]);
            list_of_tuples.Add(_t);
        }

        return list_of_tuples.ToArray();
    }

    public static (long, long, long)[] GetMaps(string name_of_map, string puzzle_input)
    {
        string wanted_input_part = String.Empty;

        foreach (string line in puzzle_input.Split("\n\n").Skip(1))
        {
            if (line.Contains(name_of_map)) wanted_input_part = line;
        }

        if (wanted_input_part == string.Empty) Environment.Exit(1);

        var list_of_tuples = new List<(long, long, long)>();
        foreach (var numbers in wanted_input_part.Split("\n").Skip(1).ToArray())
        {
            if (numbers == "") continue;
            string[] arr_numbers = numbers.Split(' ');
            long n1 = long.Parse(arr_numbers[0]);
            long n2 = long.Parse(arr_numbers[1]);
            long n3 = long.Parse(arr_numbers[2]);
            list_of_tuples.Add((n1, n2, n3));
        }

        return list_of_tuples.ToArray();
    }
}

class Mapper
{
    private readonly List<(long, long, long)[]> _maps;
    private readonly (long seed, long range)[] _seeds;

    public Mapper(string puzzle_input)
    {
        _seeds = InputHandle.GetSeeds(puzzle_input);
        _maps = new List<(long, long, long)[]> {
            InputHandle.GetMaps("seed-to-soil", puzzle_input),
            InputHandle.GetMaps("soil-to-fertilizer", puzzle_input),
            InputHandle.GetMaps("fertilizer-to-water", puzzle_input),
            InputHandle.GetMaps("water-to-light", puzzle_input),
            InputHandle.GetMaps("light-to-temperature", puzzle_input),
            InputHandle.GetMaps("temperature-to-humidity", puzzle_input),
            InputHandle.GetMaps("humidity-to-location", puzzle_input),
        };
    }

    public long FindLowestLocation()
    {
        var lowest_locations = new List<long>();

        foreach (var seed_tuple in _seeds)
        {
            long seed = seed_tuple.seed;
            long seed_end = seed + seed_tuple.range;

            long lowest_location = GetLocationFromSeed(seed);

            while (seed < seed_end)
            {
                long increment_window = 1;
                seed += increment_window;

                long result = GetLocationFromSeed(seed);
                long next_result = GetLocationFromSeed(seed + increment_window) - increment_window;

                if (result < lowest_location) lowest_location = result;

                long last_calculated_seed = seed;

                // comment out this while block and this will run for a loooong time
                while (result == next_result && seed < seed_end)
                {
                    // this while block skips a bunch of redundant calculations
                    // ..without this, the solve will become pure brute-force

                    // for any predicted increasing outcome, we know we can skip calculating it
                    // the predictable outcome
                    //  f(seed) == f(seed + n) - n
                    //  ..for all n > seed

                    // preserve the last seed for the last correct prediction
                    last_calculated_seed = seed;

                    // for every predictable outcome, increase the increment by the power of 2
                    increment_window *= 2;

                    result = GetLocationFromSeed(seed);
                    next_result = GetLocationFromSeed(seed + increment_window) - increment_window;

                    // increase seed with the increment_window
                    seed += increment_window;
                }

                seed = last_calculated_seed;
            }

            lowest_locations.Add(lowest_location);
        }

        return lowest_locations.Min();
    }

    private long GetLocationFromSeed(long value)
    {
        foreach (var map in _maps)
        {
            long offset = GetMapOffset(value, map);
            value = value + offset;
        }
        return value;
    }

    private static long GetMapOffset(long input, (long dest, long source, long range)[] map)
    {
        foreach (var (dest, source, range) in map)
        {
            // use map values only if the input falls within the range specified
            if (input >= source && input < source + range) return dest - source;
        }

        return 0;
    }
}
