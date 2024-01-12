namespace AoC.Day10;

class Part2
{
    public static string Run(string puzzle_input)
    {
        char[][] marked_pipe_maze = GetPipeMaze(puzzle_input); // used to mark the all pipes part of the loop with an 'x'
        char[][] unmarked_pipe_maze = GetPipeMaze(puzzle_input); // for cross-referenceing a marked 'x' with the original pipe

        (int row, int col) = StartPosition(marked_pipe_maze);

        (char substitue, char direction) = StartPipeAndDirection(row, col, unmarked_pipe_maze);

        unmarked_pipe_maze[row][col] = substitue;

        DirectionParser dp = new(row, col, direction);

        // Step 1. Marking all pipes that are part of the loop
        marked_pipe_maze = RecurseMarkPipeMaze(dp, marked_pipe_maze);

        // Step 2. Cross-referecing the marked pipes with its original symbol (unmarked) and count all enclosed tiles
        int result = CountEnclosedTiles(marked_pipe_maze, unmarked_pipe_maze);

        return result.ToString();
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

    private static (int row, int col) StartPosition(char[][] pipe_maze)
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

    private static (char substitue, char direction) StartPipeAndDirection(int row, int col, char[][] pipe_maze)
    {
        Dictionary<(int row, int col, char direction), char[]> dir_map = new()
        {
            { (row - 1, col,     'N'), ['F', '7', '|'] }, // North entering
            { (row,     col + 1, 'E'), ['J', '7', '-'] }, // East entering
            { (row + 1, col,     'S'), ['J', 'L', '|'] }, // South entering
            { (row,     col - 1, 'W'), ['F', 'L', '-'] }, // West entering
        };

        char[] possible_directions = ['.', '.'];

        int index = 0;
        int row_count = pipe_maze.Length;
        foreach (var kv in dir_map)
        {
            if (kv.Key.row > 0 && kv.Key.row < row_count)
            {
                if(kv.Key.col > 0 && kv.Key.col < pipe_maze[kv.Key.row].Length)
                {
                    if (kv.Value.Contains(pipe_maze[kv.Key.row][kv.Key.col]))
                    {
                        possible_directions[index] = kv.Key.direction;
                        index++;
                    }
                }
            }
        }

        char substitue = possible_directions switch
        {
            ['N', 'S'] => '|',
            ['S', 'N'] => '|',

            ['E', 'W'] => '-',
            ['W', 'E'] => '-',

            ['N', 'E'] => 'L',
            ['E', 'N'] => 'L',

            ['N', 'W'] => 'J',
            ['W', 'N'] => 'J',

            ['S', 'W'] => '7',
            ['W', 'S'] => '7',

            ['S', 'E'] => 'F',
            ['E', 'S'] => 'F',

            ['N', '.'] => '.',
            ['E', '.'] => '.',
            ['S', '.'] => '.',
            ['W', '.'] => '.',

            _ => throw new InvalidDataException(),
        };


        char direction = possible_directions[0];
        return (substitue, direction);
    }

    // stack overflows on my computer @ appr. 174450 calls, maze is 140*140 (19600 max possible), should be fine for recursion
    private static char[][] RecurseMarkPipeMaze(DirectionParser dp, char[][] pipe_maze)
    {
        char pipe = dp.NextPipe(pipe_maze);

        pipe_maze[dp.Coordinates.row][dp.Coordinates.col] = 'x'; // mark position as traversed

         // base case when we arrive back home
        if (pipe == 'S') return pipe_maze;

        return RecurseMarkPipeMaze(dp, pipe_maze);
    }

    private static int CountEnclosedTiles(char[][] marked_pipe_maze, char[][] unmarked_pipe_maze)
    {
        int result = 0;

        int row = 0;
        while (row < marked_pipe_maze.Length)
        {
            bool is_enclosed = false;
            char last_crossed = '|';

            int col = 0;
            while (col < marked_pipe_maze[row].Length)
            {
                char symbol = unmarked_pipe_maze[row][col];
                bool marked = marked_pipe_maze[row][col] == 'x';

                result += !marked && is_enclosed ? 1 : 0;

                if (marked && symbol != '-') // if crossing a pipe
                {
                    // NOTE: '-' is the only pipe symbol that is note a "crossing point" in our case

                    is_enclosed = symbol switch
                    {
                        // always switch when crossing a '|'
                        '|' => !is_enclosed,

                        // crossing 'J', only switch if last crossing was through 'F'
                        'J' => last_crossed == 'F' ? !is_enclosed : is_enclosed,

                        // crossing '7', only switch if last crossing was through 'L'
                        '7' => last_crossed == 'L' ? !is_enclosed : is_enclosed,

                        // do nothing
                        _ => is_enclosed,
                    };

                    last_crossed = symbol;
                }

                col++;
            }

            row++;
        }

        return result;
    }

    private class DirectionParser(int row, int col, char direction)
    {
        private char[] _valid_directions = ['N', 'E', 'S', 'W']; // north, east, south, west ..respectively
        private char _direction = direction;
        private int _row = row;
        private int _col = col;

        public char Direction => _direction;

        public (int row, int col) Coordinates => (_row, _col);

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
