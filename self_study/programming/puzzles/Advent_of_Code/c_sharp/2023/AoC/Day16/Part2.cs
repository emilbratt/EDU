namespace AoC.Day16;

class Part2
{
    public static string Run(string puzzle_input)
    {
        char[,] contraption = GetContraptionLayout(puzzle_input);

        int res = TryAllEntries(contraption);

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

    private static int TryAllEntries(char[,] contraption)
    {
        int highest_res = 0;

        int row_count = contraption.GetLength(0);
        int col_count = contraption.GetLength(1);
        Beam beam = new();

        // start from all rows left and right
        for (int row = 0; row < row_count; row++)
        {
            beam.Col = -1; // first move is into the grid from left
            beam.Row = row;
            beam.Direction = 'E';
            beam.EnergizedTiles = 0;
            beam = TravelThroughContraption(contraption, beam);
            if (beam.EnergizedTiles > highest_res) highest_res = beam.EnergizedTiles;

            beam.Col = col_count;  // first move is into the grid from right
            beam.Row = row;
            beam.Direction = 'W';
            beam.EnergizedTiles = 0;
            beam = TravelThroughContraption(contraption, beam);
            if (beam.EnergizedTiles > highest_res) highest_res = beam.EnergizedTiles;
        }

        // start from all collumns top and bottom
        for (int col = 0; col < col_count; col++)
        {
            beam.Col = col;
            beam.Row = -1;  // first move is into the grid from top
            beam.Direction = 'S';
            beam.EnergizedTiles = 0;
            beam = TravelThroughContraption(contraption, beam);
            if (beam.EnergizedTiles > highest_res) highest_res = beam.EnergizedTiles;

            beam.Col = col;
            beam.Row = row_count;  // first move is into the grid from bottom
            beam.Direction = 'N';
            beam.EnergizedTiles = 0;
            beam = TravelThroughContraption(contraption, beam);
            if (beam.EnergizedTiles > highest_res) highest_res = beam.EnergizedTiles;
        }

        return highest_res;
    }

    private static Beam TravelThroughContraption(char[,] contraption, Beam beam)
    {
        int row_count = contraption.GetLength(0);
        int col_count = contraption.GetLength(1);

        // this array mirrors the contraption, but as integers instead of characters
        int[,] energized = new int[row_count, col_count]; // ..starts out with only zero's

        Beam NewBeam(Beam beam)
        {
            while (true)
            {
                // move the beam one step north, west, south or east based on beam.Direction
                beam.Travel();

                // before doing anything, check if beam is within the boundary of the contraption
                if (beam.Row >= row_count || beam.Row < 0) return beam;
                if (beam.Col >= col_count || beam.Col < 0) return beam;

                // a beam split have been recorded here before, prevent repeating same travel
                if (energized[beam.Row, beam.Col] == 2) return beam;

                // not visited field, lets count it
                if (energized[beam.Row, beam.Col] == 0) beam.EnergizedTiles += 1;

                char encountered = contraption[beam.Row, beam.Col];

                // depending on the encountered value, the beam will turn 90 degrees
                beam.ReflectDirection(encountered);

                // if the beam is split, some more logic needs to take place
                bool beam_split = encountered switch
                {
                    '|' => beam.Direction == 'E' || beam.Direction == 'W',
                    '-' => beam.Direction == 'N' || beam.Direction == 'S',
                    _ => false,
                };

                // we set energized to 2 to indicate that it has had a beam split (no need to repeat)
                energized[beam.Row, beam.Col] = beam_split ? 2 : 1;

                if (beam_split)
                {
                    // get two new directions
                    (char a, char b) = beam.SplitDirection();

                    // store current row and col for use after the split
                    int old_row = beam.Row;
                    int old_col = beam.Col;

                    // send the beam with one direction and continue with it after it is done
                    beam.Direction = a;
                    beam = NewBeam(beam);

                    // continue with the beam here resetting back to old values
                    beam.Direction = b;
                    beam.Row = old_row;
                    beam.Col = old_col;
                }
            }
        }

        return NewBeam(beam);
    }

    private class Beam
    {
        public int Row;
        public int Col;
        public char Direction;
        public int EnergizedTiles = 0;

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

        public (char a, char b) SplitDirection()
        {
            // keep one direction and return the opposite direction
            switch (Direction)
            {
                case 'N':
                case 'S':
                    return ('E', 'W');

                default:
                    return ('N', 'S');
            }
        }
    }
}
