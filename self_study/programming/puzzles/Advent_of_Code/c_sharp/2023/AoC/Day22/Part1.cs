namespace AoC.Day22;

class Part1
{
    public static string Run(string puzzle_input)
    {
        Brick[] B = GetSnapShotPositions(puzzle_input);

        int ans = CountPossibleDisintegrations(B);

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
                bool collided = B[i].Collision(B[j]);
                System.Diagnostics.Debug.Assert(!collided, " invalid input, some bricks already collide)");
            }
        }

        return B;
    }

    private static int CountPossibleDisintegrations(Brick[] B)
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

        int ans = 0;
        foreach (Brick brick_a in B)
        {
            brick_a.Disintegrated = true;

            foreach (Brick brick_b in B)
            {
                if (brick_b.Disintegrated) continue;
                if (brick_b.Z.s == 1) continue;

                brick_b.Z.s--;
                brick_b.Z.e--;
                bool can_fall = true;
                foreach (Brick brick_c in B)
                {
                    if (brick_c.Disintegrated) continue;
                    if (brick_b.Index == brick_c.Index) continue;
                    if (!brick_b.Collision(brick_c)) continue;

                    can_fall = false;
                    break;
                }
                brick_b.Z.s++;
                brick_b.Z.e++;

                if (!can_fall) continue;

                brick_a.Disintegrated = false;
                break;
            }

            if (brick_a.Disintegrated) ans++;

            brick_a.Disintegrated = false;
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

        public int PossibleFallPosition(Brick b)
        {
            if (X.e < b.X.s || b.X.e < X.s) return 1;
            if (Y.e < b.Y.s || b.Y.e < Y.s) return 1;
            if (Z.s < b.Z.e) return 1;
            return b.Z.e + 1;
        }

        public bool Collision(Brick b)
        {
            if (X.e < b.X.s || b.X.e < X.s) return false;
            if (Y.e < b.Y.s || b.Y.e < Y.s) return false;
            if (Z.e < b.Z.s || b.Z.e < Z.s) return false;
            return true;
        }
    }
}
