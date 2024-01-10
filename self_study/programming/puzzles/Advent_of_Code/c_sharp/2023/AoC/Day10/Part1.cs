namespace AoC.Day10;

class Part1
{
    public static string Run(string puzzle_input)
    {
        char[][] pipe_maze = GetPipeMaze(puzzle_input);

        (int row, int col) = GetStartPosition(pipe_maze);

        char start_direction = GetStartDirection(row, col, pipe_maze);

        DirectionParser dp = new(row, col, start_direction);

        int steps = RecursePipeLoop(dp, pipe_maze);
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

    // this one needs improvement, it is very long..
    private static char GetStartDirection(int start_row, int start_col, char[][] pipe_maze)
    {
        // try the next pipe to the north, east, south and west and se if it has opening towards our start position
        char north = start_row == 0 ? '-' : pipe_maze[start_row-1][start_col];
        switch (north)
        {
            case '|':
            case '7':
            case 'F':
                return 'N';
        }
        char east = start_col == pipe_maze[start_row].Length - 1 ? '|' : pipe_maze[start_row][start_col+1];
        switch (east)
        {
            case '-':
            case 'J':
            case '7':
                return 'E';
        }
        char south = start_row == pipe_maze.Length - 1 ? '-' : pipe_maze[start_row-1][start_col];
        switch (south)
        {
            case '|':
            case 'J':
            case 'L':
                return 'S';
        }
        char west = start_col == 0 ? '|' : pipe_maze[start_row][start_col-1];
        switch (west)
        {
            case '-':
            case 'L':
            case 'F':
                return 'W';
        }

        return 'X'; // direction error
    }

    // stack overflows on my computer @ appr. 174450 calls, maze is 140*140 (19600 max possible), should be fine for recursion
    private static int RecursePipeLoop(DirectionParser dp, char[][] pipe_maze)
    {
        char pipe = dp.NextPipe(pipe_maze);

         // base case when we arrive back home
        if (pipe == 'S') return 1;
        else return 1 + RecursePipeLoop(dp, pipe_maze);
    }
}

internal class DirectionParser(int row, int col, char direction)
{
    // north, east, south, west are stored as 'N', 'E', 'S', 'W' ..respectively
    private char[] _valid_directions = ['N', 'E', 'S', 'W'];
    private char _direction = direction;
    private int _row = row;
    private int _col = col;

    public char NextPipe(char[][] pipe_maze)
    {
        if (!_valid_directions.Contains(_direction))
        {
            throw new InvalidDataException();
        }

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
            _ => 'X', // oops, did we crash into something?
        };

        return next_pipe;
    }
}
