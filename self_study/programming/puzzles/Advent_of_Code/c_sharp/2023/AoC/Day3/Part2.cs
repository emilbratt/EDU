namespace AoC.Day3;

class Part2
{
    private static string[]? _puzzle_input;
    private static int _last_row = 0;
    private static int _last_col = 0;
    private static Dictionary<(int x, int y),(int count, int product)>? _gear_map;

    public static string Run(string[] puzzle_input)
    {
        _puzzle_input = puzzle_input;
        _gear_map = new Dictionary<(int x, int y),(int count, int product)>();

        // we treat the whole input as a matrix - each line is a row and each character is a column
        _last_row = _puzzle_input.Length - 1;
        _last_col = _puzzle_input[0].Length - 1; // each line has the exact same char count, we count first line

        return HandlePuzzleInput();
    }

    private static string HandlePuzzleInput()
    {
        if (_puzzle_input == null) Environment.Exit(1);

        for (int row = 0; row <= _last_row; row++)
        {
            int col = 0;
            while (col <= _last_col)
            {
                int total_digits = 0;
                int start_col = col;

                // this triggers on the first char that is a digit and ends on the last
                while (char.IsNumber(_puzzle_input[row][col]))
                {
                    total_digits++;
                    if (col == _last_col) break; // do not go to next col if this is the last col
                    col++;
                }

                // having any digits means we have a number, lets see if it is a valid part number before adding it
                if (total_digits > 0)
                {
                    string string_number = _puzzle_input[row].Substring(start_col, total_digits);
                    int number =  int.Parse(string_number);
                    (int, int) gear_coordinates = GetAdjacentGear(row, start_col, start_col + total_digits);

                    if (gear_coordinates != (-1, -1)) AddToGearMap(gear_coordinates, number);
                } 

                col++;
            }
        }

        #pragma warning disable CS8602 // Dereference of a possibly null reference.
        int total = 0;
        foreach ((int, int) value in _gear_map.Values)
        {
            int count = value.Item1;
            int product = value.Item2;
            if (count == 2) total += product;
        }

        return total.ToString();
    }

    private static (int, int) GetAdjacentGear(int current_row, int from_col, int to_col)
    {
        if (_puzzle_input == null) Environment.Exit(1);

        for (int row = current_row - 1; row <= current_row + 1; row++)
        {
            if (row < 0 || row > _last_row) continue;

            for (int col = from_col - 1; col < to_col + 1; col++)
            {
                if (col < 0 || col > _last_col) continue;

                if (_puzzle_input[row][col] == '*') return (row, col);
            }
        }

        return (-1, -1);
    }

    private static void AddToGearMap((int, int) gear_coordinates, int number)
    {
        if (!_gear_map.ContainsKey(gear_coordinates))
        {
            _gear_map.Add(gear_coordinates, (1, number));
        }
        else
        {
            int count = _gear_map[gear_coordinates].Item1;
            int product = _gear_map[gear_coordinates].Item2;
            _gear_map[gear_coordinates] = (count + 1, product * number);
        }
    }
}
