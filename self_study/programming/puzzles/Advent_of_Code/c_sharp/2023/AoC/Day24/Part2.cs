namespace AoC.Day24;

class Part2
{
    public static string Run(string puzzle_input)
    {
        var hs = GetHailstones(puzzle_input);

        int ans = 0;
        return ans.ToString();
    }

    private static int GetHailstones(string input)
    {
        string[] s_hailstones = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        List<(int px, int py, int pz, int vx, int vy, int vz)> hailstones = [];

        for (int i = 0; i < s_hailstones.Length; i++)
        {
            string[] parts = s_hailstones[i].Split(" @ ");

            int[] p = Array.ConvertAll( parts[0].Split(", "), int.Parse );
            int[] v = Array.ConvertAll( parts[1].Split(", "), int.Parse );

            (int px, int py, int pz, int vx, int vy, int vz) hs = (p[0], p[1], p[2], v[0], v[1], v[2]);

            hailstones.Add(hs);
        }

        return 0;
    }
}
