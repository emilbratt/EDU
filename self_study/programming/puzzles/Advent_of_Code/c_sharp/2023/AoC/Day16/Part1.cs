namespace AoC.Day16;

class Part1
{
    public static string Run(string puzzle_input)
    {
        char[,] contraption = GetContraptionLayout(puzzle_input);

        var beam = new Beam
        {
            Row = 0,
            Col = 0,
            Direction = 'E',
            IsTravelling = true,
        };

        int[,] energized = TravelThroughContraption(contraption, beam);

        int res = 0;

        for (int row = 0; row < energized.GetLength(0); row++)
        {
            for (int col = 0; col < energized.GetLength(1); col++)
            {
                // any number above 0 means it is energized, count it as +1
                if (energized[row, col] > 0) res += 1;
            }
        }

        return res.ToString();
    }

    private static char[,] GetContraptionLayout(string puzzle_input)
    {
        string[] s_grid = puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        char[,] grid = new char[s_grid.Length, s_grid[0].Length];

        for (int row = 0; row < s_grid.Length; row++)
        {
            for (int col = 0; col < s_grid[row].Length; col++)
            {
                grid[row, col] = s_grid[row][col];
            }
        }

        return grid;
    }

    private static int[,] TravelThroughContraption(char[,] contraption, Beam beam)
    {
        int row_count = contraption.GetLength(0);
        int col_count = contraption.GetLength(1);

        int[,] energized = new int[row_count, col_count];

        void NewBeam(Beam beam)
        {
            while (beam.IsTravelling)
            {
                char encountered = contraption[beam.Row, beam.Col];

                if (encountered == '.')
                {
                    energized[beam.Row, beam.Col] = 1;
                }
                else if (encountered == '\\' || encountered == '/')
                {
                    beam.ReflectDirection(encountered);
                    energized[beam.Row, beam.Col] = 1;
                }
                else if (encountered == '|')
                {
                    if (beam.Direction == 'N' || beam.Direction == 'S')
                    {
                        energized[beam.Row, beam.Col] = 1;
                    }
                    else if (beam.Direction == 'E' || beam.Direction == 'W')
                    {
                        if (energized[beam.Row, beam.Col] < 2)
                        {
                            // fire up a new beam
                            NewBeam(beam.Split());

                            // we set energized to 2 to indicate that it has had a beam split (no need to repeat)
                            energized[beam.Row, beam.Col] = 2;
                        }
                        else beam.IsTravelling = false;
                    }
                }
                else if (encountered == '-')
                {
                    if (beam.Direction == 'E' || beam.Direction == 'W')
                    {
                        energized[beam.Row, beam.Col] = 1;
                    }
                    else if (beam.Direction == 'N' || beam.Direction == 'S')
                    {
                        if (energized[beam.Row, beam.Col] < 2)
                        {
                            // fire up a new beam
                            NewBeam(beam.Split());

                            // we set energized to 2 to indicate that it has had a beam split (no need to repeat)
                            energized[beam.Row, beam.Col] = 2;
                        }
                        else beam.IsTravelling = false;
                    }
                }

                beam.Travel();

                // check if beam is within the boundary of the contraption
                if (beam.Row >= row_count || beam.Row < 0) beam.IsTravelling = false;
                if (beam.Col >= col_count || beam.Col < 0) beam.IsTravelling = false;
            }
        }

        NewBeam(beam);

        return energized;
    }

    private class Beam
    {
        public int Row;
        public int Col;
        public char Direction;
        public bool IsTravelling;

        public void Travel()
        {
            Row += Direction == 'N' ? -1 : Direction == 'S' ? 1 : 0;
            Col += Direction == 'W' ? -1 : Direction == 'E' ? 1 : 0;
        }

        public void ReflectDirection(char c)
        {
            Direction = c switch
            {
                '/' =>
                    Direction == 'W' ? 'S' :
                    Direction == 'S' ? 'W' :
                    Direction == 'E' ? 'N' : 'E',

                '\\' =>
                    Direction == 'W' ? 'N' :
                    Direction == 'S' ? 'E' :
                    Direction == 'E' ? 'S' : 'W',

                _ => Direction,
            };
        }

        public Beam Split()
        {
            // creates a copy which has the opposite direction
            Beam new_beam = new()
            {
                Row = Row,
                Col = Col,
                Direction = Direction,
                IsTravelling = IsTravelling,
            };

            // split direction (set for both this beam and the one being returned)
            switch (Direction)
            {
                case 'N':
                case 'S':
                    this.Direction = 'E';
                    new_beam.Direction = 'W';
                    return new_beam;

                default:
                    this.Direction = 'N';
                    new_beam.Direction = 'S';
                    return new_beam;
            }
        }
    }
}
