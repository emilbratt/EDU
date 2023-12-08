namespace AoC.Day5;

class Part1
{
    #pragma warning disable CS8602 // Dereference of a possibly null reference.
    private static string[]? _puzzle_input;

    public static string Run(string[] puzzle_input)
    {
        _puzzle_input = puzzle_input;

        long result = GetLowestLocation();

        return result.ToString();
    }

    private static IEnumerable<long> IterIntegersFromString(string str)
    {
        // iterating over all whitepsace delimited numbers in a string
        int i = 0;
        while (i < str.Length)
        {
            string number = "";

            while (char.IsNumber(str[i]))
            {
                number += str[i].ToString();
                i++;
                if (i == str.Length) break;
            }

            if (number.Length > 0) yield return long.Parse(number);

            i++;
        }
    }

    private static Dictionary<long, (long, long)> GetMap(string name_of_map)
    {
        // map = { source => (destination, range) }
        var map = new Dictionary<long, (long, long)>();

        bool extract_numbers = false;
        int index = 0;
        while (true)
        {
            while (extract_numbers)
            {
                long[] numbers = IterIntegersFromString(_puzzle_input[index]).ToArray();

                // we are done when a new empty line appears
                if (numbers.Length == 0) return map;

                long source_start = numbers[1];
                long dest = numbers[0];
                long range = numbers[2];

                map.Add(source_start, (dest, range));

                index++;

                // we are done if last line
                if (index == _puzzle_input.Length) return map;
            }

            // error if no map is generated
            if (index == _puzzle_input.Length) Environment.Exit(1);

            // next lines will have the map content we desire
            if (_puzzle_input[index].Contains(name_of_map)) extract_numbers = true;

            index++;
        }
    }

    private static long MapConvert(long input_source, Dictionary<long, (long, long)> map)
    {
        long to_destination = input_source;

        foreach (long source_start in map.Keys)
        {
            long dest = map[source_start].Item1;
            long range = map[source_start].Item2;
            long source_end = source_start + range;

            if (input_source >= source_start && input_source <= source_end)
            {
                return input_source + (dest - source_start);
            }
        }

        return to_destination;
    }

    private static long GetLowestLocation()
    {
        // These will have type: Dictionary<long, (long dest, long range)>
        var seed_soil_map = GetMap("seed-to-soil");
        var soil_ertilizer_map = GetMap("soil-to-fertilizer");
        var fertilizer_water_map = GetMap("fertilizer-to-water");
        var water_light_map = GetMap("water-to-light");
        var light_temperature_map = GetMap("light-to-temperature");
        var temperature_humidity_map = GetMap("temperature-to-humidity");
        var humidity_location_map = GetMap("humidity-to-location");

        // all seeds go into a single array
        long[] seeds = IterIntegersFromString(_puzzle_input[0]).ToArray();

        long lowest_location = 0;

        bool first = true;
        foreach (long seed in seeds)
        {
            long soil = MapConvert(seed, seed_soil_map);
            long fertilizer = MapConvert(soil, soil_ertilizer_map);
            long water = MapConvert(fertilizer, fertilizer_water_map);
            long light = MapConvert(water, water_light_map);
            long temp = MapConvert(light, light_temperature_map);
            long humidity = MapConvert(temp, temperature_humidity_map);
            long location = MapConvert(humidity, humidity_location_map);

            if (first)
            {
                first = false;
                lowest_location = location;
            }
            else if (location < lowest_location) lowest_location = location;
        }

        return lowest_location;
    }
}
