namespace AoC.Day24;

class Part2
{
    public static string Run(string puzzle_input)
    {
        (long x, long y, long z, long vx, long vy, long vz)[] hailstones = GetHailstones(puzzle_input);

        int ans = FindPositionAndVelocity(hailstones);

        return ans.ToString();
    }

    private static (long x, long y, long z, long vx, long vy, long vz)[] GetHailstones(string input)
    {
        string[] s_hailstones = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        List<(long x, long y, long z, long vx, long vy, long vz)> hailstones = [];

        for (int i = 0; i < s_hailstones.Length; i++)
        {
            string[] parts = s_hailstones[i].Split(" @ ");

            long[] p = Array.ConvertAll( parts[0].Split(", "), long.Parse );
            long[] v = Array.ConvertAll( parts[1].Split(", "), long.Parse );

            (long x, long y, long z, long vx, long vy, long vz) hs = (p[0], p[1], p[2], v[0], v[1], v[2]);

            hailstones.Add(hs);
        }

        return hailstones.ToArray();
    }

    private static int FindPositionAndVelocity((long x, long y, long z, long vx, long vy, long vz)[] hs)
    {
        int ans = 0;


        return ans;
    }


}
