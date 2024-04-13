namespace AoC.Day22;

class Part1
{
    public static string Run(string puzzle_input)
    {
        Brick[] B = GetSnapShotPositions(puzzle_input);

        B = FallDown(B);

        List<int> removed = CanSafelyRemove(B);

        int ans = removed.Count;

        return ans.ToString();
    }

    private static Brick[] GetSnapShotPositions(string input)
    {

        string[] brick_coordinates = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        Brick[] B = new Brick[brick_coordinates.Length];

        for (int i = 0; i < brick_coordinates.Length; i++)
        {
            string b = brick_coordinates[i];
            string[] splt = b.Split('~');

            int[] s = Array.ConvertAll( splt[0].Split(','), int.Parse );
            int[] e = Array.ConvertAll( splt[1].Split(','), int.Parse );

            (int sx, int sy, int sz, int ex, int ey, int ez) = (s[0], s[1], s[2], e[0], e[1], e[2]);

            // Eevery brick only takes up one width, either in x direction, y direction or z direction.
            if (sx != ex) System.Diagnostics.Debug.Assert(sy == ey && sz == ez, $" {b} is not a single line");
            if (sy != ey) System.Diagnostics.Debug.Assert(sx == ex && sz == ez, $" {b} is not a single line");
            if (sz != ez) System.Diagnostics.Debug.Assert(sx == ex && sy == ey, $" {b} is not a single line");

            // // Move lowest coordinate to the left side.. it will not change physical size or location of the bricks..
            if (ex < sx) (ex, sx) = (sx, ex); // X
            if (ey < sy) (ey, sy) = (sy, ey); // Y
            if (ez < sz) (ez, sz) = (sz, ez); // Z

            // We have testing left point <= right point assertoin on example and real input, we can skip block above..
            bool left_is_lower = sx <= ex && sy <= ey && sz <= ez;

            System.Diagnostics.Debug
                .Assert(left_is_lower, $" {b} - a left x,y or z coordinate is greater than its right counterpart");

            B[i] = new Brick {
                Index = i,
                X = (sx, ex),
                Y = (sy, ey),
                Z = (sz, ez),
            };
        }

        for (int i = 0; i < B.Length; i++)
        {
            for (int j = 0; j < B.Length; j++)
            {
                if (i == j) continue;
                bool collided = B[i].CollidesWith(B[j]);
                System.Diagnostics.Debug.Assert(!collided, " invalid input, some bricks already collide)");
            }
        }

        return B;
    }

    private static Brick[] FallDown(Brick[] B)
    {
        bool fell = true;
        while (fell)
        {
            fell = false;
            foreach (Brick brick_a  in B)
            {
                if (brick_a.Z.s == 1) continue;

                int lowest_z_possible = 1;
                foreach (Brick brick_b in B)
                {
                    if (brick_a.Index == brick_b.Index) continue;

                    int lowest = brick_a.PossibleFallPosition(brick_b);
                    if (lowest > lowest_z_possible) lowest_z_possible = lowest;
                }

                if (lowest_z_possible < brick_a.Z.s)
                {
                    fell = true;
                    int diff = brick_a.Z.e - brick_a.Z.s;
                    brick_a.Z.s = lowest_z_possible;
                    brick_a.Z.e = lowest_z_possible + diff;
                }
            }
        }

        foreach (Brick brick in B)
        {
            brick.Z.s--;
            brick.Z.e--;
            foreach (Brick support_brick in B)
            {
                if (brick.Index == support_brick.Index) continue;

                if (support_brick.CollidesWith(brick))
                {
                    brick.SupportedBy.Add(support_brick.Index);
                }
            }
            brick.Z.s++;
            brick.Z.e++;
        }

        foreach (Brick brick in B)
        {
            foreach (int index in brick.SupportedBy)
            {
                if (B[index].Supporting.Contains(brick.Index)) continue;
                B[index].Supporting.Add(brick.Index);
            }
        }

        return B;
    }

    private static List<int> CanSafelyRemove(Brick[] B)
    {
        List<int> removed = [];
        foreach (Brick brick in B)
        {
            bool can_fall = false;

            foreach (int index in brick.Supporting)
            {
                if (B[index].SupportedBy.Count < 2)
                {
                    can_fall = true;
                    break;
                }
            }

            if (!can_fall) removed.Add(brick.Index);
        }

        return removed;
    }

    private class Brick
    {
        public required int Index;
        public required (int s, int e) X;
        public required (int s, int e) Y;
        public required (int s, int e) Z;

        public List<int> Supporting = [];
        public List<int> SupportedBy = [];

        public int PossibleFallPosition(Brick b)
        {
            if (X.e < b.X.s || b.X.e < X.s) return 1;
            if (Y.e < b.Y.s || b.Y.e < Y.s) return 1;
            if (Z.s < b.Z.e) return 1;
            return b.Z.e + 1;
        }

        public bool CollidesWith(Brick b)
        {
            if (X.e < b.X.s || b.X.e < X.s) return false;
            if (Y.e < b.Y.s || b.Y.e < Y.s) return false;
            if (Z.e < b.Z.s || b.Z.e < Z.s) return false;
            return true;
        }
    }
}
