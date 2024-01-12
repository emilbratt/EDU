namespace AoC.Day10;

class Part1
{
    public static string Run(string puzzle_input)
    {
        char[][] pipe_maze = GetPipeMaze(puzzle_input);

        (int row, int col) = GetStartPosition(pipe_maze);

        char[] start_directions = PossibleDirections(row, col, pipe_maze);

        DirectionParser dp = new(row, col, start_directions[0]);

        int steps = RecurseTraversePipeMaze(dp, pipe_maze);
        steps /= 2; // dividing by 2 gives n steps farthest from the starting position

        return steps.ToString();
    }

    private static char[][] GetPipeMaze(string puzzle_input)
    {
        string[] lines = puzzle_input.Split("\n", StringSplitOptions.RemoveEmptyEntries);

        char[][] pipe_maze = new char[lines.Length][];

        int row = 0;
        while (row < lines.Length)
        {
            pipe_maze[row] = lines[row].ToCharArray();
            row++;
        }

        return pipe_maze;
    }

    private static (int start_row, int start_col) GetStartPosition(char[][] pipe_maze)
    {
        int row = 0;
        while (row < pipe_maze.Length)
        {
            int col = 0;
            while (col < pipe_maze[row].Length)
            {
                if (pipe_maze[row][col] == 'S') return (row, col);
                col++;
            }
            row++;
        }

        return (-1, -1);
    }

    private static char[] PossibleDirections(int row, int col, char[][] pipe_maze)
    {
        char[] pd = new char[2];
        Dictionary<(int row, int col, char direction), char[]> dir_map = new()
        {
            { (row - 1, col,     'N'), ['F', '7', '|'] }, // North entering
            { (row,     col + 1, 'E'), ['J', '7', '-'] }, // East entering
            { (row,     col - 1, 'W'), ['F', 'L', '-'] }, // West entering
            { (row + 1, col,     'S'), ['J', 'L', '|'] }, // South entering
        };

        int index = 0;
        int row_count = pipe_maze.Length;
        foreach (var kv in dir_map)
        {
            if (kv.Key.row > 0 && kv.Key.row < row_count)
            {
                int col_count = pipe_maze[kv.Key.row].Length;
                if(kv.Key.col > 0 && kv.Key.col < col_count)
                {
                    {
                        if (kv.Value.Contains(pipe_maze[kv.Key.row][kv.Key.col]))
                        {
                            pd[index] = kv.Key.direction;
                            index++;
                        }
                    }
                }
            }
        }

        return pd;
    }

    // stack overflows on my computer @ appr. 174450 calls, maze is 140*140 (19600 max possible), should be fine for recursion
    private static int RecurseTraversePipeMaze(DirectionParser dp, char[][] pipe_maze)
    {
        char pipe = dp.NextPipe(pipe_maze);

         // base case when we arrive back home
        if (pipe == 'S') return 1;
        else return 1 + RecurseTraversePipeMaze(dp, pipe_maze);
    }

    private class DirectionParser(int row, int col, char direction)
    {
        private char[] _valid_directions = ['N', 'E', 'S', 'W']; // north, east, south, west ..respectively
        private char _direction = direction;
        private int _row = row;
        private int _col = col;

        public char NextPipe(char[][] pipe_maze)
        {
            if (!_valid_directions.Contains(_direction)) throw new InvalidDataException();

            _row += _direction switch
            {
                'N' => -1,
                'S' => 1,
                _ => 0,
            };

            _col += _direction switch
            {
                'E' => 1,
                'W' => -1,
                _ => 0,
            };

            char next_pipe = pipe_maze[_row][_col];

            // straight pipes (or final destination 'S') will not affect travel direction
            switch (next_pipe)
            {
                case 'S':
                case '-':
                case '|':
                    return next_pipe;
            }
            // ..however,  non-straight pipes will change our direction..
            _direction = next_pipe switch
            {
                // the false side of the ternary should always yield correct value based on the condition
                '7' => _direction == 'N' ? 'W' : 'S',
                'F' => _direction == 'W' ? 'S' : 'E',
                'L' => _direction == 'W' ? 'N' : 'E',
                'J' => _direction == 'E' ? 'N' : 'W',
                _ => throw new InvalidDataException(), // oops, did we crash into something?
            };

            return next_pipe;
        }
    }
}
