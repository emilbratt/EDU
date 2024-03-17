namespace AoC.Day24;

class Part1
{
    public static string Run(string puzzle_input)
    {
        long min_dist = 7;
        long max_dist = 27;

        (long px, long py, long vx, long vy)[] hailstones = GetHailstones(puzzle_input);

        // Dynamically adjust for example input vs real input by checking first x position..
        if (hailstones[0].px != 19)
        {
            min_dist = 200000000000000;
            max_dist = 400000000000000;
        }

        int ans = CountIntersections(hailstones, min_dist, max_dist);

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

    private static int CountIntersections((long px, long py, long vx, long vy)[] hs,
                                          long min,
                                          long max)
    {
        int ans = 0;

        for (int i = 0; i < hs.Length - 1; i++)
        {
            for (int j = i + 1; j < hs.Length; j++)
            {
                if (WillCollide(hs[i], hs[j], min, max)) ans++;
            }
        }

        return ans;
    }

    private static bool WillCollide((long px, long py, long vx, long vy) h1,
                                    (long px, long py, long vx, long vy) h2,
                                    long min,
                                    long max)
    {
        long D = (h2.vx * h1.vy) - (h2.vy * h1.vx);
        if (D == 0) return false;

        // Difference (delta) for both positions.
        long dy = h2.py - h1.py;
        long dx = h2.px - h1.px;
        long u = (dy * h2.vx - dx * h2.vy) / D;
        long v = (dy * h1.vx - dx * h1.vy) / D;
        if (u < 0 || v < 0) return false;

        // Intersection point for both X.
        long xi = h1.px +  h1.vx * u;
        if (xi <= min || xi >= max) return false;

        // Intersection point for both Y.
        long yi = h1.py + h1.vy * u;
        if (yi <= min || yi >= max) return false;

        // If we made it to this point, they intersect inside the boundaries limit (min and max).
        return true;
    }
}
