class Program
{
    static void Main(string[] args)
    {
        int number_of_arguments = args.Length;

        // NO ARGUMENTS PASSED -> RUN THE WHOLE CALENDAR
        if (number_of_arguments == 0)
        {
            // lets measure the total execution time for all puzzles
            var stop_watch = System.Diagnostics.Stopwatch.StartNew();

            for (int day = 1; day <= 25; day++)
            {
                Puzzle(day.ToString(), "1");
                Puzzle(day.ToString(), "2");
            }

            stop_watch.Stop();
            long timer = stop_watch.ElapsedMilliseconds;

            Console.WriteLine($"Total execution time: {timer} milliseconds");
        }

        // ONE ARGUMENT PASSED -> RUN ONE DAY AND BOTH PARTS
        else if (number_of_arguments == 1)
        {
            Puzzle(args[0], "1");
            Puzzle(args[0], "2");
        }

        // TWO ARGUMENT PASSED -> RUN ONE PART FOR ONE DAY
        else if (number_of_arguments == 2)
        {
            Puzzle(args[0], args[1]);
        }
    }

    private static void Puzzle(string day, string part)
    {
        var puzzle_io = new AoC.PuzzleIO();

        string puzzle_input = puzzle_io.Input(day);

        var stop_watch = System.Diagnostics.Stopwatch.StartNew();

        string puzzle_output = (day, part) switch
        {
            // Trebuchet?!
            ( "1", "1" ) => AoC.Day1.Part1.Run(puzzle_input),
            ( "1", "2" ) => AoC.Day1.Part2.Run(puzzle_input),

            // Cube Conundrum
            ( "2", "1" ) => AoC.Day2.Part1.Run(puzzle_input),
            ( "2", "2" ) => AoC.Day2.Part2.Run(puzzle_input),

            // Gear Ratios
            ( "3", "1" ) => AoC.Day3.Part1.Run(puzzle_input),
            ( "3", "2" ) => AoC.Day3.Part2.Run(puzzle_input),

            // Scratchcards
            ( "4", "1" ) => AoC.Day4.Part1.Run(puzzle_input),
            ( "4", "2" ) => AoC.Day4.Part2.Run(puzzle_input),

            // If You Give A Seed A Fertilizer
            ( "5", "1" ) => AoC.Day5.Part1.Run(puzzle_input),
            ( "5", "2" ) => AoC.Day5.Part2.Run(puzzle_input),

            // Wait For It
            ( "6", "1" ) => AoC.Day6.Part1.Run(puzzle_input),
            ( "6", "2" ) => AoC.Day6.Part2.Run(puzzle_input),

            // Camel Cards
            ( "7", "1" ) => AoC.Day7.Part1.Run(puzzle_input),
            ( "7", "2" ) => AoC.Day7.Part2.Run(puzzle_input),

            // Haunted Wasteland
            ( "8", "1" ) => AoC.Day8.Part1.Run(puzzle_input),
            ( "8", "2" ) => AoC.Day8.Part2.Run(puzzle_input),

            // Mirage Maintenance
            ( "9", "1" ) => AoC.Day9.Part1.Run(puzzle_input),
            ( "9", "2" ) => AoC.Day9.Part2.Run(puzzle_input),

            // Pipe Maze
            ( "10", "1" ) => AoC.Day10.Part1.Run(puzzle_input),
            ( "10", "2" ) => AoC.Day10.Part2.Run(puzzle_input),

            // Cosmic Expansion
            ( "11", "1" ) => AoC.Day11.Part1.Run(puzzle_input),
            ( "11", "2" ) => AoC.Day11.Part2.Run(puzzle_input),

            // Hot Springs
            ( "12", "1" ) => AoC.Day12.Part1.Run(puzzle_input),
            ( "12", "2" ) => AoC.Day12.Part2.Run(puzzle_input),

            // Point of Incidence
            ( "13", "1" ) => AoC.Day13.Part1.Run(puzzle_input),
            ( "13", "2" ) => AoC.Day13.Part2.Run(puzzle_input),

            // Parabolic Reflector Dish
            ( "14", "1" ) => AoC.Day14.Part1.Run(puzzle_input),
            ( "14", "2" ) => AoC.Day14.Part2.Run(puzzle_input),

            // Lens Library
            ( "15", "1" ) => AoC.Day15.Part1.Run(puzzle_input),
            ( "15", "2" ) => AoC.Day15.Part2.Run(puzzle_input),

            // The Floor Will Be Lava
            ( "16", "1" ) => AoC.Day16.Part1.Run(puzzle_input),
            ( "16", "2" ) => AoC.Day16.Part2.Run(puzzle_input),

            // Aplenty
            ( "19", "1" ) => AoC.Day19.Part1.Run(puzzle_input),
            ( "19", "2" ) => AoC.Day19.Part2.Run(puzzle_input),

            _ => String.Empty,
        };

        stop_watch.Stop();
        long time_lapsed = stop_watch.ElapsedMilliseconds;

        if (puzzle_input == String.Empty)
        {
            Console.WriteLine($"Day {day} Part {part}\t| No puzzle input or puuzle not implemented");
        }
        else
        {
            string output_response = puzzle_io.Output(puzzle_output, day, part);
            Console.WriteLine($"Day {day} part {part}\t| Output: {output_response}\t| time: {time_lapsed} milliseconds");
        }
    }
}
