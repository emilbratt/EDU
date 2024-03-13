namespace AoC.Day24;

class Part1
{
    public static string Run(string puzzle_input)
    {
        const long min_test = 200000000000000;
        const long max_test = 400000000000000;

        // const long min_test = 7;
        // const long max_test = 27;
        // string ex_input = @"19, 13, 30 @ -2,  1, -2
        //                     18, 19, 22 @ -1, -1, -2
        //                     20, 25, 34 @ -2, -2, -4
        //                     12, 31, 28 @ -1, -2, -1
        //                     20, 19, 15 @  1, -5, -3";

        (long px, long py, long vx, long vy)[] hailstones = GetHailstones(puzzle_input);


        int ans = CountIntersections(hailstones, min_test, max_test);

        return ans.ToString();
    }

    private static (long px, long py, long vx, long vy)[] GetHailstones(string input)
    {
        string[] s_hailstones = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        List<(long px, long py, long vx, long vy)> hailstones = [];

        for (int i = 0; i < s_hailstones.Length; i++)
        {
            string[] parts = s_hailstones[i].Split(" @ ");

            long[] p = Array.ConvertAll( parts[0].Split(", "), long.Parse );
            long[] v = Array.ConvertAll( parts[1].Split(", "), long.Parse );

            (long px, long py, long pz, long vy) hs = (p[0], p[1], v[0], v[1]);

            hailstones.Add(hs);
        }

        return hailstones.ToArray();
    }

    private static int CountIntersections((long px, long py, long vx, long vy)[] hs, long min, long max)
    {
        int ans = 0;

        for (int i = 0; i < hs.Length - 1; i++)
        {
            for (int j = i + 1; j < hs.Length; j++)
            {
                Console.WriteLine($"{i},{j}");
                PrintHailstone(hs[i]);
                PrintHailstone(hs[j]);
                Console.WriteLine();
            }
        }

        return ans;
    }

    private static void PrintHailstone((long px, long py, long vx, long vy) hs)
    {
        Console.WriteLine($"Hailstone: {hs.px}, {hs.py} @ {hs.vx}, {hs.vy}");
    }
}
