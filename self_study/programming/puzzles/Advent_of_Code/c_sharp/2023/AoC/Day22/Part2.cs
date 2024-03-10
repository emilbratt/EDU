namespace AoC.Day22;

class Part2
{
    public static string Run(string puzzle_input)
    {
        Brick[] B = GetSnapShotPositions(puzzle_input);

        B = FallDown(B);

        B = FindSupportedByAndSupporting(B);

        int ans = SumOfAllFallenForAllPossibleDisintegrations(B);

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
            foreach (Brick falling_brick  in B)
            {
                if (falling_brick.Z.s == 1) continue;

                int lowest_z_possible = 1;
                foreach (Brick land_on_brick in B)
                {
                    if (falling_brick.Index == land_on_brick.Index) continue;

                    int lowest = falling_brick.FallsDownOn(land_on_brick);
                    if (lowest > lowest_z_possible) lowest_z_possible = lowest;
                }

                if (lowest_z_possible < falling_brick.Z.s)
                {
                    fell = true;
                    int diff = falling_brick.Z.e - falling_brick.Z.s;
                    falling_brick.Z.s = lowest_z_possible;
                    falling_brick.Z.e = lowest_z_possible + diff;
                }
            }
        }

        return B;
    }

    private static Brick[] FindSupportedByAndSupporting(Brick[] B)
    {
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

    private static int SumOfAllFallenForAllPossibleDisintegrations(Brick[] B)
    {
        List<int> safe_to_remove = []; // Used for a small perf. increase.

        // Commenting out the foreach block below will make this run slightly slower, not by much though..
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
            if (!can_fall) safe_to_remove.Add(brick.Index);
        }

        // Can this be done faster?
        // Bruteforce impl. where we test how many fall by simulating every single brick and add +1 for each fallen..
        int ans = 0;
        foreach (Brick disintegrated_brick in B)
        {
            if (safe_to_remove.Contains(disintegrated_brick.Index)) continue;

            disintegrated_brick.Disintegrated = true;

            List<int> fallen = [];

            bool fell = true;
            while (fell)
            {
                fell = false;
                foreach (Brick falling_brick in B)
                {
                    if (falling_brick.Z.s == 1) continue;
                    if (fallen.Contains(falling_brick.Index)) continue;
                    if (falling_brick.Z.s < disintegrated_brick.Z.e) continue;
                    if (falling_brick.Index == disintegrated_brick.Index) continue;

                    falling_brick.Z.s--;
                    falling_brick.Z.e--;

                    bool collided = false;
                    foreach (Brick land_on_brick in B)
                    {
                        if (falling_brick.Index == land_on_brick.Index) continue;
                        if (falling_brick.CollidesWith(land_on_brick))
                        {
                            collided = true;
                            break;
                        }
                    }
                    if (!collided)
                    {
                        fell = true;
                        fallen.Add(falling_brick.Index);
                    }
                    else
                    {
                        falling_brick.Z.s++;
                        falling_brick.Z.e++;
                    }
                }
            }

            ans += fallen.Count;

            // Restore all fallen bricks to their original position (just move all 1 up).
            foreach (int i in fallen)
            {
                B[i].Z.s++;
                B[i].Z.e++;
            }

            disintegrated_brick.Disintegrated = false;
        }

        return ans;
    }

    private class Brick
    {
        public required int Index;
        public required (int s, int e) X;
        public required (int s, int e) Y;
        public required (int s, int e) Z;

        public bool Disintegrated = false;
        public List<int> Supporting = []; // ..bricks resting on this brick
        public List<int> SupportedBy = []; // ..bricks this brick rests on

        public int FallsDownOn(Brick b)
        {
            if (b.Disintegrated) return 1;

            if (X.e < b.X.s || b.X.e < X.s) return 1;
            if (Y.e < b.Y.s || b.Y.e < Y.s) return 1;
            if (Z.s < b.Z.e) return 1;

            return b.Z.e + 1;
        }

        public bool CollidesWith(Brick b)
        {
            if (b.Disintegrated) return false;

            if (X.e < b.X.s || b.X.e < X.s) return false;
            if (Y.e < b.Y.s || b.Y.e < Y.s) return false;
            if (Z.e < b.Z.s || b.Z.e < Z.s) return false;

            return true;
        }
    }
}
