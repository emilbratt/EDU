﻿// Commandline-argument for this application should be passed as follows
// dotnet run <day> <part>

// Example for day 1 and part 2: dotnet run 1 2
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
        var puzzle_io = new AoC.PuzzleIO(day, part);

        string puzzle_input = puzzle_io.In();

        if (puzzle_input == String.Empty) return;

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

            _ => String.Empty,
        };

        stop_watch.Stop();
        long timer = stop_watch.ElapsedMilliseconds;

        string response = puzzle_io.Out(puzzle_output);

        Console.WriteLine($"Day {day} part {part}\t| Output: {response}\t| time: {timer} milliseconds");
    }
}