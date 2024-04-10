namespace AoC.Day24;

class Part2
{
    public static string Run(string puzzle_input)
    {
        var comparer = new HSComparer(); // ..for sorting the hailstones by lowest value.

        (long x, long y, long z, long vx, long vy, long vz)[] hailstones = GetSortedHailstones(puzzle_input, comparer);

        long ans = SmashHailstonesWithRock(hailstones);

        return ans.ToString();
    }

    private static (long x, long y, long z, long vx, long vy, long vz)[] GetSortedHailstones(string input,
                                                                                             HSComparer comparer)
    {
        string[] s_hailstones = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        var hailstones = new (long x, long y, long z, long vx, long vy, long vz)[s_hailstones.Length];

        for (int i = 0; i < s_hailstones.Length; i++)
        {
            string[] parts = s_hailstones[i].Split(" @ ");

            long[] p = Array.ConvertAll( parts[0].Split(", "), long.Parse );
            long[] v = Array.ConvertAll( parts[1].Split(", "), long.Parse );

            (long x, long y, long z, long vx, long vy, long vz) = (p[0], p[1], p[2], v[0], v[1], v[2]);
            hailstones[i] = (x, y, z, vx, vy, vz);
        }

        Array.Sort(hailstones, comparer);

        return hailstones;
    }

    class HSComparer : IComparer<(long, long, long, long, long, long)>
    {
        // Logic for sorting the array of tuples starting with first element (x) and if equal,
        // then try 2nd (y) element and if y is equal, then try the 3rd (z) element (last attempt)..
        public int Compare((long, long, long, long, long, long) a, (long, long, long, long, long, long) b)
        {
            int res = a.Item1.CompareTo(b.Item1);
            if (res == 0) res = a.Item2.CompareTo(b.Item2); // both x where equal, try y..
            if (res == 0) res = a.Item3.CompareTo(b.Item3); // both y where equal too, try z..

            // This will return one of: [-1, 0 or 1].
            return res;
        }
    }

    private static long SmashHailstonesWithRock((long x, long y, long z, long vx, long vy, long vz)[] hailstones)
    {
        HashSet<long> possible_x = [];
        HashSet<long> possible_y = [];
        HashSet<long> possible_z = [];

        for (int i = 0; i < hailstones.Length; i++)
        {
            for (int j = i; j < hailstones.Length; j++)
            {
                (long x, long y, long z, long vx, long vy, long vz) A = hailstones[i];
                (long x, long y, long z, long vx, long vy, long vz) B = hailstones[j];

                if (A.vx == B.vx && Math.Abs(A.vx) > 100)
                {
                    HashSet<long> new_x = [];
                    long diff = B.x - A.x;
                    for (long v = -1000; v <= 1000; v++)
                    {
                        if (v == A.vx) continue;
                        if (diff % (v-A.vx) == 0) new_x.Add(v);
                    }

                    if (possible_x.Count != 0) possible_x.IntersectWith(new_x);
                    else possible_x = new_x;
                }

                if (A.vy == B.vy && Math.Abs(A.vy) > 100)
                {
                    HashSet<long> new_y = [];
                    long diff = B.y - A.y;
                    for (long v = -1000; v <= 1000; v++)
                    {
                        if (v == A.vy) continue;
                        if (diff % (v-A.vy) == 0) new_y.Add(v);
                    }

                    if (possible_y.Count != 0) possible_y.IntersectWith(new_y);
                    else possible_y = new_y;
                }

                if (A.vz == B.vz && Math.Abs(A.vz) > 100)
                {
                    HashSet<long> new_z = [];
                    long diff = B.z - A.z;
                    for (long v = -1000; v <= 1000; v++)
                    {
                        if (v == A.vz) continue;
                        if (diff % (v-A.vz) == 0) new_z.Add(v);
                    }

                    if (possible_z.Count != 0) possible_z.IntersectWith(new_z);
                    else possible_z = new_z;
                }
            }
        }

        (long x, long y, long z) P = (possible_x.Single(), possible_y.Single(), possible_z.Single());

        (long x, long y, long z, long vx, long vy, long vz) RA = hailstones[0];
        (long x, long y, long z, long vx, long vy, long vz) RB = hailstones[1];

        double a = (double)(RA.vy - P.y) / (RA.vx - P.x);
        double b = (double)(RB.vy - P.y) / (RB.vx - P.x);

        double c = RA.y - (a * RA.x);
        double d = RB.y - (b * RB.x);

        long x = (long)((d - c) / (a - b));
        long y = (long)(a * x + c);

        long time = (x - RA.x) / (RA.vx - P.x);
        long z = RA.z + (RA.vz - P.z) * time;

        return x + y + z;
    }
}
